use parser::{ParseError, SyntaxKind, Token, TreeSink};
use rowan::{GreenNode, TextRange, TextSize};

use crate::{syntax_node::SyntaxTreeBuilder, SyntaxError, T};

/// A tree sink that works on normal text. This is the main one that we will use.
pub struct TextTreeSink<'a> {
    text: &'a str,
    tokens: &'a [Token],
    text_pos: TextSize,
    token_pos: usize,
    inner: SyntaxTreeBuilder,
    error_ranges: Vec<TextRange>,
}

impl<'a> TreeSink for TextTreeSink<'a> {
    type FinishResult = (GreenNode, Vec<SyntaxError>);

    fn token(&mut self) {
        self.eat_trivias();
        self.do_token();
    }

    fn start_node(&mut self, kind: SyntaxKind) {
        self.eat_trivias();
        self.inner.start_node(kind);
        self.eat_trivias();
    }

    fn finish_node(&mut self) {
        self.inner.finish_node();
    }

    fn start_error_node(&mut self) {
        self.start_node(T![error]);
        self.error_ranges.push(TextRange::empty(self.text_pos));
    }

    fn finish_error_node(&mut self, e: ParseError) {
        self.finish_node();
        self.inner.error(e, self.error_ranges.pop().unwrap())
    }

    fn error(&mut self, error: ParseError) {
        self.inner.error(error, TextRange::empty(self.text_pos))
    }

    fn finish(self) -> (GreenNode, Vec<SyntaxError>) {
        self.inner.finish_raw()
    }
}

impl<'a> TextTreeSink<'a> {
    pub(super) fn new(text: &'a str, tokens: &'a [Token]) -> Self {
        Self {
            text,
            tokens,
            text_pos: 0.into(),
            token_pos: 0,
            inner: SyntaxTreeBuilder::default(),
            error_ranges: Vec::new(),
        }
    }

    fn current_token(&self) -> Token {
        self.tokens[self.token_pos]
    }

    fn eat_trivias(&mut self) {
        while let Some(&token) = self.tokens.get(self.token_pos) {
            if !token.kind.is_trivia() {
                break;
            }
            self.do_token();
        }
    }

    fn do_token(&mut self) {
        let token = self.current_token();
        let range = TextRange::at(self.text_pos, token.len);
        let text = &self.text[range];
        self.token_pos += 1;
        self.text_pos += token.len;
        self.inner.token(token.kind, text);
        self.update_error_ranges(range);
    }

    fn update_error_ranges(&mut self, range: TextRange) {
        for error_node in self.error_ranges.iter_mut() {
            *error_node = error_node.cover(range)
        }
    }
}

/// A tree sink that will start the first `defer` amount of root nodes without consuming
/// whitespace. It will then only end nodes when it needs to such as when `token` or another
/// `start_node` is called. This means that when the processing is done, we have `defer` amount of
/// nodes waiting to be ended. This way we can `eat_whitespace` before we end the nodes.
/// The effect is that when there is whitespace found before a node, it will be parsed before it
/// and node inside of the node. It will also parse whitespace before a token. The exception is
/// trailing whitespace which is handled by defering. We cannot eat whitespace before ending a node
/// because it will take whitespace from the start of the next node. We cannot eat whitespace after
/// ending a node because if multiple nodes are ended, we want the whitespace to be at the start of
/// the next node after all current nodes have been ended.
///
/// An example is parsing a `SourceFile`. In lua a `SourceFile` is a block so we want to parse it
/// like this:
/// ```
/// SourceFile
///   Block
///     Stmt1
///     Stmt2
///     ...
/// ```
/// This means that we would set `defer` to 2 because there are two root nodes.
pub struct DeferedTextTreeSink<'a> {
    sink: TextTreeSink<'a>,
    defer: u8,
    started: u8,
    finished: u8,
}

impl<'a> TreeSink for DeferedTextTreeSink<'a> {
    type FinishResult = (GreenNode, Vec<SyntaxError>);

    fn token(&mut self) {
        assert!(self.started == self.defer);

        self.consume_finish_defers();

        self.sink.token();
    }

    fn start_node(&mut self, kind: SyntaxKind) {
        self.consume_finish_defers();

        if self.started < self.defer {
            self.sink.inner.start_node(kind);
            self.started += 1;
            return;
        }

        self.sink.start_node(kind);
    }

    fn finish_node(&mut self) {
        if self.finished < self.defer {
            self.finished += 1;
            return;
        }

        self.sink.finish_node();
    }

    fn error(&mut self, error: ParseError) {
        assert!(self.started == self.defer, "There should be no errors when starting");

        self.sink.error(error)
    }

    fn start_error_node(&mut self) {
        assert!(self.started == self.defer, "There should be no errors when starting");

        self.consume_finish_defers();

        self.sink.start_error_node();
    }

    fn finish_error_node(&mut self, e: ParseError) {
        // we don't have to do a check here because to finish we have to start
        // and start_error_node will do the checks
        self.sink.finish_error_node(e)
    }

    fn finish(mut self) -> Self::FinishResult {
        self.sink.eat_trivias();
        self.consume_finish_defers();
        self.sink.finish()
    }
}

impl<'a> DeferedTextTreeSink<'a> {
    pub fn new(text: &'a str, tokens: &'a [Token], defer: u8) -> Self {
        let sink = TextTreeSink::new(text, tokens);
        Self { sink, defer, started: 0, finished: 0 }
    }

    fn consume_finish_defers(&mut self) {
        for _ in 0..self.finished {
            self.sink.finish_node();
        }
        self.finished = 0
    }
}

use parser::{ParseError, SyntaxKind, Token, TreeSink};
use rowan::{TextRange, TextSize};

use crate::syntax_node::SyntaxTreeBuilder;

pub struct TextTreeSink<'a> {
    text: &'a str,
    tokens: &'a [Token],
    text_pos: TextSize,
    token_pos: usize,
    inner: SyntaxTreeBuilder,
    error_ranges: Vec<TextRange>,
}

impl<'a> TreeSink for TextTreeSink<'a> {
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
        self.error_ranges.push(TextRange::empty(self.text_pos));
    }

    fn finish_error_node(&mut self, e: ParseError) {
        self.inner.error(e, self.error_ranges.pop().unwrap())
    }

    fn error(&mut self, error: ParseError) {
        self.inner.error(error, TextRange::empty(self.text_pos))
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
        let text = &self.text[token.range];
        self.token_pos += 1;
        self.text_pos += token.range.len();
        self.inner.token(token.kind, text);
        self.update_error_ranges(token.range);
    }

    fn update_error_ranges(&mut self, range: TextRange) {
        for error_node in self.error_ranges.iter_mut() {
            *error_node = error_node.cover(range)
        }
    }
}

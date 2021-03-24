use std::marker::PhantomData;

use parser::{ParseErrorKind, SyntaxKind, Token, TreeSink};
use rowan::{TextRange, TextSize};

use crate::syntax_node::SyntaxTreeBuilder;

pub struct TextTreeSink<'a> {
    text: &'a str,
    tokens: &'a [Token],
    text_pos: TextSize,
    token_pos: usize,
    inner: SyntaxTreeBuilder,
}

impl<'a> TreeSink for TextTreeSink<'a> {
    fn token(&mut self, token: Token) {
        self.eat_trivias();
        self.do_token(token);
    }

    fn error(&mut self, error: ParseErrorKind) {
        todo!();
        // self.inner.error(error, self.text_pos);
    }

    fn start_node(&mut self, kind: SyntaxKind) {
        self.eat_trivias();
        self.inner.start_node(kind);
        self.eat_trivias();
    }

    fn finish_node(&mut self) {
        self.inner.finish_node();
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
        }
    }

    fn eat_trivias(&mut self) {
        while let Some(&token) = self.tokens.get(self.token_pos) {
            if !token.kind.is_trivia() {
                break;
            }
            self.do_token(token);
        }
    }

    fn do_token(&mut self, token: Token) {
        let text = &self.text[token.range];
        self.token_pos += 1;
        self.text_pos += token.range.len();
        self.inner.token(token.kind, text);
    }
}

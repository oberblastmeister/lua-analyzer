//! See `TextTokenSource` docs.

use parser::{Token, TokenSource, T};

use rowan::TextRange;

/// Implementation of `parser::TokenSource` that takes tokens from source code text.
pub(crate) struct TextTokenSource<'t> {
    text: &'t str,

    // token_offset_pairs: Vec<(Token, TextSize)>,
    tokens: &'t [Token],

    // /// Current token and position
    // curr: (parser::Token, usize),
    curr: (Token, usize),
}

impl<'t> TextTokenSource<'t> {
    pub(crate)  fn new(text: &'t str, tokens: &'t [Token]) -> TextTokenSource<'t> {
        let first = mk_token(0, &tokens, TextRange::new(0.into(), 0.into()));
        TextTokenSource {
            text,
            tokens,
            curr: (first, 0),
        }
    }

    fn curr_range(&self) -> TextRange {
        self.curr.0.range
    }
}

impl<'t> TokenSource for TextTokenSource<'t> {
    fn current(&self) -> parser::Token {
        self.curr.0
    }

    fn lookahead_nth(&self, n: usize) -> parser::Token {
        mk_token(n, &self.tokens, self.curr_range())
    }

    fn bump(&mut self) {
        if self.curr.0.kind == T![eof] {
            return;
        }

        let pos = self.curr.1 + 1;
        self.curr = (mk_token(pos, &self.tokens, self.curr_range()), pos);
    }

    fn is_keyword(&self, kw: &str) -> bool {
        self.tokens
            .get(self.curr.1)
            .map(|token| &self.text[token.range] == kw)
            .unwrap_or(false)
    }
}

fn mk_token(pos: usize, tokens: &[Token], eof_range: TextRange) -> Token {
    match tokens.get(pos) {
        Some(token) => *token,
        None => Token {
            kind: T![eof],
            range: eof_range,
        },
    }
}

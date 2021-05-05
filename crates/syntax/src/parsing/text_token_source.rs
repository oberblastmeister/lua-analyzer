//! See `TextTokenSource` docs.

use parser::{Token, TokenSource, T};

use rowan::{TextRange, TextSize};

/// Implementation of `parser::TokenSource` that takes tokens from source code text.
pub(crate) struct TextTokenSource<'t> {
    text: &'t str,

    token_offset_pairs: Vec<(Token, TextSize)>,
    // tokens: Vec<Token>,

    // /// Current token and position
    // curr: (parser::Token, usize),
    curr: (Token, usize),
}

impl<'t> TextTokenSource<'t> {
    pub(crate) fn new(text: &'t str, raw_tokens: &'t [Token]) -> TextTokenSource<'t> {
        let token_offset_pairs: Vec<_> = raw_tokens
            .iter()
            .filter_map({
                let mut len = 0.into();
                move |token| {
                    let pair = if token.kind.is_trivia() { None } else { Some((*token, len)) };
                    len += token.len;
                    pair
                }
            })
            .collect();

        let first = mk_token(0, &token_offset_pairs, TextRange::new(0.into(), 0.into()));

        TextTokenSource { text, token_offset_pairs, curr: (first, 0) }
    }

    fn curr_range(&self) -> TextRange {
        if let Some((token, offset)) =
            self.token_offset_pairs.get(self.curr.1).or(self.token_offset_pairs.last())
        {
            TextRange::at(*offset, token.len)
        } else {
            TextRange::empty(0.into())
        }
    }

    #[inline]
    fn curr_text(&self) -> &str {
        &self.text[self.curr_range()]
    }
}

impl<'t> TokenSource for TextTokenSource<'t> {
    fn current(&self) -> parser::Token {
        self.curr.0
    }

    fn lookahead_nth(&self, n: usize) -> parser::Token {
        mk_token(self.curr.1 + n, &self.token_offset_pairs, self.curr_range())
    }

    fn bump(&mut self) {
        if self.curr.0.kind == T![eof] {
            return;
        }

        let pos = self.curr.1 + 1;
        self.curr = (mk_token(pos, &self.token_offset_pairs, self.curr_range()), self.curr.1 + 1);
    }

    fn is_keyword(&self, kw: &str) -> bool {
        self.curr_text() == kw
    }
}

fn mk_token(pos: usize, tokens: &[(Token, TextSize)], eof_range: TextRange) -> Token {
    match tokens.get(pos) {
        Some((token, _offset)) => *token,
        None => Token { kind: T![eof], len: 0.into() },
    }
}

pub mod error;

use std::iter;

use parser::{SyntaxKind, Token};
use rowan::{TextRange, TextSize};

use crate::SyntaxError;

use self::error::{LexResult, SyntaxResult};

pub trait Lexer<'a> {
    fn new(text: &'a str) -> Self;

    fn next_syntax_kind(&mut self) -> LexResult<SyntaxKind>;

    fn next_token(mut self) -> LexResult<Token>
    where
        Self: Sized,
    {
        let res = self.next_syntax_kind();
        let len_consumed = self.len_consumed();
        res.map(|kind| Token { kind, len: len_consumed.into() })
    }

    fn len_consumed(&self) -> TextSize;
}

pub fn first_token<'a, L>(text: &'a str) -> Option<SyntaxResult<Token>>
where
    L: Lexer<'a>,
{
    if text.is_empty() {
        return None;
    }

    let res = L::new(text).next_token();
    let range = TextRange::up_to(res.ok_ref().len);
    Some(res.map_err(|e| SyntaxError::new(e.to_string(), range)))
}

pub fn first_syntax_kind<'a, L>(text: &'a str) -> Option<SyntaxResult<SyntaxKind>>
where
    L: Lexer<'a>,
{
    Some(first_token::<L>(text)?.map(|token| token.kind))
}

/// Tokenize text into a vector of tokens and errors
pub fn tokenize<'a, L>(text: &'a str) -> (Vec<Token>, Vec<SyntaxError>)
where
    L: Lexer<'a>,
{
    let mut errors = Vec::new();
    let tokens = tokenizer::<L>(text)
        .map(|res| {
            let (token, err) = res.inner();
            errors.extend(err);
            token
        })
        .collect();

    (tokens, errors)
}

/// Returns a tokenizer iterator that will turn the source text into tokens
pub fn tokenizer<'a, L>(mut text: &'a str) -> impl Iterator<Item = SyntaxResult<Token>> + '_
where
    L: Lexer<'a>,
{
    let mut pos = TextSize::from(0);

    iter::from_fn(move || {
        let res = first_token::<L>(text)?.map_err(|err| {
            let range = err.range();
            err.with_range(range + pos)
        });

        let len = res.ok_ref().len;

        pos += len;
        text = &text[len.into()..];

        Some(res)
    })
}

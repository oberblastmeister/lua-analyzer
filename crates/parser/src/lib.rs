mod lexer;
mod parser;
#[macro_use]
mod syntax_kind;
mod grammar;
mod event;
mod token_set;
mod parse_error;

use rowan::TextRange;
pub use syntax_kind::SyntaxKind;
pub use lexer::{tokenize_iter, first_token, Token, LexError, LexErrorMsg};
pub use event::Event;
pub(crate) use token_set::TokenSet;
pub use parse_error::{ParseErrorKind, ParseError};

pub trait TokenSource {
    fn current(&self) -> Token;

    /// Lookahead n token
    fn lookahead_nth(&self, n: usize) -> Token;

    /// bump cursor to next token
    fn bump(&mut self);

    /// Is the current token a specified keyword?
    fn is_keyword(&self, kw: &str) -> bool;
}

/// `TreeSink` abstracts details of a particular syntax tree implementation.
pub trait TreeSink {
    /// Adds new token to the current branch.
    fn token(&mut self, kind: SyntaxKind, n_tokens: u8);

    /// Start new branch and make it current.
    fn start_node(&mut self, kind: SyntaxKind);

    /// Finish current branch and restore previous
    /// branch as current.
    fn finish_node(&mut self);

    fn error(&mut self, error: ParseErrorKind);
}

mod parser;
#[macro_use]
mod syntax_kind;
mod event;
mod grammar;
mod parse_error;
mod token_set;

pub use event::Event;
pub use grammar::LuaOp;
pub use parse_error::ParseError;
use rowan::TextRange;
pub use syntax_kind::SyntaxKind;
pub use token_set::TokenSet;

#[macro_export]
#[doc(hidden)]
macro_rules! assert_matches {
    ($expression:expr, $($stuff:tt)+) => {
        assert!(matches!($expression, $($stuff)+));
    };
}

/// A lua token
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Token {
    /// The kind of token.
    pub kind: SyntaxKind,
    /// The range of the token.
    pub range: TextRange,
}

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
    fn token(&mut self);

    /// Start new branch and make it current.
    fn start_node(&mut self, kind: SyntaxKind);

    /// Finish current branch and restore previous
    /// branch as current.
    fn finish_node(&mut self);

    fn start_error_node(&mut self);

    fn finish_error_node(&mut self, e: ParseError);

    fn error(&mut self, error: ParseError);
}

fn parse_from_tokens<F, TS>(token_source: &mut dyn TokenSource, tree_sink: &mut TS, f: F)
where
    F: FnOnce(&mut parser::Parser),
    TS: TreeSink,
{
    let mut p = parser::Parser::new(token_source);
    f(&mut p);
    let events = p.finish();
    event::process(tree_sink, events);
}

/// Parse given tokens into the given sink as a rust file.
pub fn parse<TS>(token_source: &mut dyn TokenSource, tree_sink: &mut TS)
where
    TS: TreeSink,
{
    parse_from_tokens(token_source, tree_sink, grammar::root);
}

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
pub use syntax_kind::SyntaxKind;
pub use token_set::TokenSet;

use rowan::TextSize;

use self::parser::Parser;

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
    /// The length of the token.
    pub len: TextSize,
}

impl Token {
    pub fn with_range(self, len: TextSize) -> Token {
        Token { kind: self.kind, len }
    }
}

impl Token {
    pub fn new(kind: SyntaxKind, len: TextSize) -> Token {
        Token { kind, len }
    }
}

/// `Token` abstracts the cursor of `TokenSource` operates on.
pub trait TokenSource {
    fn current(&self) -> Token;

    /// Lookahead n token
    fn lookahead_nth(&self, n: usize) -> Token;

    /// bump cursor to next token
    fn bump(&mut self) -> Option<Token>;

    /// Is the current token a specified keyword?
    fn is_keyword(&self, kw: &str) -> bool;
}

/// `TreeSink` abstracts details of a particular syntax tree implementation.
pub trait TreeSink {
    type FinishResult;

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

    fn finish(self) -> Self::FinishResult;
}

fn parse_from_tokens<F, TS>(
    token_source: &mut dyn TokenSource,
    tree_sink: TS,
    f: F,
) -> TS::FinishResult
where
    F: FnOnce(&mut parser::Parser),
    TS: TreeSink,
{
    let mut p = parser::Parser::new(token_source);
    f(&mut p);
    let events = p.finish();
    event::process(tree_sink, events)
}

/// Parse given tokens into the given sink as a rust file.
pub fn parse<TS>(token_source: &mut dyn TokenSource, tree_sink: TS) -> TS::FinishResult
where
    TS: TreeSink,
{
    parse_from_tokens(token_source, tree_sink, grammar::root)
}

/// A parsing function for a block.
/// Blocks can be from functions, do stmts, loops, etc.
/// Anything that has an end.
pub struct Reparser(fn(&mut Parser));

impl Reparser {
    pub fn for_node(node: SyntaxKind) -> Option<Reparser> {
        grammar::reparser(node).map(Reparser)
    }

    pub fn parse<TS>(self, token_source: &mut dyn TokenSource, tree_sink: TS) -> TS::FinishResult
    where
        TS: TreeSink,
    {
        let Reparser(r) = self;
        let mut p = Parser::new(token_source);
        r(&mut p);
        let events = p.finish();
        event::process(tree_sink, events)
    }
}

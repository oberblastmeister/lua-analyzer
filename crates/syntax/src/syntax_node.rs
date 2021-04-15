use std::fmt;

use parser::{LexError, LexErrorMsg, ParseError};
use rowan::{GreenNodeBuilder, Language, TextRange, TextSize};

use crate::{Parse, SyntaxKind};

pub(crate) use rowan::GreenNode;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RustLanguage {}

impl Language for RustLanguage {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> SyntaxKind {
        SyntaxKind::from(raw.0)
    }

    fn kind_to_raw(kind: SyntaxKind) -> rowan::SyntaxKind {
        rowan::SyntaxKind(kind.into())
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SyntaxError(String, TextRange);

impl SyntaxError {
    pub fn new(msg: String, range: TextRange) -> SyntaxError {
        SyntaxError(msg, range)
    }

    pub fn new_at_offset(message: impl Into<String>, offset: TextSize) -> SyntaxError {
        SyntaxError(message.into(), TextRange::empty(offset))
    }

    pub fn range(&self) -> TextRange {
        self.1
    }
}

impl From<LexError> for SyntaxError {
    fn from(e: LexError) -> SyntaxError {
        SyntaxError(e.msg.to_string(), e.range)
    }
}

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

pub type SyntaxNode = rowan::SyntaxNode<RustLanguage>;
pub type SyntaxToken = rowan::SyntaxToken<RustLanguage>;
pub type SyntaxElement = rowan::SyntaxElement<RustLanguage>;
pub type SyntaxNodeChildren = rowan::SyntaxNodeChildren<RustLanguage>;
pub type SyntaxElementChildren = rowan::SyntaxElementChildren<RustLanguage>;

#[derive(Default)]
pub struct SyntaxTreeBuilder {
    errors: Vec<SyntaxError>,
    inner: GreenNodeBuilder<'static>,
}

impl SyntaxTreeBuilder {
    pub(crate) fn finish_raw(self) -> (GreenNode, Vec<SyntaxError>) {
        let green = self.inner.finish();
        (green, self.errors)
    }

    pub fn finish(self) -> Parse<SyntaxNode> {
        let (green, errors) = self.finish_raw();
        Parse::new(green, errors)
    }

    pub fn token(&mut self, kind: SyntaxKind, text: &str) {
        let kind = RustLanguage::kind_to_raw(kind);
        self.inner.token(kind, text)
    }

    pub fn start_node(&mut self, kind: SyntaxKind) {
        let kind = RustLanguage::kind_to_raw(kind);
        self.inner.start_node(kind)
    }

    pub fn finish_node(&mut self) {
        self.inner.finish_node()
    }

    pub fn error(&mut self, error: ParseError, range: TextRange) {
        self.errors.push(SyntaxError(error.to_string(), range))
    }
}

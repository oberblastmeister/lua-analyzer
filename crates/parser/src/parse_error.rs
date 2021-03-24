use rowan::TextRange;
use thiserror::Error;

use crate::{LexErrorMsg, SyntaxKind};

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ParseErrorKind {
    #[error("{0}")]
    Message(&'static str),

    #[error("Expected {expected:?}, got {got:?}")]
    Expected {
        expected: SyntaxKind,
        got: SyntaxKind,
    },

    #[error("Lexer error: {0}")]
    Lexer(LexErrorMsg),
}

impl ParseErrorKind {
    pub fn expected(expected: SyntaxKind, got: SyntaxKind) -> ParseErrorKind {
        ParseErrorKind::Expected {
            expected,
            got,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseError {
    kind: ParseErrorKind,
    range: TextRange,
}

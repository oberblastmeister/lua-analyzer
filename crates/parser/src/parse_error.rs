use thiserror::Error;

use crate::SyntaxKind;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ParseError {
    #[error("{0}")]
    Message(&'static str),

    #[error("Expected {expected:?}, got {got:?}")]
    Expected {
        expected: SyntaxKind,
        got: SyntaxKind,
    },
}

impl ParseError {
    pub fn expected(expected: SyntaxKind, got: SyntaxKind) -> ParseError {
        ParseError::Expected { expected, got }
    }
}

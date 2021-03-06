use core::fmt;

use thiserror::Error;

use crate::SyntaxKind;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ParseError {
    #[error("{0}")]
    Message(String),

    #[error("Expected {expected:?}, got {got:?}")]
    Expected { expected: SyntaxKind, got: SyntaxKind },
}

impl ParseError {
    pub fn msg<S: fmt::Display>(msg: S) -> ParseError {
        ParseError::Message(msg.to_string())
    }

    pub fn expected(expected: SyntaxKind, got: SyntaxKind) -> ParseError {
        ParseError::Expected { expected, got }
    }
}

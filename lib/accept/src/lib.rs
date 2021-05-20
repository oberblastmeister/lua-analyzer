//! A library to create lexers or anything that does accepting easily. Inspired by Rob Pike's lexer but made rusty.

pub mod source;
mod accept;

pub use self::accept::{Lexable, Accept, Acceptor, Any, combinators};

use std::str::Chars;

use rowan::TextSize;

use crate::{SyntaxKind, T};

pub(crate) const EOF_CHAR: char = '\0';

/// A lua token
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Token {
    /// The kind of token.
    pub kind: SyntaxKind,
    /// The length of the token.
    pub len: TextSize,
}

impl Token {
    fn new(kind: SyntaxKind, len: u32) -> Token {
        Token {
            kind,
            len: TextSize::from(len),
        }
    }
}

pub struct Lexer<'a> {
    input_len: u32,
    chars: Chars<'a>,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Lexer<'a> {
        Lexer {
            input_len: input.len() as u32,
            chars: input.chars(),
        }
    }

    fn chars(&self) -> Chars<'a> {
        self.chars.clone()
    }

    fn nth(&self, n: usize) -> char {
        self.chars().nth(n).unwrap_or(EOF_CHAR)
    }

    /// Peeks next char from stream without consuming it
    fn peek(&self) -> char {
        self.nth(0)
    }

    /// Checks if there is nothing more to consume.
    fn is_eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    fn chars_len(&self) -> u32 {
        self.chars.as_str().len() as u32
    }

    /// Returns amount of already consumed symbols.
    fn pos(&self) -> u32 {
        self.input_len - self.chars_len()
    }

    fn bump(&mut self) -> Option<char> {
        self.chars.next()
    }

    fn bump_peek(&mut self) -> char {
        self.bump();
        self.peek()
    }

    fn lex_main(&mut self) -> SyntaxKind {
        let c = self.peek();

        let kind = match c {
            '(' => T!['('],
            ')' => T![')'],
            '{' => T!['{'],
            '}' => T!['}'],
            '[' => T!['['],
            ']' => T![']'],

            ',' => T![,],
            '.' => T![.],
            ';' => T![;],
            ':' => T![:],

            '+' => T![+],
            '*' => T![*],
            '/' => T![/],

            '!' => T![!],

            '-' => match self.bump_peek() {
                '-' => self.comment(),
                _ => T![-],
            },

            '\0' => T![eof],

            _ => T![unknown],
        };

        kind
    }

    fn comment(&mut self) -> SyntaxKind {
        assert_eq!(self.peek(), '-');
        self.chars.find(|c| *c == '\n');
        T![comment]
    }
}

use std::convert::TryFrom;
use std::str::Chars;

/// A trait that defines the source that a lexer or parser can operate on
pub trait Source {
    type Item;

    fn advance(&mut self) -> Option<Self::Item>;

    fn bump_raw(&mut self) -> Self::Item {
        self.advance().unwrap()
    }

    fn bump_expect(&mut self, msg: &str) -> Self::Item {
        self.advance().expect(msg)
    }

    fn nth(&self, n: u32) -> Self::Item;

    fn current(&self) -> Self::Item {
        self.nth(0)
    }

    fn is_eof(&self) -> bool {
        self.nth_is_eof(0)
    }

    fn nth_is_eof(&self, n: u32) -> bool;
}

pub(crate) const EOF_CHAR: char = '\0';

/// A simple source that works on chars
pub struct CharSource<'a> {
    input_len: u32,
    chars: Chars<'a>,
}

impl<'a> CharSource<'a> {
    pub fn new(text: &'a str) -> CharSource<'a> {
        CharSource { input_len: u32::try_from(text.len()).unwrap(), chars: text.chars() }
    }

    pub fn chars(&self) -> Chars<'a> {
        self.chars.clone()
    }

    pub fn chars_len(&self) -> u32 {
        self.chars.as_str().len() as u32
    }

    /// Returns amount of already consumed chars.
    pub fn pos(&self) -> u32 {
        self.input_len - self.chars_len()
    }

    /// The rest of the str that hasn't been consumed yet
    pub fn rest(&self) -> &'a str {
        self.chars.as_str()
    }
}

impl Source for CharSource<'_> {
    type Item = char;

    fn advance(&mut self) -> Option<char> {
        self.chars.next()
    }

    fn nth(&self, n: u32) -> char {
        self.chars().nth(n as usize).unwrap_or(EOF_CHAR)
    }

    fn is_eof(&self) -> bool {
        self.nth_is_eof(0)
    }

    fn nth_is_eof(&self, n: u32) -> bool {
        self.chars.clone().nth(n as usize).is_none()
    }
}

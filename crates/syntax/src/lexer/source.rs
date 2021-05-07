use std::convert::TryFrom;
use std::str::Chars;

use accept::Advancer;

pub(crate) const EOF_CHAR: char = '\0';

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

impl Advancer for CharSource<'_> {
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

use std::str::Chars;
use std::{iter, ops::Range};

use itertools::{Either, Itertools};
use rowan::{TextRange, TextSize};
use thiserror::Error;

use crate::{SyntaxError, SyntaxKind, T};
use parser::Token;

pub(crate) const EOF_CHAR: char = '\0';

macro_rules! done {
    ($expr:expr) => {
        return Ok($expr);
    };
}

macro_rules! assert_matches {
    ($expr:expr, $($stuff:tt)+) => {
        assert!(matches!($expr, $($stuff)+))
    };
}

struct WithLen<T> {
    inner: T,
    len: TextSize,
}

impl<T> WithLen<T> {
    fn new(t: T, len: TextSize) -> Self {
        Self { inner: t, len }
    }
}

pub fn tokenize(text: &str) -> (Vec<Token>, Vec<SyntaxError>) {
    let mut tokens = vec![];
    let mut errors = vec![];
    for res in tokenize_iter(text) {
        match res {
            Ok(tok) => tokens.push(tok),
            Err(e) => {
                tokens.push(e.to_unknown_token());
                errors.push(e.into());
            }
        }
    }
    (tokens, errors)
}

pub fn tokenize_iter(mut input: &str) -> impl Iterator<Item = Result<Token, SyntaxError>> + '_ {
    let mut pos = TextSize::from(0);

    iter::from_fn(move || {
        if input.is_empty() {
            return None;
        }

        match first_token(input) {
            Ok(token) => {
                let len = token.len;

                let token = Token::new(token.inner, TextRange::at(pos, len));

                pos += len;
                input = &input[len.into()..];

                Some(Ok(token))
            }
            Err(err) => {
                let len = err.len;

                let e = SyntaxError::new(err.inner.0, TextRange::at(pos, len));

                pos += len;
                input = &input[len.into()..];

                Some(Err(e))
            }
        }
    })
}

fn first_token(input: &str) -> Result<WithLen<SyntaxKind>, WithLen<LexErrorMsg>> {
    Lexer::new(input).next_token()
}

#[derive(Debug, Error)]
#[error("{msg}")]
pub struct LexError {
    pub msg: LexErrorMsg,
    pub range: TextRange,
}

impl LexError {
    pub fn to_unknown(&self) -> Token {
        Token {
            kind: T![unknown],
            range: self.range,
        }
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
#[error("{0}")]
pub struct LexErrorMsg(String);

type LexResult<T, E = LexErrorMsg> = Result<T, E>;

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

    fn nth(&self, n: u32) -> char {
        self.chars().nth(n as usize).unwrap_or(EOF_CHAR)
    }

    fn eof(&self) -> bool {
        self.at('\0')
    }

    /// Peeks next char from stream without consuming it
    fn current(&self) -> char {
        self.nth(0)
    }

    fn at<T: Peek>(&self, t: T) -> bool {
        T::peek(t, self)
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

    fn bump_then(&mut self) -> char {
        self.bump();
        self.current()
    }

    fn next_token(mut self) -> Result<WithLen<SyntaxKind>, WithLen<LexErrorMsg>> {
        self.lex_main()
            .map(|kind| WithLen::new(kind, self.pos().into()))
            .map_err(|e| WithLen::new(e, self.pos().into()))
    }

    fn accept<T: Accept + Copy>(&mut self, t: T) -> bool {
        T::accept(t, self)
    }

    fn accept_while<T: Accept + Copy>(&mut self, t: T) {
        T::accept_while(t, self)
    }

    fn accept_while_count<T: Accept + Copy>(&mut self, t: T) -> u32 {
        T::accept_while_count(t, self)
    }

    fn accept_repeat<T: Accept + Copy>(&mut self, t: T, repeat: u32) -> bool {
        T::accept_repeat(t, self, repeat)
    }

    fn lex_main(&mut self) -> LexResult<SyntaxKind> {
        let c = self.current();

        // return on special cases
        let kind = match c {
            '=' => match self.bump_then() {
                '=' => {
                    self.bump();
                    done!(T![==])
                }
                _ => done!(T![=]),
            },
            '~' => match self.bump_then() {
                '=' => {
                    self.bump();
                    done!(T![~=]);
                }
                _ => done!(T![unknown]),
            },

            '(' => T!['('],
            ')' => T![')'],
            '{' => T!['{'],
            '}' => T!['}'],
            '[' => match self.bump_then() {
                '[' => done!(self.multiline_string()?),
                '=' => done!(self.multiline_string()?),
                _ => done!(T!['[']),
            },
            ']' => T![']'],

            ',' => T![,],
            '.' => match self.bump_then() {
                '.' => match self.bump_then() {
                    '.' => {
                        self.bump();
                        done!(T![...])
                    }
                    _ => done!(T![..]),
                },
                _ => done!(T![.]),
            },
            ';' => T![;],
            ':' => match self.bump_then() {
                ':' => {
                    self.bump();
                    done!(T![::]);
                }
                _ => done!(T![:]),
            },

            '+' => T![+],
            '*' => T![*],
            '/' => T![/],
            '%' => T![%],
            '^' => T![^],

            '!' => T![!],

            '-' => match self.bump_then() {
                '-' => done!(self.comment()),
                _ => done!(T![-]),
            },

            c @ '\'' | c @ '"' => done!(self.string(c)?),

            _ if is_ident_start(c) => done!(self.ident()),
            _ if is_number(c) => done!(self.number()?),
            _ if is_whitespace(c) => done!(self.whitespace()),

            _ => T![unknown],
        };

        // if we got here, that means that the token was only length 1
        self.bump().unwrap();

        Ok(kind)
    }

    fn number(&mut self) -> LexResult<SyntaxKind> {
        assert!(self.at(is_number));

        if self.accept(('0', 'x')) {
            self.accept_while(is_hex);
            done!(T![number]);
        }

        self.accept_while(is_number);
        if self.accept('.') {
            self.accept_while(is_number);
        }
        if self.accept('e') || self.accept('E') {
            self.accept('-');
            self.accept_while(is_number);
        }

        Ok(T![number])
    }

    fn whitespace(&mut self) -> SyntaxKind {
        assert!(self.at(is_whitespace));

        self.accept_while(is_whitespace);
        T![whitespace]
    }

    fn comment(&mut self) -> SyntaxKind {
        assert!(self.at('-'));

        self.bump().unwrap();

        self.chars.find(|c| *c == '\n');

        T![comment]
    }

    fn bracket_enclosed(&mut self) -> LexResult<()> {
        fn close(l: &mut Lexer<'_>, count: u32) -> LexResult<()> {
            let mut err = Ok(());

            let mut set_err = || {
                if err.is_ok() {
                    err = Err(LexErrorMsg("Invalid bracket notation".to_string()));
                }
            };

            macro_rules! expect {
                ($it:expr) => {
                    if !$it {
                        set_err()
                    }
                };
            }

            expect!(l.accept('['));

            l.accept_while(|c| c != ']');

            expect!(l.accept(']'));

            let close_count = l.accept_while_count('=');
            if count != close_count {
                set_err();
                l.accept_while('=');
            }

            expect!(l.accept(']'));

            err
        }

        assert_matches!(self.current(), '[' | '=');

        if self.at('[') {
            close(self, 0)
        } else if self.at('=') {
            let count = self.accept_while_count('=');
            close(self, count)
        } else {
            Ok(())
        }
    }

    fn multiline_string(&mut self) -> LexResult<SyntaxKind> {
        assert!(self.at('[') || self.at('='));

        self.bracket_enclosed().map(|()| T![str])
    }

    fn multiline_comment(&mut self) -> LexResult<SyntaxKind> {
        assert!(self.at('[') || self.at('='));

        self.bracket_enclosed().map(|()| T![comment])
    }

    fn string(&mut self, delimit: char) -> LexResult<SyntaxKind> {
        assert!(matches!(delimit, '\'' | '"'));
        assert_eq!(self.current(), delimit);
        self.bump().unwrap();

        self.chars
            .find(|c| *c == delimit)
            .ok_or(LexErrorMsg("UnfinishedString".into()))?;

        Ok(T![str])
    }

    fn ident(&mut self) -> SyntaxKind {
        assert!(self.at(is_ident_start));

        let start = self.pos();
        let text = self.chars.as_str();

        self.bump().unwrap();
        self.accept_while(is_ident_continue);

        let text = &text[0..(self.pos() - start) as usize];
        SyntaxKind::from_keyword(text).unwrap_or(T![ident])
    }
}

pub trait Peek {
    fn nth(self, p: &Lexer<'_>, n: u32) -> bool;

    fn peek(self, p: &Lexer<'_>) -> bool
    where
        Self: Sized,
    {
        self.nth(p, 0)
    }
}

trait Accept: Peek {
    fn accept(self, l: &mut Lexer<'_>) -> bool
    where
        Self: Peek + Sized,
    {
        if self.peek(l) && !l.eof() {
            l.bump().unwrap();
            true
        } else {
            false
        }
    }

    fn accept_while(self, l: &mut Lexer<'_>)
    where
        Self: Peek + Sized + Copy,
    {
        while self.accept(l) {}
    }

    fn accept_while_count(self, l: &mut Lexer<'_>) -> u32
    where
        Self: Peek + Sized + Copy,
    {
        let mut count = 0;
        while self.accept(l) {
            count += 1;
        }
        count
    }

    fn accept_repeat(self, l: &mut Lexer<'_>, repeat: u32) -> bool
    where
        Self: Peek + Sized + Copy,
    {
        if (0..repeat).all(|i| self.nth(l, i)) {
            for _ in 0..repeat {
                l.bump().unwrap();
            }
            true
        } else {
            false
        }
    }
}

impl Peek for char {
    fn nth(self, l: &Lexer<'_>, n: u32) -> bool {
        l.nth(n) == self
    }
}
impl Accept for char {}

impl<T: Fn(char) -> bool> Peek for T {
    fn nth(self, l: &Lexer<'_>, n: u32) -> bool {
        self(l.nth(n))
    }
}
impl<T: Fn(char) -> bool> Accept for T {}

impl Peek for Range<char> {
    fn nth(self, l: &Lexer<'_>, n: u32) -> bool {
        self.contains(&l.nth(n))
    }
}
impl Accept for Range<char> {}

macro_rules! tuple_impls {
    { $( ($( $n:tt $name:ident )+ $(,)?) ),+ $(,)? } => {
        $(
            impl<$($name),+> Peek for ($($name),+)
            where
                $($name: Peek,)+
                {
                    fn nth(self, l: &Lexer<'_>, n: u32) -> bool {
                        $( self.$n.nth(l, $n + n) )&&+
                    }
                }

            impl<$($name),+> Accept for ($($name),+)
            where
                $($name: Accept,)+
                {
                    fn accept(self, l: &mut Lexer<'_>) -> bool {
                        if self.peek(l) {
                            $( $n; l.bump().unwrap(); )+
                            true
                        } else {
                            false
                        }
                    }
                }
        )+
    };
}

tuple_impls! {
    (0 TO 1 T1),
    (0 TO 1 T1 2 T2),
    (0 TO 1 T1 2 T2 3 T3),
    (0 TO 1 T1 2 T2 3 T3 4 T4),
    (0 TO 1 T1 2 T2 3 T3 4 T4 5 T5),
    (0 TO 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6),
    (0 TO 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7),
    (0 TO 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8),
}

const fn is_number(c: char) -> bool {
    matches!(c, '0'..='9')
}

const fn is_ident_start(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

const fn is_ident_continue(c: char) -> bool {
    is_ident_start(c) || matches!(c, '0'..='9')
}

const fn is_whitespace(c: char) -> bool {
    c.is_ascii_whitespace()
}

const fn is_hex(c: char) -> bool {
    matches!(c, '0'..='9' | 'A'..='F' | 'a'..='f')
}

#[cfg(test)]
mod tests {
    use std::str;

    use super::*;

    #[test]
    fn accept_tuple() {
        let mut lexer = Lexer::new("[=");
        lexer.accept(('[', '='));
        assert!(lexer.eof());
    }

    #[test]
    fn accept_tuple_fail() {
        let mut lexer = Lexer::new("[]");
        lexer.accept(('[', '='));
        assert_eq!(lexer.pos(), 0);
        assert_eq!(lexer.current(), '[');
        lexer.bump().unwrap();
        assert_eq!(lexer.current(), ']');
    }

    #[test]
    fn accept_repeat() {
        let mut lexer = Lexer::new("===================");
        lexer.accept_repeat('=', 19);
        assert!(lexer.eof());
    }

    #[test]
    fn accept_repeat_not_enough() {
        let mut lexer = Lexer::new("==");
        lexer.accept_repeat('=', 4);
        assert_eq!(lexer.pos(), 0);
        lexer.accept_repeat('=', 2);
        assert!(lexer.eof());
    }

    #[test]
    fn accept_repeat_none() {
        let mut lexer = Lexer::new("not");
        assert!(lexer.accept_repeat('=', 0));
        assert_eq!(lexer.pos(), 0);
    }
}

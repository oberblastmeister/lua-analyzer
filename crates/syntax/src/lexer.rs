use std::str::Chars;
use std::{iter, ops::Range};

use rowan::{TextRange, TextSize};
use thiserror::Error;

use crate::accept::{and, not, or, seq, Accept, Any, Lexable};
use crate::{SyntaxError, SyntaxKind, T};
use parser::Token;

pub(crate) const EOF_CHAR: char = '\0';

macro_rules! done {
    ($expr:expr) => {
        return ($expr, None);
    };
}

macro_rules! bail {
    () => {
        return (T![unknown], None)
    };
    (T![$match:tt]) => {
        return (T![$match], None)
    };
    (T![$match:tt], $( $stuff:tt )*) => {
        return (T![$match], Some(LexErrorMsg(format!($( $stuff)* ))))
    };
    ($expr:expr, $( $stuff:tt )*) => {
        return ($expr, Some(LexErrorMsg(format!($( $stuff)* ))))
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

/// A token with an optional error
pub struct TokenErr {
    pub token: Token,
    pub err: Option<SyntaxError>,
}

impl TokenErr {
    pub fn new(token: Token, err: Option<SyntaxError>) -> Self {
        Self { token, err }
    }
}

pub fn tokenize(text: &str) -> (Vec<Token>, Vec<SyntaxError>) {
    let mut errors = Vec::new();
    let tokens = tokenize_iter(text)
        .map(|TokenErr { token, err }| {
            errors.extend(err);
            token
        })
        .collect();

    (tokens, errors)
}

pub fn tokenize_iter(mut input: &str) -> impl Iterator<Item = TokenErr> + '_ {
    let mut pos = TextSize::from(0);

    iter::from_fn(move || {
        let mut token_err = lex_first_token(input)?;
        token_err.token.range += pos;
        token_err.err.as_mut().map(|err| {
            err.1 += pos;
        });

        let len = token_err.token.range.len();

        pos += len;
        input = &input[len.into()..];

        Some(token_err)
    })
}

pub fn lex_first_token(text: &str) -> Option<TokenErr> {
    if text.is_empty() {
        return None;
    }

    let WithLen { len, inner: (kind, err) } = first_lex_result(text);
    let range = TextRange::up_to(len);
    let token = Token::new(kind, range);
    let err = err.map(|err| SyntaxError::new(err.0, range));
    Some(TokenErr::new(token, err))
}

fn first_lex_result(text: &str) -> WithLen<LexResult<SyntaxKind>> {
    Lexer::new(text).next_lex_result()
}

#[derive(Debug, Error)]
#[error("{msg}")]
pub struct LexError {
    pub msg: LexErrorMsg,
    pub range: TextRange,
}

impl LexError {
    pub fn to_unknown(&self) -> Token {
        Token { kind: T![unknown], range: self.range }
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
#[error("{0}")]
pub struct LexErrorMsg(String);

type LexResult<T> = (T, Option<LexErrorMsg>);

pub struct Lexer<'a> {
    input_len: u32,
    chars: Chars<'a>,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Lexer<'a> {
        Lexer { input_len: input.len() as u32, chars: input.chars() }
    }

    pub(crate) fn chars(&self) -> Chars<'a> {
        self.chars.clone()
    }

    /// Returns nth character relative to the current cursor position.
    /// If requested position doesn't exist, `EOF_CHAR` is returned.
    /// However, getting `EOF_CHAR` doesn't always mean actual end of file,
    /// it should be checked with `is_eof` method.
    pub(crate) fn nth(&self, n: u32) -> char {
        self.chars().nth(n as usize).unwrap_or(EOF_CHAR)
    }

    pub(crate) fn is_eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    /// Peeks next char from stream without consuming it
    fn current(&self) -> char {
        self.nth(0)
    }

    fn at<T: Lexable>(&self, t: T) -> bool {
        t.peek(self)
    }

    fn chars_len(&self) -> u32 {
        self.chars.as_str().len() as u32
    }

    /// Returns amount of already consumed chars.
    fn pos(&self) -> u32 {
        self.input_len - self.chars_len()
    }

    pub(crate) fn bump_raw(&mut self) -> Option<char> {
        self.chars.next()
    }

    fn bump_then(&mut self, accept: impl Accept + Copy) -> char {
        self.bump(accept);
        self.current()
    }

    fn next_lex_result(mut self) -> WithLen<LexResult<SyntaxKind>> {
        let res = self.lex_main();
        let pos = self.pos();
        WithLen::new(res, pos.into())
    }

    fn bump<T: Accept + Copy>(&mut self, t: T) {
        t.bump(self)
    }

    fn accept<T: Accept + Copy>(&mut self, t: T) -> bool {
        t.accept(self)
    }

    fn accept_while<T: Accept + Copy>(&mut self, t: T) {
        t.accept_while(self)
    }

    fn accept_until<T: Accept + Copy>(&mut self, t: T) {
        t.accept_until(self)
    }

    fn accept_while_count<T: Accept + Copy>(&mut self, t: T) -> u32 {
        t.accept_while_count(self)
    }

    fn accept_repeat<T: Accept + Copy>(&mut self, t: T, repeat: u32) -> bool {
        t.accept_repeat(self, repeat)
    }

    fn lex_main(&mut self) -> LexResult<SyntaxKind> {
        let c = self.current();

        // return on special cases
        let kind = match c {
            '=' => match self.bump_then(Any) {
                '=' => {
                    self.bump('=');
                    done!(T![==])
                }
                _ => done!(T![=]),
            },
            '~' => match self.bump_then(Any) {
                '=' => {
                    self.bump('=');
                    done!(T![~=]);
                }
                _ => done!(T![unknown]),
            },

            '(' => T!['('],
            ')' => T![')'],
            '{' => T!['{'],
            '}' => T!['}'],
            '[' => match self.bump_then(Any) {
                '[' => return self.multiline_string(),
                '=' => return self.multiline_string(),
                _ => done!(T!['[']),
            },
            ']' => T![']'],

            ',' => T![,],
            '.' => match self.bump_then(Any) {
                '.' => match self.bump_then(Any) {
                    '.' => {
                        self.bump('.');
                        done!(T![...])
                    }
                    _ => done!(T![..]),
                },
                _ => done!(T![.]),
            },
            ';' => T![;],
            ':' => match self.bump_then(Any) {
                ':' => {
                    self.bump(':');
                    done!(T![::]);
                }
                _ => done!(T![:]),
            },

            '+' => T![+],
            '*' => T![*],
            '/' => T![/],
            '%' => T![%],
            '^' => T![^],

            '>' => match self.bump_then(Any) {
                '=' => {
                    self.bump('=');
                    done!(T![>=]);
                }
                _ => {
                    done!(T![>]);
                }
            },
            '<' => match self.bump_then(Any) {
                '=' => {
                    self.bump('=');
                    done!(T![<=]);
                }
                _ => {
                    done!(T![<]);
                }
            },

            '!' => T![!],

            '-' => match self.bump_then(Any) {
                '-' => done!(self.comment()),
                _ => done!(T![-]),
            },

            c @ '\'' | c @ '"' => return self.string(c),

            _ if is_ident_start(c) => done!(self.ident()),
            _ if is_number(c) => return self.number(),
            _ if is_whitespace(c) => done!(self.whitespace()),

            _ => T![unknown],
        };

        // if we got here, that means that the token was only length 1
        self.bump(Any);

        (kind, None)
    }

    fn number(&mut self) -> LexResult<SyntaxKind> {
        assert!(self.at(is_number));

        if self.accept(seq!('0', 'x')) {
            self.accept_while(is_hex);
            return (T![number], None);
        }

        self.accept_while(is_number);
        if self.accept('.') {
            self.accept_while(is_number);
        }
        if self.accept('e') || self.accept('E') {
            self.accept('-');
            self.accept_while(is_number);
        }

        done!(T![number]);
    }

    fn whitespace(&mut self) -> SyntaxKind {
        assert!(self.at(is_whitespace));

        self.accept_while(is_whitespace);
        T![whitespace]
    }

    fn comment(&mut self) -> SyntaxKind {
        self.bump('-');

        self.chars.find(|c| *c == '\n');

        T![comment]
    }

    fn bracket_enclosed(&mut self) -> LexResult<()> {
        fn close(l: &mut Lexer<'_>, count: u32) -> LexResult<()> {
            let mut err = None;

            let mut set_err = || {
                if err.is_none() {
                    err = Some(LexErrorMsg("Invalid bracket notation".to_string()));
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

            loop {
                l.accept_while(or!(seq!('\\', ']'), not!(']')));

                if !l.accept(']') {
                    // we hit eof
                    bail!((), "Could not find bracket string close");
                }

                let close_count = l.accept_while_count('=');
                if count != close_count {
                    continue;
                } else {
                    break;
                }
            }

            expect!(l.accept(']'));

            ((), err)
        }

        assert_matches!(self.current(), '[' | '=');

        if self.at('[') {
            close(self, 0)
        } else if self.at('=') {
            let count = self.accept_while_count('=');
            close(self, count)
        } else {
            ((), None)
        }
    }

    fn multiline_string(&mut self) -> LexResult<SyntaxKind> {
        assert!(self.at(or!('[', '=')));

        let res = self.bracket_enclosed();
        (T![str], res.1)
    }

    fn multiline_comment(&mut self) -> LexResult<SyntaxKind> {
        assert!(self.at(or!('[', '=')));

        let res = self.bracket_enclosed();
        (T![comment], res.1)
    }

    fn string(&mut self, delimit: char) -> LexResult<SyntaxKind> {
        assert_matches!(delimit, '\'' | '"');
        self.bump(delimit);

        loop {
            match self.current() {
                '\0' => bail!(T![str], "Could not find closing delimiter `{}`", delimit),
                '\n' => bail!(T![str], "Unexpected newline in string"),
                c if c == delimit => {
                    self.bump(delimit);
                    break;
                }
                _ => (),
            }
            self.bump(or!(seq!('\\', delimit), Any));
        }

        (T![str], None)
    }

    fn ident(&mut self) -> SyntaxKind {
        let start = self.pos();
        let text = self.chars.as_str();

        self.bump(is_ident_start);
        self.accept_while(is_ident_continue);

        let text = &text[0..(self.pos() - start) as usize];
        SyntaxKind::from_keyword(text).unwrap_or(T![ident])
    }
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
    use super::*;

    #[test]
    fn accept_tuple() {
        let mut lexer = Lexer::new("[=");
        lexer.accept(seq!('[', '='));
        assert!(lexer.is_eof());
    }

    #[test]
    fn accept_tuple_fail() {
        let mut lexer = Lexer::new("[]");
        lexer.accept(seq!('[', '='));
        assert_eq!(lexer.pos(), 0);
        assert_eq!(lexer.current(), '[');
        lexer.bump_raw().unwrap();
        assert_eq!(lexer.current(), ']');
    }

    #[test]
    fn accept_repeat() {
        let mut lexer = Lexer::new("===================");
        lexer.accept_repeat('=', 19);
        assert!(lexer.is_eof());
    }

    #[test]
    fn accept_repeat_not_enough() {
        let mut lexer = Lexer::new("==");
        lexer.accept_repeat('=', 4);
        assert_eq!(lexer.pos(), 0);
        lexer.accept_repeat('=', 2);
        assert!(lexer.is_eof());
    }

    #[test]
    fn accept_repeat_none() {
        let mut lexer = Lexer::new("not");
        assert!(lexer.accept_repeat('=', 0));
        assert_eq!(lexer.pos(), 0);
    }

    #[test]
    fn not() {
        let mut lexer = Lexer::new(r"\]");
        assert!(!lexer.accept(not!(seq!('\\', ']'))));
        assert_eq!(lexer.pos(), 0);
    }

    #[test]
    fn combinations() {
        let mut lexer = Lexer::new(r"\]    \]  ]]");
        lexer.accept_while(or!(seq!('\\', ']'), not!(']')));
        assert_eq!(lexer.pos(), 10);
    }

    #[test]
    fn run_current() {
        tokenize("local hello = 5");
    }
}

mod error;

use std::iter;
use std::str::Chars;

use rowan::{TextRange, TextSize};

use self::error::{LexResult, SyntaxResult};
use crate::accept::{not, or, seq, Accept, Any, Lexable, Repeat, Until, While};
use crate::{SyntaxError, SyntaxKind, T};
use parser::Token;

pub(crate) const EOF_CHAR: char = '\0';

macro_rules! done {
    ($expr:expr) => {
        return LexResult::new($expr, None);
    };
}

macro_rules! bail {
    (T![$match:tt]) => {
        return LexResult::new(T![$match], None)
    };
    (T![$match:tt], $( $stuff:tt )*) => {
        return LexResult::new(T![$match], Some(format!($( $stuff)* )))
    };
    ($expr:expr, $( $stuff:tt )*) => {
        return LexResult::new($expr, Some(format!($( $stuff)* )))
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

/// Tokenize text into a vector of tokens and errors
pub fn tokenize(text: &str) -> (Vec<Token>, Vec<SyntaxError>) {
    let mut errors = Vec::new();
    let tokens = tokenizer(text)
        .map(|res| {
            let (token, err) = res.inner();
            errors.extend(err);
            token
        })
        .collect();

    (tokens, errors)
}

/// Returns a tokenizer iterator that will turn the source text into tokens
pub fn tokenizer(mut text: &str) -> impl Iterator<Item = SyntaxResult<Token>> + '_ {
    let mut pos = TextSize::from(0);

    iter::from_fn(move || {
        let res = lex_first_token(text)?;

        let range = res.ok_ref().range;

        let res = res
            .map(|token| token.with_range(range + pos))
            .map_err(|err| err.with_range(range + pos));

        let len = range.len();

        pos += len;
        text = &text[len.into()..];

        Some(res)
    })
}

pub fn lex_first_syntax_kind(text: &str) -> Option<SyntaxResult<SyntaxKind>> {
    Some(lex_first_token(text)?.map(|token| token.kind))
}

pub fn lex_first_token(text: &str) -> Option<SyntaxResult<Token>> {
    if text.is_empty() {
        return None;
    }

    let WithLen { len, inner: lex_result } = first_lex_result(text);
    let range = TextRange::up_to(len);
    Some(
        lex_result
            .map(|kind| Token::new(kind, range))
            .map_err(|e| SyntaxError::new(e.to_string(), range)),
    )
}

fn first_lex_result(text: &str) -> WithLen<LexResult<SyntaxKind>> {
    Lexer::new(text).next_lex_result()
}

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

    fn accept_count<T: Accept + Copy>(&mut self, t: T) -> u32 {
        t.accept_count(self)
    }

    fn lex_main(&mut self) -> LexResult<SyntaxKind> {
        let c = self.current();

        // return on special cases
        let kind = match c {
            // '\0' if self.is_eof() => done!(T![eof]),
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

            '#' => T![#],
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
                '-' => return self.comment(),
                _ => done!(T![-]),
            },

            c @ '\'' | c @ '"' => return self.string(c),

            _ if is_ident_start(c) => done!(self.ident()),
            _ if is_number(c) => return self.number(),
            _ if is_whitespace(c) => done!(self.whitespace()),

            _ => {
                self.bump(Any);
                bail!(T![unknown], "Got an unknown token")
            }
        };

        // if we got here, that means that the token was only length 1
        self.bump(Any);

        LexResult::just(kind)
    }

    fn number(&mut self) -> LexResult<SyntaxKind> {
        assert!(self.at(is_number));

        if self.accept(seq!('0', 'x')) {
            self.accept(While(is_hex));
            return LexResult::just(T![number]);
        }

        self.accept(While(is_number));
        if self.accept('.') {
            self.accept(While(is_number));
        }
        if self.accept('e') || self.accept('E') {
            self.accept('-');
            self.accept(While(is_number));
        }

        done!(T![number]);
    }

    fn whitespace(&mut self) -> SyntaxKind {
        assert!(self.at(is_whitespace));

        self.accept(While(is_whitespace));

        T![whitespace]
    }

    fn comment(&mut self) -> LexResult<SyntaxKind> {
        self.bump('-');

        if self.accept('[') {
            if self.at('[') {
                self.bracket_enclosed();
            }
        }

        self.accept(Until('\n'));

        LexResult::just(T![comment])
    }

    fn bracket_enclosed(&mut self) -> LexResult<()> {
        fn close(l: &mut Lexer<'_>, count: u32) -> LexResult<()> {
            let mut err = None;

            let mut set_err = || {
                if err.is_none() {
                    err = Some("Invalid bracket notation".to_string());
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
                l.accept(While(or!(seq!('\\', ']'), not!(']'))));

                if !l.accept(']') {
                    // we hit eof
                    bail!((), "Could not find bracket string close");
                }

                let close_count = l.accept_count(While('='));
                if count != close_count {
                    continue;
                } else {
                    break;
                }
            }

            expect!(l.accept(']'));

            LexResult::unit(err)
        }

        assert_matches!(self.current(), '[' | '=');

        if self.at('[') {
            close(self, 0)
        } else if self.at('=') {
            let count = self.accept_count(While('='));
            close(self, count)
        } else {
            LexResult::unit(None)
        }
    }

    fn multiline_string(&mut self) -> LexResult<SyntaxKind> {
        assert!(self.at(or!('[', '=')));

        self.bracket_enclosed().map(|_| T![str])
    }

    fn multiline_comment(&mut self) -> LexResult<SyntaxKind> {
        assert!(self.at(or!('[', '=')));

        self.bracket_enclosed().map(|_| T![comment])
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

        LexResult::just(T![str])
    }

    fn ident(&mut self) -> SyntaxKind {
        let start = self.pos();
        let text = self.chars.as_str();

        self.bump(is_ident_start);
        self.accept(While(is_ident_continue));

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
        lexer.accept(Repeat('=', 19));
        assert!(lexer.is_eof());
    }

    #[test]
    fn accept_repeat_not_enough() {
        let mut lexer = Lexer::new("==");
        lexer.accept(Repeat('=', 4));
        assert_eq!(lexer.pos(), 0);
        lexer.accept(Repeat('=', 2));
        assert!(lexer.is_eof());
    }

    #[test]
    fn accept_repeat_none() {
        let mut lexer = Lexer::new("not");
        assert!(lexer.accept(Repeat('=', 0)));
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
        lexer.accept(While(or!(seq!('\\', ']'), not!(']'))));
        assert_eq!(lexer.pos(), 10);
    }

    #[test]
    fn run_current() {
        tokenize("local hello = 5");
    }
}

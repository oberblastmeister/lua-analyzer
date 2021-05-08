use rowan::{TextRange, TextSize};

use crate::lexer::{
    error::{LexResult, SyntaxResult},
    Lexer,
};
use crate::{SyntaxError, SyntaxKind, T};
use accept::source::{CharSource, Source};
use accept::{
    combinators::{or, seq, Until, While, Not},
    Accept, Acceptor, Any,
};
use parser::Token;

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

pub fn lex_first_syntax_kind(text: &str) -> Option<SyntaxResult<SyntaxKind>> {
    Some(lex_first_token(text)?.map(|token| token.kind))
}

pub fn lex_first_token(text: &str) -> Option<SyntaxResult<Token>> {
    if text.is_empty() {
        return None;
    }

    let res = first_lex_result(text);
    let range = TextRange::up_to(res.ok_ref().len);
    Some(res.map_err(|e| SyntaxError::new(e.to_string(), range)))
}

fn first_lex_result(text: &str) -> LexResult<Token> {
    LuaLexer::new(text).next_lex_result()
}

pub struct LuaLexer<'a> {
    source: CharSource<'a>,
}

impl<'a> Lexer<'a> for LuaLexer<'a> {
    fn new(text: &'a str) -> LuaLexer<'a> {
        LuaLexer { source: CharSource::new(text) }
    }

    fn next_syntax_kind(&mut self) -> LexResult<SyntaxKind> {
        self.lex_main()
    }

    fn len_consumed(&self) -> TextSize {
        self.source.pos().into()
    }
}

impl<'a> LuaLexer<'a> {
    fn new(input: &'a str) -> LuaLexer<'a> {
        LuaLexer { source: CharSource::new(input) }
    }

    fn next_lex_result(mut self) -> LexResult<Token> {
        let res = self.lex_main();
        let pos = self.source.pos();
        res.map(|kind| Token { kind, len: pos.into() })
    }

    fn lex_main(&mut self) -> LexResult<SyntaxKind> {
        let c = self.source.current();

        // return on special cases
        let kind = match c {
            // '\0' if self.is_eof() => done!(T![eof]),
            '=' => match self.source.bump_then(Any) {
                '=' => {
                    self.source.bump('=');
                    done!(T![==])
                }
                _ => done!(T![=]),
            },
            '~' => match self.source.bump_then(Any) {
                '=' => {
                    self.source.bump('=');
                    done!(T![~=]);
                }
                _ => done!(T![unknown]),
            },

            '(' => T!['('],
            ')' => T![')'],
            '{' => T!['{'],
            '}' => T!['}'],
            '[' => match self.source.bump_then(Any) {
                '[' => return self.multiline_string(),
                '=' => return self.multiline_string(),
                _ => done!(T!['[']),
            },
            ']' => T![']'],

            ',' => T![,],
            '.' => match self.source.bump_then(Any) {
                '.' => match self.source.bump_then(Any) {
                    '.' => {
                        self.source.bump('.');
                        done!(T![...])
                    }
                    _ => done!(T![..]),
                },
                _ => done!(T![.]),
            },
            ';' => T![;],
            ':' => match self.source.bump_then(Any) {
                ':' => {
                    self.source.bump(':');
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

            '>' => match self.source.bump_then(Any) {
                '=' => {
                    self.source.bump('=');
                    done!(T![>=]);
                }
                _ => {
                    done!(T![>]);
                }
            },
            '<' => match self.source.bump_then(Any) {
                '=' => {
                    self.source.bump('=');
                    done!(T![<=]);
                }
                _ => {
                    done!(T![<]);
                }
            },

            '!' => T![!],

            '-' => match self.source.bump_then(Any) {
                '-' => return self.comment(),
                _ => done!(T![-]),
            },

            c @ '\'' | c @ '"' => return self.string(c),

            _ if is_ident_start(c) => done!(self.ident()),
            _ if is_number(c) => return self.number(),
            _ if is_whitespace(c) => done!(self.whitespace()),

            _ => {
                self.source.bump(Any);
                bail!(T![unknown], "Got an unknown token")
            }
        };

        // if we got here, that means that the token was only length 1
        self.source.bump(Any);

        LexResult::just(kind)
    }

    fn number(&mut self) -> LexResult<SyntaxKind> {
        assert!(self.source.at(is_number));

        if self.source.accept(seq!('0', 'x')) {
            self.source.accept(While(is_hex));
            return LexResult::just(T![number]);
        }

        self.source.accept(While(is_number));
        if self.source.accept('.') {
            self.source.accept(While(is_number));
        }
        if self.source.accept('e') || self.source.accept('E') {
            self.source.accept('-');
            self.source.accept(While(is_number));
        }

        done!(T![number]);
    }

    fn whitespace(&mut self) -> SyntaxKind {
        assert!(self.source.at(is_whitespace));

        self.source.accept(While(is_whitespace));

        T![whitespace]
    }

    fn comment(&mut self) -> LexResult<SyntaxKind> {
        self.source.bump('-');

        if self.source.accept('[') {
            if self.source.at('[') {
                self.bracket_enclosed();
            }
        }

        self.source.accept(Until('\n'));

        LexResult::just(T![comment])
    }

    fn bracket_enclosed(&mut self) -> LexResult<()> {
        fn close(l: &mut LuaLexer<'_>, count: u32) -> LexResult<()> {
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

            expect!(l.source.accept('['));

            loop {
                l.source.accept(While(or!(seq!('\\', ']'), Not(']'))));

                if !l.source.accept(']') {
                    // we hit eof
                    bail!((), "Could not find bracket string close");
                }

                let close_count = l.source.accept_count(While('='));
                if count != close_count {
                    continue;
                } else {
                    break;
                }
            }

            expect!(l.source.accept(']'));

            LexResult::unit(err)
        }

        assert_matches!(self.source.current(), '[' | '=');

        if self.source.at('[') {
            close(self, 0)
        } else if self.source.at('=') {
            let count = self.source.accept_count(While('='));
            close(self, count)
        } else {
            LexResult::unit(None)
        }
    }

    fn multiline_string(&mut self) -> LexResult<SyntaxKind> {
        assert!(self.source.at(or!('[', '=')));

        self.bracket_enclosed().map(|_| T![str])
    }

    fn multiline_comment(&mut self) -> LexResult<SyntaxKind> {
        assert!(self.source.at(or!('[', '=')));

        self.bracket_enclosed().map(|_| T![comment])
    }

    fn string(&mut self, delimit: char) -> LexResult<SyntaxKind> {
        assert_matches!(delimit, '\'' | '"');
        self.source.bump(delimit);

        loop {
            match self.source.current() {
                '\0' => bail!(T![str], "Could not find closing delimiter `{}`", delimit),
                '\n' => bail!(T![str], "Unexpected newline in string"),
                c if c == delimit => {
                    self.source.bump(delimit);
                    break;
                }
                _ => (),
            }
            self.source.bump(or!(seq!('\\', delimit), Any));
        }

        LexResult::just(T![str])
    }

    fn ident(&mut self) -> SyntaxKind {
        let start = self.source.pos();
        let text = self.source.rest();

        self.source.bump(is_ident_start);
        self.source.accept(While(is_ident_continue));

        let text = &text[0..(self.source.pos() - start) as usize];
        SyntaxKind::from_keyword(text).unwrap_or(T![ident])
    }
}

#[inline]
const fn is_number(c: char) -> bool {
    matches!(c, '0'..='9')
}

#[inline]
const fn is_ident_start(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

#[inline]
const fn is_ident_continue(c: char) -> bool {
    is_ident_start(c) || matches!(c, '0'..='9')
}

#[inline]
const fn is_whitespace(c: char) -> bool {
    c.is_ascii_whitespace()
}

#[inline]
const fn is_hex(c: char) -> bool {
    matches!(c, '0'..='9' | 'A'..='F' | 'a'..='f')
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::tokenize;
    use accept::combinators::{Repeat, Not};

    #[test]
    fn accept_tuple() {
        let mut lexer = LuaLexer::new("[=");
        lexer.source.accept(seq!('[', '='));
        assert!(lexer.source.is_eof());
    }

    #[test]
    fn accept_tuple_fail() {
        let mut lexer = LuaLexer::new("[]");
        lexer.source.accept(seq!('[', '='));
        assert_eq!(lexer.source.pos(), 0);
        assert_eq!(lexer.source.current(), '[');
        lexer.source.bump_raw();
        assert_eq!(lexer.source.current(), ']');
    }

    #[test]
    fn accept_repeat() {
        let mut lexer = LuaLexer::new("===================");
        lexer.source.accept(Repeat('=', 19));
        assert!(lexer.source.is_eof());
    }

    #[test]
    fn accept_repeat_not_enough() {
        let mut lexer = LuaLexer::new("==");
        lexer.source.accept(Repeat('=', 4));
        assert_eq!(lexer.source.pos(), 0);
        lexer.source.accept(Repeat('=', 2));
        assert!(lexer.source.is_eof());
    }

    #[test]
    fn accept_repeat_none() {
        let mut lexer = LuaLexer::new("not");
        assert!(lexer.source.accept(Repeat('=', 0)));
        assert_eq!(lexer.source.pos(), 0);
    }

    #[test]
    fn not() {
        let mut lexer = LuaLexer::new(r"\]");
        assert!(!lexer.source.accept(Not(seq!('\\', ']'))));
        assert_eq!(lexer.source.pos(), 0);
    }

    #[test]
    fn combinations() {
        let mut lexer = LuaLexer::new(r"\]    \]  ]]");
        lexer.source.accept(While(or!(seq!('\\', ']'), Not(']'))));
        assert_eq!(lexer.source.pos(), 10);
    }

    #[test]
    fn run_current() {
        tokenize::<LuaLexer>("local hello = 5");
    }
}

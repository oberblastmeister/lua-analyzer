use std::str::Chars;
use std::{iter, ops::Range};

use rowan::{TextRange, TextSize};
use thiserror::Error;

use crate::{SyntaxKind, T};

pub(crate) const EOF_CHAR: char = '\0';

macro_rules! bail {
    ($kind:ident) => {
        return Err(LexErrorKind::$kind);
    };
}

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

pub fn tokenize(mut input: &str) -> impl Iterator<Item = Result<Token, LexError>> + '_ {
    iter::from_fn(move || {
        if input.is_empty() {
            return None;
        }

        match first_token(input) {
            Ok(token) => {
                input = &input[token.len.into()..];
                Some(Ok(token))
            }
            Err(e) => {
                input = &input[e.len.into()..];
                Some(Err(e))
            }
        }
    })
}

pub fn first_token(input: &str) -> Result<Token, LexError> {
    Lexer::new(input).next_token()
}

#[derive(Debug, Error)]
enum LexErrorKind {
    #[error("There were no digits after the e")]
    WrongScientificNum,

    #[error("Unknown token")]
    Unknown,

    #[error("Unfinished string")]
    UnfinishedString,

    #[error("Invalid bracket notation")]
    InvalidBracketNotation,

    #[error("Not correct = amount")]
    NotCorrectEqualAmount,
}

#[derive(Debug)]
pub struct LexError {
    kind: LexErrorKind,
    len: TextSize,
}

impl LexError {
    fn new(kind: LexErrorKind, len: u32) -> LexError {
        LexError {
            kind,
            len: TextSize::from(len),
        }
    }
}

type LexResult<T, E = LexErrorKind> = Result<T, E>;

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
        self.peek() == '\0'
    }

    /// Peeks next char from stream without consuming it
    fn peek(&self) -> char {
        self.nth(0)
    }

    fn is<T: Peek>(&self, t: T) -> bool {
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

    fn bump_peek(&mut self) -> char {
        self.bump();
        self.peek()
    }

    fn next_token(&mut self) -> Result<Token, LexError> {
        self.lex_main()
            .map(|kind| Token::new(kind, self.pos()))
            .map_err(|e| LexError::new(e, self.pos()))
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
        let c = self.peek();

        // return on special cases
        let kind = match c {
            '\0' => panic!("Should not be gotten"),

            '(' => T!['('],
            ')' => T![')'],
            '{' => T!['{'],
            '}' => T!['}'],
            '[' => match self.bump_peek() {
                '[' => done!(self.multiline_string()?),
                '=' => done!(self.multiline_string()?),
                _ => done!(T!['[']),
            },
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
        assert!(is_number(self.peek()));

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
        assert!(is_whitespace(self.peek()));
        self.accept_while(is_whitespace);
        T![whitespace]
    }

    fn comment(&mut self) -> SyntaxKind {
        assert_eq!(self.peek(), '-');
        self.bump().unwrap();

        self.chars.find(|c| *c == '\n');

        T![comment]
    }

    fn bracket_enclosed(&mut self) -> LexResult<()> {
        fn close(l: &mut Lexer<'_>, count: u32) -> LexResult<()> {
            let mut err = Ok(());
            let mut set_err = || {
                if err.is_ok() {
                    err = Err(LexErrorKind::InvalidBracketNotation);
                }
            };

            macro_rules! expect {
                ($it:expr, $e:ident) => {
                    if !$it {
                        set_err()
                    }
                };
            }

            expect!(l.accept('['), NotCorrectEqualAmount);

            l.accept_while(|c| c != ']');

            expect!(l.accept(']'), InvalidBracketNotation);
            expect!(l.accept_repeat('=', count), NotCorrectEqualAmount);
            expect!(l.accept(']'), InvalidBracketNotation);

            err
        }

        assert_matches!(self.peek(), '[' | '=');

        if self.is('[') {
            close(self, 0)
        } else if self.is('=') {
            let count = self.accept_while_count('=');
            close(self, count)
        } else {
            Ok(())
        }
    }

    fn multiline_string(&mut self) -> LexResult<SyntaxKind> {
        assert_matches!(self.peek(), '[' | '=');

        self.bracket_enclosed().map(|()| T![str])
    }

    fn multiline_comment(&mut self) -> LexResult<SyntaxKind> {
        assert_matches!(self.peek(), '[' | '=');

        self.bracket_enclosed().map(|()| T![comment])
    }

    fn string(&mut self, delimit: char) -> LexResult<SyntaxKind> {
        assert!(matches!(delimit, '\'' | '"'));
        assert_eq!(self.peek(), delimit);
        self.bump().unwrap();

        self.chars
            .find(|c| *c == delimit)
            .ok_or(LexErrorKind::UnfinishedString)?;

        Ok(T![str])
    }

    fn ident(&mut self) -> SyntaxKind {
        assert!(is_ident_start(self.peek()));

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
        if self.peek(l) {
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
    use super::*;

    fn get_text(
        input: &str,
        tokens: impl Iterator<Item = Result<Token, LexError>>,
    ) -> Vec<Result<(Token, &str), (LexError, &str)>> {
        let mut pos: usize = 0;
        tokens
            .into_iter()
            .map(|res| match res {
                Ok(token) => {
                    let token_len: usize = token.len.into();
                    let new_pos = pos + token_len;
                    let text = &input[pos..new_pos];
                    pos = new_pos;
                    Ok((token, text))
                }
                Err(e) => {
                    let token_len: usize = e.len.into();
                    let new_pos = pos + token_len;
                    let text = &input[pos..new_pos];
                    pos = new_pos;
                    Err((e, text))
                }
            })
            .collect()
    }

    fn check(input: &str) {
        let tokens = get_text(input, tokenize(input));
        insta::assert_debug_snapshot!(tokens);
    }

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
        assert_eq!(lexer.peek(), '[');
        lexer.bump().unwrap();
        assert_eq!(lexer.peek(), ']');
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

    #[test]
    fn first_token_test() {
        insta::assert_debug_snapshot!(first_token("hello"))
    }

    #[test]
    fn empty() {
        check("")
    }

    #[test]
    fn idents() {
        check(
            "hello
   world yes
              ",
        )
    }

    #[test]
    fn numbers() {
        check(
            "3 3.0
  3.1416    314.16e-2
      3.1416E1  0xff",
        )
    }

    #[test]
    fn strings() {
        check(
            r#"
"str
"
"this is a string"

'
another string
'
              "#,
        )
    }

    #[test]
    fn multiline_strings() {
        check(
            r#"
[[
bracketd string134 asd
asdf
]]
            "#,
        )
    }

    #[test]
    fn multiline_equals() {
        check(
            r#"
[=====[
bracketd string134 asd

]=====]
            "#,
        )
    }

    #[test]
    fn math() {
        check(
            "
5 + 1
12 * 3
34 - 23
2/1
              ",
        )
    }

    #[test]
    fn operators() {
        check(
            "
!13 !true
            ",
        )
    }

    #[test]
    fn keywords() {
        check(
            r#"
-- taken from the lua reference
 and       break     do        else      elseif    end
 false     for       function  goto      if        in
 local     nil       not       or        repeat    return
 then      true      until     while
 local var = "asdf"
            "#
             )
    }
}

use std::ops::Range;

use crate::lexer::Lexer;

pub trait Lexable {
    fn nth(self, p: &Lexer<'_>, n: u32) -> bool;

    fn peek(self, p: &Lexer<'_>) -> bool
    where
        Self: Sized,
    {
        self.nth(p, 0)
    }
}

pub trait Accept: Lexable {
    fn accept(self, l: &mut Lexer<'_>) -> bool
    where
        Self: Lexable + Sized,
    {
        if self.peek(l) {
            l.bump_raw().expect("Whatever was acceptable did not take into account eof");
            true
        } else {
            false
        }
    }

    fn accept_count(self, l: &mut Lexer<'_>) -> u32
    where
        Self: Lexable + Sized,
    {
        self.accept(l) as u32
    }

    fn bump(self, l: &mut Lexer<'_>)
    where
        Self: Lexable + Sized + Copy,
    {
        assert!(self.accept(l), "Failed to accept");
    }
}

impl Lexable for char {
    fn nth(self, l: &Lexer<'_>, n: u32) -> bool {
        !l.is_eof() && l.nth(n) == self
    }
}
impl Accept for char {}

impl<T: Fn(char) -> bool> Lexable for T {
    fn nth(self, l: &Lexer<'_>, n: u32) -> bool {
        !l.is_eof() && self(l.nth(n))
    }
}
impl<T: Fn(char) -> bool> Accept for T {}

impl Lexable for Range<char> {
    fn nth(self, l: &Lexer<'_>, n: u32) -> bool {
        !l.is_eof() && self.contains(&l.nth(n))
    }
}
impl Accept for Range<char> {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Seq<T: Copy>(pub T);

macro_rules! tuple_seq {
    { $( ($( $n:tt $name:ident )+) ),+ $(,)? } => {
        $(
            impl<$($name),+> Lexable for Seq<($($name),+)>
            where
                $($name: Lexable + Copy,)+
                {
                    fn nth(self, l: &Lexer<'_>, n: u32) -> bool {
                        $( self.0.$n.nth(l, $n + n) )&&+
                    }
                }

            impl<$($name),+> Accept for Seq<($($name),+)>
            where
                $($name: Lexable + Copy + Accept,)+
            {
                fn accept(self, l: &mut Lexer<'_>) -> bool
                    where
                        Self: Lexable + Sized + Copy,
                {
                    if self.peek(l) {
                        $( self.0.$n.accept(l); )+
                        true
                    } else {
                        false
                    }
                }
            }
        )+
    };
}

macro_rules! _seq {
    ($( $expr:expr ),* $(,)?) => {
        $crate::accept::Seq(($( $expr, )*))
    };
}
pub(crate) use _seq as seq;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct And<T: Copy>(pub T);

macro_rules! tuple_and {
    { $( ($( $n:tt $name:ident )+) ),+ $(,)? } => {
        $(
            impl<$($name),+> Lexable for And<($($name),+)>
            where
                $($name: Lexable + Copy,)+
                {
                    fn nth(self, l: &Lexer<'_>, n: u32) -> bool {
                        $( self.0.$n.nth(l, n) )&&+
                    }
                }

            impl<$($name),+> Accept for And<($($name),+)>
            where
                $($name: Lexable + Copy + Accept,)+
            {
            }
        )+
    };
}

macro_rules! _and {
    ($( $expr:expr ),* $(,)?) => {
        And(($( $expr, )*))
    };
}
pub(crate) use _and as and;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Or<T: Copy>(pub T);

macro_rules! tuple_or {
    { $( ($( $n:tt $name:ident )+) ),+ $(,)? } => {
        $(
            impl<$($name),+> Lexable for Or<($($name),+)>
            where
                $($name: Lexable + Copy,)+
                {
                    fn nth(self, l: &Lexer<'_>, n: u32) -> bool {
                        $( self.0.$n.nth(l, n) )||+
                    }
                }

            impl<$($name),+> Accept for Or<($($name),+)>
            where
                $($name: Lexable + Copy + Accept,)+
            {
                fn accept(self, l: &mut Lexer<'_>) -> bool
                    where
                        Self: Lexable + Sized,
                    {
                        $( self.0.$n.accept(l) )||+
                    }
            }
        )+
    };
}

macro_rules! _or {
    ($( $expr:expr ),* $(,)?) => {
        $crate::accept::Or(($( $expr, )*))
    };
}
pub(crate) use _or as or;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Not<T>(pub T);

impl<T: Lexable> Lexable for Not<T> {
    fn nth(self, p: &Lexer<'_>, n: u32) -> bool
    where
        Self: Sized,
    {
        !p.is_eof() && !T::nth(self.0, p, n)
    }
}

macro_rules! _not {
    ($expr:expr) => {
        $crate::accept::Not($expr)
    };
}
pub(crate) use _not as not;

impl<T: Lexable + Copy> Accept for Not<T> {}

macro_rules! tuple_impls {
    ($( $stuff:tt )*) => {
        tuple_seq!{ $($stuff)* }
        tuple_and!{ $($stuff)* }
        tuple_or!{ $($stuff)* }
    }
}

#[derive(Clone, Copy)]
pub struct Any;

impl Lexable for Any {
    fn nth(self, l: &Lexer<'_>, n: u32) -> bool {
        l.chars().nth(n as usize).is_some()
    }
}

impl Accept for Any {}

tuple_impls! {
    (0 TO 1 T1),
    (0 TO 1 T1 2 T2),
    (0 TO 1 T1 2 T2 3 T3),
    (0 TO 1 T1 2 T2 3 T3 4 T4),
    (0 TO 1 T1 2 T2 3 T3 4 T4 5 T5),
    (0 TO 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6),
    (0 TO 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7),
    (0 TO 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8),
    (0 TO 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9),
    (0 TO 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10),
    (0 TO 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11),
    (0 TO 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct While<T>(pub T);

impl<T: Lexable> Lexable for While<T> {
    fn nth(self, l: &Lexer<'_>, n: u32) -> bool {
        self.0.nth(l, n)
    }
}

impl<T: Copy + Lexable + Accept> Accept for While<T> {
    fn accept(self, l: &mut Lexer<'_>) -> bool
    where
        Self: Lexable + Sized + Copy,
    {
        if !self.peek(l) {
            return false;
        }

        while self.0.accept(l) {}

        true
    }

    fn accept_count(self, l: &mut Lexer<'_>) -> u32
    where
        Self: Lexable + Sized,
    {
        let mut count = 0;
        while self.0.accept(l) {
            count += 1;
        }
        count
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Until<T>(pub T);

impl<T: Lexable> Lexable for Until<T> {
    fn nth(self, l: &Lexer<'_>, n: u32) -> bool {
        While(Not(self.0)).nth(l, n)
    }
}

impl<T: Copy + Lexable + Accept> Accept for Until<T> {
    fn accept(self, l: &mut Lexer<'_>) -> bool
    where
        Self: Lexable + Sized + Copy,
    {
        While(Not(self.0)).accept(l)
    }

    fn accept_count(self, l: &mut Lexer<'_>) -> u32
    where
        Self: Lexable + Sized,
    {
        While(Not(self.0)).accept_count(l)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Repeat<T>(pub T, pub u32);

impl<T: Lexable + Copy> Lexable for Repeat<T> {
    fn nth(self, p: &Lexer<'_>, n: u32) -> bool {
        (0..self.1).all(|i| self.0.nth(p, n + i))
    }
}

impl<T: Copy + Lexable + Accept> Accept for Repeat<T> {
    fn accept(self, l: &mut Lexer<'_>) -> bool
    where
        Self: Lexable + Sized,
    {
        if self.peek(l) {
            for _ in 0..self.1 {
                self.0.accept(l);
            }
            true
        } else {
            false
        }
    }
}

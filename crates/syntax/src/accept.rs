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
        Self: Lexable + Sized + Copy,
    {
        if self.peek(l) {
            l.bump_raw()
                .expect("Whatever was acceptable did not take into account eof");
            true
        } else {
            false
        }
    }

    fn bump(self, l: &mut Lexer<'_>)
    where
        Self: Lexable + Sized + Copy,
    {
        assert!(self.accept(l), "Failed to accept");
    }

    fn accept_while(self, l: &mut Lexer<'_>)
    where
        Self: Lexable + Sized + Copy,
    {
        while self.accept(l) {}
    }

    fn accept_until(self, l: &mut Lexer<'_>)
    where
        Self: Lexable + Sized + Copy,
    {
        while (Not(self)).accept(l) {}
    }

    fn accept_while_count(self, l: &mut Lexer<'_>) -> u32
    where
        Self: Lexable + Sized + Copy,
    {
        let mut count = 0;
        while self.accept(l) {
            count += 1;
        }
        count
    }

    fn accept_repeat(self, l: &mut Lexer<'_>, repeat: u32) -> bool
    where
        Self: Lexable + Sized + Copy,
    {
        if (0..repeat).all(|i| self.nth(l, i)) {
            for _ in 0..repeat {
                l.bump_raw().unwrap();
            }
            true
        } else {
            false
        }
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

#[derive(Copy, Clone)]
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

#[derive(Copy, Clone)]
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

#[derive(Copy, Clone)]
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

#[derive(Copy, Clone)]
pub struct Not<T: Copy>(pub T);

impl<T: Lexable + Copy> Lexable for Not<T> {
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

//! Some structs that can be used to combined different things that can be accepted;

use super::{Lexable, Accept, Source};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Seq<T>(pub T);

macro_rules! tuple_seq {
    { $( ($( $n:tt $name:ident )+) ),+ $(,)? } => {
        $(
            impl<A, $($name),+> Lexable<A> for Seq<($($name),+)>
            where
                A: Source,
                $($name: Lexable<A> + Copy,)+
                {
                    fn nth(self, l: &A, n: u32) -> bool {
                        $( self.0.$n.nth(l, $n + n) )&&+
                    }
                }

            impl<A, $($name),+> Accept<A> for Seq<($($name),+)>
            where
                A: Source,
                $($name: Lexable<A> + Copy + Accept<A>,)+
            {
                fn accept(self, l: &mut A) -> bool
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

#[macro_export]
macro_rules! _seq {
    ($( $expr:expr ),* $(,)?) => {
        $crate::combinators::Seq(($( $expr, )*))
    };
}
pub use _seq as seq;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct And<T: Copy>(pub T);

macro_rules! tuple_and {
    { $( ($( $n:tt $name:ident )+) ),+ $(,)? } => {
        $(
            impl<A, $($name),+> Lexable<A> for And<($($name),+)>
            where
                A: Source,
                $($name: Lexable<A> + Copy,)+
                {
                    fn nth(self, l: &A, n: u32) -> bool {
                        $( self.0.$n.nth(l, n) )&&+
                    }
                }

            impl<A, $($name),+> Accept<A> for And<($($name),+)>
            where
                A: Source,
                $($name: Lexable<A> + Copy + Accept<A>,)+
            {
            }
        )+
    };
}

#[macro_export]
macro_rules! _and {
    ($( $expr:expr ),* $(,)?) => {
        $crate::And(($( $expr, )*))
    };
}
pub use _and as and;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Or<T: Copy>(pub T);

macro_rules! tuple_or {
    { $( ($( $n:tt $name:ident )+) ),+ $(,)? } => {
        $(
            impl<A, $($name),+> Lexable<A> for Or<($($name),+)>
            where
                A: Source,
                $($name: Lexable<A> + Copy,)+
                {
                    fn nth(self, l: &A, n: u32) -> bool {
                        $( self.0.$n.nth(l, n) )||+
                    }
                }

            impl<A, $($name),+> Accept<A> for Or<($($name),+)>
            where
                A: Source,
                $($name: Lexable<A> + Copy + Accept<A>,)+
            {
                fn accept(self, l: &mut A) -> bool
                    {
                        $( self.0.$n.accept(l) )||+
                    }
            }
        )+
    };
}

#[macro_export]
macro_rules! _or {
    ($( $expr:expr ),* $(,)?) => {
        $crate::combinators::Or(($( $expr, )*))
    };
}
pub use _or as or;

macro_rules! tuple_impls {
    ($( $stuff:tt )*) => {
        tuple_seq!{ $($stuff)* }
        tuple_and!{ $($stuff)* }
        tuple_or!{ $($stuff)* }
    }
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
    (0 TO 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9),
    (0 TO 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10),
    (0 TO 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11),
    (0 TO 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Not<T>(pub T);

impl<T, A> Lexable<A> for Not<T>
where
    A: Source,
    T: Lexable<A>,
{
    fn nth(self, p: &A, n: u32) -> bool
    where
        Self: Sized,
    {
        !p.is_eof() && !self.0.nth(p, n)
    }
}

impl<T, A> Accept<A> for Not<T>
where
    A: Source,
    T: Lexable<A> + Accept<A>,
{
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct While<T>(pub T);

impl<A, T> Lexable<A> for While<T>
where
    A: Source,
    T: Lexable<A>,
{
    fn nth(self, l: &A, n: u32) -> bool {
        self.0.nth(l, n)
    }
}

impl<A, T> Accept<A> for While<T>
where
    A: Source,
    T: Copy + Lexable<A> + Accept<A>,
{
    fn accept(self, l: &mut A) -> bool {
        if l.is_eof() && !self.peek(l) {
            return false;
        }

        while !l.is_eof() && self.0.accept(l) {}

        true
    }

    fn accept_count(self, l: &mut A) -> u32 {
        let mut count = 0;
        while self.0.accept(l) {
            count += 1;
        }
        count
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Until<T>(pub T);

impl<A, T> Lexable<A> for Until<T>
where
    A: Source,
    T: Lexable<A>,
{
    fn nth(self, l: &A, n: u32) -> bool {
        While(Not(self.0)).nth(l, n)
    }
}

impl<A, T> Accept<A> for Until<T>
where
    A: Source,
    T: Copy + Lexable<A> + Accept<A>,
{
    fn accept(self, l: &mut A) -> bool {
        While(Not(self.0)).accept(l)
    }

    fn accept_count(self, l: &mut A) -> u32 {
        While(Not(self.0)).accept_count(l)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Repeat<T>(pub T, pub u32);

impl<A, T> Lexable<A> for Repeat<T>
where
    A: Source,
    T: Lexable<A> + Copy,
{
    fn nth(self, p: &A, n: u32) -> bool {
        (0..self.1).all(|i| self.0.nth(p, n + i))
    }
}

impl<A, T> Accept<A> for Repeat<T>
where
    A: Source,
    T: Copy + Lexable<A> + Accept<A>,
{
    fn accept(self, l: &mut A) -> bool {
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

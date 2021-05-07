use std::ops::Range;

pub trait Advancer {
    type Item;

    // const EOF_PLACEHOLDER: Self::Item;

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

impl<A: Advancer> private::Sealed for A {}

pub trait Acceptor: Advancer + private::Sealed {
    fn at<T: Lexable<Self>>(&self, t: T) -> bool
    where
        Self: Sized,
    {
        t.peek(self)
    }

    fn bump<T: Accept<Self>>(&mut self, t: T)
    where
        Self: Sized,
    {
        t.bump(self)
    }

    fn bump_then<T: Accept<Self>>(&mut self, t: T) -> Self::Item
    where
        Self: Sized,
    {
        t.bump(self);
        self.nth(0)
    }

    fn accept<T: Accept<Self>>(&mut self, t: T) -> bool
    where
        Self: Sized,
    {
        t.accept(self)
    }

    fn accept_count<T: Accept<Self>>(&mut self, t: T) -> u32
    where
        Self: Sized,
    {
        t.accept_count(self)
    }
}

impl<A: Advancer> Acceptor for A {}

pub trait Lexable<A: Advancer> {
    fn nth(self, p: &A, n: u32) -> bool
    where
        Self: Sized;

    fn peek(self, p: &A) -> bool
    where
        Self: Sized,
    {
        self.nth(p, 0)
    }
}

pub trait Accept<A>: Lexable<A>
where
    A: Advancer,
{
    fn accept(self, l: &mut A) -> bool
    where
        Self: Sized,
    {
        if self.peek(l) {
            l.bump_expect("Whatever was acceptable did not take into account eof");
            true
        } else {
            false
        }
    }

    fn accept_count(self, l: &mut A) -> u32
    where
        Self: Sized,
    {
        self.accept(l) as u32
    }

    fn bump(self, l: &mut A)
    where
        Self: Sized,
    {
        assert!(self.accept(l), "Failed to accept");
    }
}

impl<A> Lexable<A> for char
where
    A: Advancer<Item = char>,
{
    fn nth(self, l: &A, n: u32) -> bool {
        l.nth(n) == self
    }
}
impl<A> Accept<A> for char where A: Advancer<Item = char> {}

impl<T, A, F> Lexable<A> for F
where
    A: Advancer<Item = T>,
    F: Fn(T) -> bool,
{
    fn nth(self, l: &A, n: u32) -> bool {
        self(l.nth(n))
    }
}
impl<T, A, F> Accept<A> for F
where
    A: Advancer<Item = T>,
    F: Fn(T) -> bool,
{
}

impl<A> Lexable<A> for Range<char>
where
    A: Advancer<Item = char>,
{
    fn nth(self, l: &A, n: u32) -> bool {
        self.contains(&l.nth(n))
    }
}
impl<A> Accept<A> for Range<char> where A: Advancer<Item = char> {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Seq<T>(pub T);

macro_rules! tuple_seq {
    { $( ($( $n:tt $name:ident )+) ),+ $(,)? } => {
        $(
            impl<A, $($name),+> Lexable<A> for Seq<($($name),+)>
            where
                A: Advancer,
                $($name: Lexable<A> + Copy,)+
                {
                    fn nth(self, l: &A, n: u32) -> bool {
                        $( self.0.$n.nth(l, $n + n) )&&+
                    }
                }

            impl<A, $($name),+> Accept<A> for Seq<($($name),+)>
            where
                A: Advancer,
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
macro_rules! seq {
    ($( $expr:expr ),* $(,)?) => {
        $crate::Seq(($( $expr, )*))
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct And<T: Copy>(pub T);

macro_rules! tuple_and {
    { $( ($( $n:tt $name:ident )+) ),+ $(,)? } => {
        $(
            impl<A, $($name),+> Lexable<A> for And<($($name),+)>
            where
                A: Advancer,
                $($name: Lexable<A> + Copy,)+
                {
                    fn nth(self, l: &A, n: u32) -> bool {
                        $( self.0.$n.nth(l, n) )&&+
                    }
                }

            impl<A, $($name),+> Accept<A> for And<($($name),+)>
            where
                A: Advancer,
                $($name: Lexable<A> + Copy + Accept<A>,)+
            {
            }
        )+
    };
}

#[macro_export]
macro_rules! and {
    ($( $expr:expr ),* $(,)?) => {
        $crate::And(($( $expr, )*))
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Or<T: Copy>(pub T);

macro_rules! tuple_or {
    { $( ($( $n:tt $name:ident )+) ),+ $(,)? } => {
        $(
            impl<A, $($name),+> Lexable<A> for Or<($($name),+)>
            where
                A: Advancer,
                $($name: Lexable<A> + Copy,)+
                {
                    fn nth(self, l: &A, n: u32) -> bool {
                        $( self.0.$n.nth(l, n) )||+
                    }
                }

            impl<A, $($name),+> Accept<A> for Or<($($name),+)>
            where
                A: Advancer,
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
macro_rules! or {
    ($( $expr:expr ),* $(,)?) => {
        $crate::Or(($( $expr, )*))
    };
}

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
    A: Advancer,
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
    A: Advancer,
    T: Lexable<A> + Accept<A>,
{
}

#[macro_export]
macro_rules! not {
    ($expr:expr) => {
        $crate::Not($expr)
    };
}

#[derive(Clone, Copy)]
pub struct Any;

impl<A> Lexable<A> for Any
where
    A: Advancer,
{
    fn nth(self, l: &A, n: u32) -> bool {
        !l.nth_is_eof(n)
    }
}

impl<A> Accept<A> for Any where A: Advancer {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct While<T>(pub T);

impl<A, T> Lexable<A> for While<T>
where
    A: Advancer,
    T: Lexable<A>,
{
    fn nth(self, l: &A, n: u32) -> bool {
        self.0.nth(l, n)
    }
}

impl<A, T> Accept<A> for While<T>
where
    A: Advancer,
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
    A: Advancer,
    T: Lexable<A>,
{
    fn nth(self, l: &A, n: u32) -> bool {
        While(Not(self.0)).nth(l, n)
    }
}

impl<A, T> Accept<A> for Until<T>
where
    A: Advancer,
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
    A: Advancer,
    T: Lexable<A> + Copy,
{
    fn nth(self, p: &A, n: u32) -> bool {
        (0..self.1).all(|i| self.0.nth(p, n + i))
    }
}

impl<A, T> Accept<A> for Repeat<T>
where
    A: Advancer,
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

mod private {
    pub trait Sealed {}
}

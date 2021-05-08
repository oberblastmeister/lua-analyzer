pub mod combinators;

use std::ops::Range;

use crate::source::Source;

/// An acceptor is something that can accept things from a `Source`. This trait is automatically
/// implemented for all `Source`s.
pub trait Acceptor: Source + private::Sealed {
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

impl<S: Source> private::Sealed for S {}

impl<A: Source> Acceptor for A {}

/// Something that can be lexed. This is implemented for most types already but can also be
/// implemented for custom types, for example rowan `SyntaxKind`.
pub trait Lexable<A: Source> {
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
    A: Source,
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
    A: Source<Item = char>,
{
    fn nth(self, l: &A, n: u32) -> bool {
        l.nth(n) == self
    }
}
impl<A> Accept<A> for char where A: Source<Item = char> {}

impl<T, A, F> Lexable<A> for F
where
    A: Source<Item = T>,
    F: Fn(T) -> bool,
{
    fn nth(self, l: &A, n: u32) -> bool {
        self(l.nth(n))
    }
}
impl<T, A, F> Accept<A> for F
where
    A: Source<Item = T>,
    F: Fn(T) -> bool,
{
}

impl<A> Lexable<A> for Range<char>
where
    A: Source<Item = char>,
{
    fn nth(self, l: &A, n: u32) -> bool {
        self.contains(&l.nth(n))
    }
}
impl<A> Accept<A> for Range<char> where A: Source<Item = char> {}

#[derive(Clone, Copy)]
pub struct Any;

impl<A> Lexable<A> for Any
where
    A: Source,
{
    fn nth(self, l: &A, n: u32) -> bool {
        !l.nth_is_eof(n)
    }
}

impl<A> Accept<A> for Any where A: Source {}

mod private {
    pub trait Sealed {}
}

use crate::{SyntaxError, SyntaxKind};

/// A Result that is always ok and can have an optional error
pub struct OkResult<T, E>(T, Option<E>);

impl<T, E> OkResult<T, E> {
    pub fn new(t: T, err: Option<E>) -> OkResult<T, E> {
        OkResult(t, err)
    }

    pub fn just(t: T) -> OkResult<T, E> {
        OkResult(t, None)
    }

    pub fn map<U, F>(self, f: F) -> OkResult<U, E>
    where
        F: FnOnce(T) -> U,
    {
        OkResult(f(self.0), self.1)
    }

    pub fn map_err<F, U>(self, f: F) -> OkResult<T, U>
    where
        F: FnOnce(E) -> U,
    {
        OkResult(self.0, self.1.map(f))
    }

    pub fn inner(self) -> (T, Option<E>) {
        (self.0, self.1)
    }

    pub fn ok(self) -> T {
        self.0
    }

    pub fn ok_ref(&self) -> &T {
        &self.0
    }

    pub fn err(self) -> Option<E> {
        self.1
    }

    pub fn has_err(&self) -> bool {
        self.1.is_some()
    }
}

impl<E> OkResult<(), E> {
    pub fn unit(err: Option<E>) -> OkResult<(), E> {
        OkResult((), err)
    }

    pub fn empty() -> OkResult<(), E> {
        OkResult((), None)
    }
}

// impl<T, E> ResultInfo<T, E> {
//     pub fn just(kind: T) -> ResultInfo<SyntaxKind, E> {
//         ResultInfo(kind, None)
//     }
// }

pub type LexResult<T> = OkResult<T, String>;

pub type SyntaxResult<T> = OkResult<T, SyntaxError>;

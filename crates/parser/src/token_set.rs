//! A bit-set of `SyntaxKind`s.

use crate::SyntaxKind;

#[macro_export]
macro_rules! TS {
    [] => {
        TokenSet::EMPTY
    };
    [$( $stuff:tt ),* $(,)?] => {
        TokenSet::new(&[$( T![$stuff], )*])
    };
}

/// A bit-set of `SyntaxKind`s
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TokenSet(u128);

impl TokenSet {
    pub(crate) const EMPTY: TokenSet = TokenSet(0);

    pub(crate) const fn new(kinds: &[SyntaxKind]) -> TokenSet {
        let mut res = 0u128;
        let mut i = 0;
        while i < kinds.len() {
            res |= mask(kinds[i]);
            i += 1
        }
        TokenSet(res)
    }

    pub(crate) const fn union(self, other: TokenSet) -> TokenSet {
        TokenSet(self.0 | other.0)
    }

    pub(crate) const fn contains(&self, kind: SyntaxKind) -> bool {
        self.0 & mask(kind) != 0
    }
}

const fn mask(kind: SyntaxKind) -> u128 {
    1u128 << (kind as usize)
}

#[test]
fn token_set_works_for_tokens() {
    let ts = TS![eof, +, -];
    assert!(ts.contains(T![eof]));
    assert!(ts.contains(T![+]));
    assert!(ts.contains(T![-]));
    assert!(!ts.contains(T![comment]));
}

#[test]
fn comma_not_in_literal() {
    let ts_m = TS![true, false, number, str, nil];
    let ts = TokenSet::new(&[T![true], T![false], T![number], T![str], T![nil]]);
    assert_eq!(ts_m, ts);
    assert!(!ts.contains(T![,]));
}

#[test]
fn correct_empty() {
    let ts = TS![];
    assert_eq!(TS![], TokenSet::EMPTY);
}

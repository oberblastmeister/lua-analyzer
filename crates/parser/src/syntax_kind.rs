#[macro_use]
mod generated;

pub use generated::SyntaxKind;

use accept::{Accept, source::Source, Lexable};

impl From<u16> for SyntaxKind {
    fn from(d: u16) -> SyntaxKind {
        assert!(d <= (SyntaxKind::__LAST as u16));
        unsafe { std::mem::transmute::<u16, SyntaxKind>(d) }
    }
}

impl From<SyntaxKind> for u16 {
    fn from(k: SyntaxKind) -> u16 {
        k as u16
    }
}

impl<A> Lexable<A> for SyntaxKind
where
    A: Source<Item = SyntaxKind>,
{
    fn nth(self, p: &A, n: u32) -> bool {
        p.nth(n) == self
    }
}

impl<A> Accept<A> for SyntaxKind where A: Source<Item = SyntaxKind> {}

/// for the bit-set to work
#[test]
fn syntax_kinds_in_bounds() {
    assert!(T![__] as u16 == 0);
    eprintln!(
        "Last syntaxkind id
              = {:?}",
        SyntaxKind::__LAST as u16
    );
    assert!((SyntaxKind::__LAST as u16) < 128);
}

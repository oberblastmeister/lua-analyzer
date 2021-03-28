mod expr;

use crate::{parser::Parser, SyntaxKind::*, TokenSource};

pub(crate) fn root<T: TokenSource>(p: &mut Parser<T>) {
    let m = p.start();
    m.complete(p, Program);
}

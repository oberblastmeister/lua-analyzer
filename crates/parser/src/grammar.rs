mod expressions;
mod statements;

use crate::{parser::Parser, SyntaxKind::*, TokenSource};
use expressions::expr;
use statements::stmt;

pub(crate) fn root<T: TokenSource>(p: &mut Parser<T>) {
    let m = p.start();
    while !p.at(T![eof]) {
        stmt(p);
    }
    m.complete(p, Program);
}

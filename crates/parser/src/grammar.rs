mod expressions;
mod statements;

use crate::{parser::Parser, SyntaxKind::*, TokenSource};
use expressions::expr_single;
use statements::stmt;

pub(crate) fn root(p: &mut Parser) {
    let m = p.start();
    while !p.at(T![eof]) {
        stmt(p);
    }
    m.complete(p, Program);
}

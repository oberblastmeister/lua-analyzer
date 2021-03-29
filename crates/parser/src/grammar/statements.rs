use super::expr;
use crate::{parser::Parser, TokenSource};

pub(super) fn stmt(p: &mut Parser) {
    match p.current() {
        T![local] => assignment(p),
        it => panic!("Not implemented for {:?}", it),
    }
}

fn assignment(p: &mut Parser) {
    p.bump(T![local]);
    p.bump(T![ident]);
    p.bump(T![=]);
    expr(p);
}

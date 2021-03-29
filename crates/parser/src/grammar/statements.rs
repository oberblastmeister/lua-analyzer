use super::expr;
use crate::{parser::Parser, TokenSource};

pub(super) fn stmt<T: TokenSource>(p: &mut Parser<T>) {
    match p.current() {
        T![local] => assignment(p),
        it => panic!("Not implemented for {:?}", it),
    }
}

fn assignment<T: TokenSource>(p: &mut Parser<T>) {
    p.bump(T![local]);
    p.bump(T![ident]);
    p.bump(T![=]);
    expr(p);
}

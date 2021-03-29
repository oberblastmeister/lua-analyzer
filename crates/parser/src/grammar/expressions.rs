use crate::{parser::Parser, TokenSource};

const LOWEST: u8 = 0;

pub(super) fn expr<T: TokenSource>(p: &mut Parser<T>) {
    expr_bp(p, LOWEST);
}

fn expr_bp<T: TokenSource>(p: &mut Parser<T>, bp: u8) {
    lhs(p);
}

fn lhs<T: TokenSource>(p: &mut Parser<T>) {
    match p.current() {
        T![number] => p.bump_any(),
        T![str] => p.bump_any(),
        _ => todo!(),
    }
}

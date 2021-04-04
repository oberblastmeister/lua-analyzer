use super::expr;
use crate::SyntaxKind::*;
use crate::{
    parser::{MarkerComplete, Parser},
    TokenSource,
};

pub(super) fn stmt(p: &mut Parser) -> Option<MarkerComplete> {
    let m = p.start();
    match p.current() {
        T![local] => {
            assignment(p);
        }
        it => panic!("Not implemented for {:?}", it),
    }
    Some(m.complete(p, Stmt))
}

fn assignment(p: &mut Parser) -> MarkerComplete {
    let m = p.start();
    p.bump(T![local]);
    name(p);
    p.bump(T![=]);
    expr(p);
    m.complete(p, AssignStmt)
}

fn name(p: &mut Parser) -> MarkerComplete {
    let m = p.start();
    p.bump(T![ident]);
    m.complete(p, Name)
}

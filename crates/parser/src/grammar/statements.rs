use super::{expr, expressions::call_expr};
use crate::{parser::Marker, SyntaxKind::*};
use crate::{
    parser::{MarkerComplete, Parser},
    TokenSource,
};

macro_rules! no_ret {
    ($expr:expr) => {{
        $expr;
    }};
}

pub(super) fn stmt(p: &mut Parser) -> Option<MarkerComplete> {
    let m = p.start();
    let peek = p.nth(1);
    match p.current() {
        T![local] => no_ret!(local_stmt(p)),
        T![function] => no_ret!(function_def_stmt(p, false)),
        T![return] => no_ret!(return_stmt(p)),
        T![ident] if peek == T![=] => no_ret!(assignment(p, false)),
        T![ident] if peek == T!['('] => no_ret!(call_expr_stmt(p)),
        _ => p.err_recover("Expected a statement"),
    }
    Some(m.complete(p, Stmt))
}

fn local_stmt(p: &mut Parser) -> Option<MarkerComplete> {
    Some(match p.nth(1) {
        T![function] => function_def_stmt(p, true),
        T![ident] => assignment(p, true),
        _ => {
            p.err_recover("Expected a local function or local assignment");
            return None;
        }
    })
}

fn call_expr_stmt(p: &mut Parser) -> MarkerComplete {
    let m = p.start();
    call_expr(p);
    m.complete(p, CallExpr)
}

fn function_def_stmt(p: &mut Parser, is_local: bool) -> MarkerComplete {
    let m = p.start();

    if is_local {
        p.bump(T![local]);
    }

    p.expect(T![function]);
    name(p);
    param_list(p);
    body(p);
    p.expect(T![end]);
    m.complete(p, FunctionDefStmt)
}

fn return_stmt(p: &mut Parser) -> MarkerComplete {
    let m = p.start();
    p.bump(T![return]);
    expr(p);
    m.complete(p, ReturnStmt)
}

fn param_list(p: &mut Parser) -> MarkerComplete {
    let m = p.start();
    p.expect(T!['(']);
    p.expect(T![')']);
    m.complete(p, Paramlist)
}

fn assignment(p: &mut Parser, is_local: bool) -> MarkerComplete {
    let m = p.start();
    if is_local {
        p.bump(T![local]);
    }
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

fn body(p: &mut Parser) -> MarkerComplete {
    let m = p.start();
    while !p.at(T![eof]) && !p.at(T![end]) {
        stmt(p);
    }
    m.complete(p, Body)
}

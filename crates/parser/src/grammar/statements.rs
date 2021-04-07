use super::{expr_single, expressions::call_expr, expressions::expr};
use crate::{parser::Marker, SyntaxKind::*, TokenSet};
use crate::{
    parser::{MarkerComplete, Parser},
    TokenSource,
};

pub(super) fn stmt(p: &mut Parser) -> Option<MarkerComplete> {
    let peek = p.nth(1);
    Some(match p.current() {
        T![local] => local_stmt(p)?,
        T![function] => function_def_stmt(p, false),
        T![return] => return_stmt(p),
        T![ident] if peek == T![=] => assignment(p, false),
        T![ident] if peek == T!['('] => call_expr_stmt(p),
        T!['('] => call_expr_stmt(p),
        _ => {
            p.err_recover("Expected a statement");
            return None;
        }
    })
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
    const CALL_TS: TokenSet = TokenSet::new(&[T![ident], T!['(']]);

    assert!(p.at_ts(CALL_TS));

    let m = p.start();
    expr_single(p);
    m.complete(p, CallExprStmt)
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

pub(super) fn param_list(p: &mut Parser) -> MarkerComplete {
    const END: TokenSet = TokenSet::new(&[T![')'], T![eof]]);

    let m = p.start();
    p.expect(T!['(']);
    if !p.at_ts(END) {
        multi_name(p);
    }
    p.expect(T![')']);
    m.complete(p, Paramlist)
}

fn assignment(p: &mut Parser, is_local: bool) -> MarkerComplete {
    let m = p.start();
    if is_local {
        p.bump(T![local]);
    }
    name(p);
    p.expect(T![=]);
    expr(p);
    m.complete(p, AssignStmt)
}

fn multi_name(p: &mut Parser) -> MarkerComplete {
    let m = p.start();
    name(p);
    while p.at(T![,]) {
        p.bump(T![,]);

        name(p);
    }
    m.complete(p, MultiName)
}

fn name(p: &mut Parser) -> MarkerComplete {
    let m = p.start();
    p.expect(T![ident]);
    m.complete(p, Name)
}

pub(super) fn body(p: &mut Parser) -> MarkerComplete {
    let m = p.start();
    while !p.at(T![eof]) && !p.at(T![end]) {
        stmt(p);
    }
    m.complete(p, Body)
}

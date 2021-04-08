use super::{
    expr_single,
    expressions::expr,
    expressions::{call_expr, name_ref},
};
use crate::{parser::Marker, SyntaxKind::*, TokenSet};
use crate::{
    parser::{MarkerComplete, Parser},
    TokenSource,
};

pub(super) fn stmt(p: &mut Parser) -> Option<MarkerComplete> {
    let peek = p.nth(1);
    Some(match p.current() {
        T![::] => label_stmt(p),
        T![goto] => goto_stmt(p),
        T![local] => local_stmt(p)?,
        T![function] => function_def_stmt(p, false),
        T![return] => return_stmt(p),
        T![do] => do_stmt(p),
        T![while] => while_stmt(p),
        T![for] => for_stmt(p),
        T![break] => break_stmt(p),
        T![ident] if peek == T![=] => assignment(p, false),
        T![ident] if peek == T!['('] => call_expr_stmt(p),
        T!['('] => call_expr_stmt(p),
        _ => {
            p.err_recover("Expected a statement");
            return None;
        }
    })
}

fn goto_stmt(p: &mut Parser) -> MarkerComplete {
    let m = p.start();
    p.bump(T![goto]);
    name_ref(p);
    m.complete(p, GotoStmt)
}

fn label_stmt(p: &mut Parser) -> MarkerComplete {
    assert!(p.at(T![::]));
    let m = p.start();
    label_delim(p);
    name(p);
    label_delim(p);
    m.complete(p, LabelStmt)
}

fn label_delim(p: &mut Parser) -> MarkerComplete {
    let m = p.start();
    p.expect(T![::]);
    m.complete(p, LabelDelim)
}

fn while_stmt(p: &mut Parser) -> MarkerComplete {
    let m = p.start();
    p.bump(T![while]);
    expr_single(p);
    p.expect(T![do]);
    body(p);
    p.expect(T![end]);
    m.complete(p, WhileStmt)
}

fn for_stmt(p: &mut Parser) -> MarkerComplete {
    let m = p.start();
    p.bump(T![for]);
    for_content(p);
    p.expect(T![do]);
    body(p);
    p.expect(T![end]);
    m.complete(p, ForStmt)
}

fn for_content(p: &mut Parser) -> MarkerComplete {
    let m = p.start();
    if p.at(T![ident]) && p.nth(1) == T![=] {
        numeric_for(p);
    } else {
        generic_for(p);
    }
    m.complete(p, ForContent)
}

fn generic_for(p: &mut Parser) -> MarkerComplete {
    assert!(p.at(T![ident]));
    let m = p.start();
    multi_name(p);
    p.bump(T![in]);
    expr_single(p);
    m.complete(p, GenericFor)
}

fn numeric_for(p: &mut Parser) -> MarkerComplete {
    assert!(p.at(T![ident]));
    let m = p.start();
    name(p);
    p.bump(T![=]);
    expr(p);
    m.complete(p, GenericFor)
}

fn break_stmt(p: &mut Parser) -> MarkerComplete {
    let m = p.start();
    p.bump(T![break]);
    m.complete(p, BreakStmt)
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

fn do_stmt(p: &mut Parser) -> MarkerComplete {
    let m = p.start();
    p.bump(T![do]);
    body(p);
    p.expect(T![end]);
    m.complete(p, DoStmt)
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
    multi_name(p);
    if p.accept(T![=]) {
        expr(p);
    }
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

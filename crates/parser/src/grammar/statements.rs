use super::{
    expr_single,
    name,
    name_ref,
    multi_name,
    body,
    param_list,
    expressions::expr,
};
use crate::parser::{MarkerComplete, Parser};
use crate::SyntaxKind::*;

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
        T![ident] if peek == T!['('] => expr_stmt(p),
        T![ident] => expr_stmt(p),
        T!['('] => expr_stmt(p),
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

fn expr_stmt(p: &mut Parser) -> MarkerComplete {
    let m = p.start();
    expr_single(p);
    m.complete(p, ExprStmt)
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

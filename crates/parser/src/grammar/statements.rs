use super::{
    block, expr_single,
    expressions::{expr, LITERAL},
    multi_name, name, name_ref, param_list,
};
use crate::parser::{MarkerComplete, Parser};
use crate::SyntaxKind::*;
use crate::{ParseError, TokenSet, TS};

macro_rules! none {
    ($expr:expr) => {{
        $expr;
        return None;
    }};
}

pub(super) fn stmt(p: &mut Parser) -> Option<MarkerComplete> {
    Some(match p.current() {
        T![if] => if_stmt(p),
        T![::] => label_stmt(p),
        T![goto] => goto_stmt(p),
        T![local] => local_stmt(p)?,
        T![repeat] => repeat_until_stmt(p),
        T![function] => function_def_stmt(p, false),
        T![return] => return_stmt(p),
        T![do] => do_stmt(p),
        T![while] => while_stmt(p),
        T![for] => for_stmt(p),
        T![break] => break_stmt(p),
        T![ident] => expr_stmt(p),
        T!['('] => expr_stmt(p),
        _ if p.at_ts(LITERAL) => {
            none!(p.err_recover("A literal cannot be the start of a statement"))
        }
        _ => none!(p.err_recover("Expected a statement")),
    })
}

fn if_stmt(p: &mut Parser) -> MarkerComplete {
    let m = p.start();
    p.bump(T![if]);
    expr_single(p);
    p.expect(T![then]);
    block(p);
    match p.current() {
        T![elseif] => {
            elseif_branch(p);
        }
        T![else] => {
            else_branch(p);
        }
        _ => (),
    }
    p.expect(T![end]);
    m.complete(p, IfStmt)
}

fn elseif_branch(p: &mut Parser) -> MarkerComplete {
    let m = p.start();
    p.bump(T![elseif]);
    expr_single(p);
    p.expect(T![then]);
    block(p);
    match p.current() {
        T![elseif] => {
            elseif_branch(p);
        }
        T![else] => {
            else_branch(p);
        }
        _ => (),
    };
    m.complete(p, ElseIfBranch)
}

fn else_branch(p: &mut Parser) -> MarkerComplete {
    let m = p.start();
    p.bump(T![else]);
    block(p);
    m.complete(p, ElseBranch)
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
    block(p);
    p.expect(T![end]);
    m.complete(p, WhileStmt)
}

fn for_stmt(p: &mut Parser) -> MarkerComplete {
    let m = p.start();
    p.bump(T![for]);
    for_content(p);
    p.expect(T![do]);
    block(p);
    p.expect(T![end]);
    m.complete(p, ForStmt)
}

fn for_content(p: &mut Parser) -> MarkerComplete {
    let m = p.start();
    if p.at(T![ident]) {
        if p.nth(1) == T![=] {
            numeric_for(p);
        } else {
            generic_for(p);
        }
    } else {
        p.error("Expected an identifier");
    }
    m.complete(p, ForContent)
}

fn generic_for(p: &mut Parser) -> MarkerComplete {
    assert!(p.at(T![ident]));
    let m = p.start();
    multi_name(p);
    p.expect(T![in]);
    expr_single(p);
    m.complete(p, GenericFor)
}

fn numeric_for(p: &mut Parser) -> MarkerComplete {
    assert!(p.at(T![ident]));
    let m = p.start();
    name(p);
    p.bump(T![=]);
    expr(p);
    m.complete(p, NumericFor)
}

fn repeat_until_stmt(p: &mut Parser) -> MarkerComplete {
    let m = p.start();
    p.bump(T![repeat]);
    block(p);
    p.expect(T![until]);
    expr_single(p);
    m.complete(p, RepeatUntilStmt)
}

fn break_stmt(p: &mut Parser) -> MarkerComplete {
    let m = p.start();
    p.bump(T![break]);
    m.complete(p, BreakStmt)
}

fn local_stmt(p: &mut Parser) -> Option<MarkerComplete> {
    Some(match p.nth(1) {
        T![function] => function_def_stmt(p, true),
        T![ident] => local_assign_stmt(p),
        _ => {
            // we can't do err_recover here because local is a recovery token
            // so it will just infinately loop because the parser is never advanced
            let e = p.start_error();
            p.bump_any();
            e.complete(
                p,
                ParseError::Message("Expected a local function or local assignment"),
            );
            return None;
        }
    })
}

fn expr_stmt(p: &mut Parser) -> MarkerComplete {
    let m = p.start();
    expr(p);
    if p.accept(T![=]) {
        expr(p);
        m.complete(p, AssignStmt)
    } else {
        m.complete(p, ExprStmt)
    }
}

fn function_def_stmt(p: &mut Parser, is_local: bool) -> MarkerComplete {
    let m = p.start();

    if is_local {
        p.bump(T![local]);
    }

    p.expect(T![function]);
    name(p);
    param_list(p);
    block(p);
    p.expect(T![end]);
    m.complete(p, FunctionDefStmt)
}

fn do_stmt(p: &mut Parser) -> MarkerComplete {
    let m = p.start();
    p.bump(T![do]);
    block(p);
    p.expect(T![end]);
    m.complete(p, DoStmt)
}

fn return_stmt(p: &mut Parser) -> MarkerComplete {
    let m = p.start();
    p.bump(T![return]);
    expr(p);
    m.complete(p, ReturnStmt)
}

fn local_assign_stmt(p: &mut Parser) -> MarkerComplete {
    let m = p.start();
    p.bump(T![local]);
    multi_name(p);
    if p.accept(T![=]) {
        expr(p);
    }
    m.complete(p, LocalAssignStmt)
}

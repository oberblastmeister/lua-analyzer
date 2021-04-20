use super::{
    block, expr_single,
    expressions::{expr, EXPR_FIRST, LITERAL_FIRST},
    multi_name_r, name, name_r, name_ref, name_ref_r, param_list,
};
use crate::parser::{MarkerComplete, Parser};
use crate::SyntaxKind::*;
use crate::{ParseError, TokenSet, TS};

pub(super) const STMT_RECOVERY: TokenSet = TS![function, do, ::];

pub(super) fn stmt(p: &mut Parser) {
    match p.current() {
        T![if] => {
            if_stmt(p);
        }
        T![::] => {
            label_stmt(p);
        }
        T![goto] => {
            goto_stmt(p);
        }
        T![local] => {
            local_stmt(p);
        }
        T![repeat] => {
            repeat_until_stmt(p);
        }
        T![function] => {
            function_def_stmt(p);
        }
        T![return] => {
            return_stmt(p);
        }
        T![do] => {
            do_stmt(p);
        }
        T![while] => {
            while_stmt(p);
        }
        T![for] => {
            for_stmt(p);
        }
        T![break] => {
            break_stmt(p);
        }
        T![ident] => {
            expr_stmt(p);
        }
        T!['('] => {
            expr_stmt(p);
        }
        _ if p.at_ts(LITERAL_FIRST) => {
            p.err_and_bump("A literal cannot be the start of a statement");
        }
        T![end] => {
            let e = p.start_error();
            p.bump(T![end]);
            e.complete(p, ParseError::Message("Unmatched end token"));
        }
        _ => p.err_and_bump("Expected a statement"),
    };
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
    name_r(p, STMT_RECOVERY);
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
    multi_name_r(p, STMT_RECOVERY);
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
        T![function] => local_function_def_stmt(p),
        _ => local_assign_stmt(p),
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

fn local_function_def_stmt(p: &mut Parser) -> MarkerComplete {
    let m = p.start();
    p.bump(T![local]);
    p.expect(T![function]);
    name_r(p, STMT_RECOVERY);
    param_list(p);
    block(p);
    p.expect(T![end]);
    m.complete(p, LocalFunctionDefStmt)
}

fn function_def_stmt(p: &mut Parser) -> MarkerComplete {
    let m = p.start();

    p.expect(T![function]);
    function_def_content(p);
    param_list(p);
    block(p);
    p.expect(T![end]);
    m.complete(p, FunctionDefStmt)
}

fn function_def_content(p: &mut Parser) -> MarkerComplete {
    const RECOVERY: TokenSet = TS!['('].union(STMT_RECOVERY);

    let m = p.start();
    if p.at(T![ident]) && p.nth(1) == T!['('] {
        name_r(p, TS!['('].union(RECOVERY));
    } else {
        let m = function_name_index(p).precede(p);
        if p.at(T![:]) {
            p.bump(T![:]);
            name_r(p, RECOVERY);
            m.complete(p, FunctionMethod);
        } else {
            name_r(p, RECOVERY);
            m.complete(p, FunctionStatic);
        }
    }
    m.complete(p, FunctionDefContent)
}

fn function_name_index(p: &mut Parser) -> MarkerComplete {
    const RECOVERY: TokenSet = TS![., :].union(STMT_RECOVERY);

    let m = p.start();

    name_ref_r(p, RECOVERY);

    loop {
        if p.at(T![:]) {
            break;
        }

        if p.at(T![.]) {
            p.bump(T![.]);
        } else {
            break;
        }

        if p.nth(1) == T!['('] {
            break;
        }

        name_ref_r(p, RECOVERY)
    }

    m.complete(p, FunctionNameIndex)
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
    if p.at_ts(EXPR_FIRST) {
        expr(p);
    }
    m.complete(p, ReturnStmt)
}

fn local_assign_stmt(p: &mut Parser) -> MarkerComplete {
    let m = p.start();
    p.bump(T![local]);
    multi_name_r(p, STMT_RECOVERY);
    if p.accept(T![=]) {
        expr(p);
    }
    m.complete(p, LocalAssignStmt)
}

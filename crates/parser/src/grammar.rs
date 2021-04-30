mod expressions;
mod statements;

use crate::{
    parser::{MarkerComplete, MarkerRegular, Parser},
    SyntaxKind, TokenSet, TS,
};
use expressions::expr_single;
pub use expressions::LuaOp;
use statements::{stmt, STMT_RECOVERY};

pub(crate) fn root(p: &mut Parser) {
    let m = p.start();
    while !p.at(T![eof]) {
        stmt(p);
    }
    m.complete(p, N![SourceFile]);
}

fn multi_name(p: &mut Parser) -> MarkerComplete {
    let m = p.start();
    name(p);
    while p.at(T![,]) {
        p.bump(T![,]);

        name(p);
    }
    m.complete(p, N![MultiName])
}

fn multi_name_r(p: &mut Parser, recovery: TokenSet, vararg: bool) {
    let recovery = recovery.union(TokenSet::new(&[T![,]]));

    let m = p.start();

    if vararg && p.at(T![...]) {
        m.complete(p, N![MultiName]);
        return;
    }

    name_r(p, recovery);
    while p.at(T![,]) {
        p.bump(T![,]);

        if vararg && p.at(T![...]) {
            break;
        }

        name_r(p, recovery);
    }
    m.complete(p, N![MultiName]);
}

fn name_r(p: &mut Parser, recovery: TokenSet) -> Option<MarkerComplete> {
    if p.at(T![ident]) {
        Some(name_unchecked(p))
    } else {
        p.err_recover("expected a name", recovery);
        None
    }
}

fn name_unchecked(p: &mut Parser) -> MarkerComplete {
    let m = p.start();
    p.bump(T![ident]);
    m.complete(p, N![Name])
}

fn name_ref_unchecked(p: &mut Parser) -> MarkerComplete {
    let m = p.start();
    p.bump(T![ident]);
    m.complete(p, N![NameRef])
}

fn name_ref_r(p: &mut Parser, recovery: TokenSet) {
    if p.at(T![ident]) {
        name_ref_unchecked(p);
    } else {
        p.err_recover("expected a name referencer", recovery);
    }
}

fn name_ref(p: &mut Parser) -> MarkerComplete {
    let m = p.start();
    p.expect(T![ident]);
    m.complete(p, N![NameRef])
}

fn name(p: &mut Parser) {
    if p.at(T![ident]) {
        let m = p.start();
        p.bump(T![ident]);
        m.complete(p, N![Name]);
    } else {
        p.error("expected a name");
    }
}

pub(crate) fn block(p: &mut Parser) {
    const END: TokenSet = TS![eof, end, elseif, else, until];

    let m = p.start();
    while !p.at_ts(END) {
        stmt(p);
    }
    m.complete(p, N![Block]);
}

const VARARG_ERROR_MSG: &str = "Nothing can be after a vararg";

fn param_list(p: &mut Parser) -> MarkerComplete {
    const END: TokenSet = TS![')', eof];

    let m = p.start();
    p.expect(T!['(']);

    if !p.at_ts(END) {
        multi_name_r(p, STMT_RECOVERY, true);
    }

    if p.at(T![...]) {
        p.bump(T![...]);

        if !p.at(T![')']) {
            p.err_and_bump(VARARG_ERROR_MSG)
        }
    }

    p.expect(T![')']);
    m.complete(p, N![ParamList])
}

pub(crate) fn reparser(node: SyntaxKind) -> Option<fn(&mut Parser)> {
    Some(match node {
        N![Block] => block,
        _ => return None,
    })
}

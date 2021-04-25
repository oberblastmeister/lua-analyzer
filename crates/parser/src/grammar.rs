mod expressions;
mod statements;

use crate::{
    parser::{MarkerComplete, Parser},
    SyntaxKind::*,
    TokenSet, TS,
};
use expressions::expr_single;
pub use expressions::LuaOp;
use statements::{stmt, STMT_RECOVERY};

pub(crate) fn root(p: &mut Parser) {
    let m = p.start();
    while !p.at(T![eof]) {
        stmt(p);
    }
    m.complete(p, SourceFile);
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

fn multi_name_r(p: &mut Parser, recovery: TokenSet, vararg: bool) {
    let recovery = recovery.union(TokenSet::new(&[T![,]]));

    let m = p.start();

    if vararg && p.at(T![...]) {
        m.complete(p, MultiName);
        return
    }

    name_r(p, recovery);
    while p.at(T![,]) {
        p.bump(T![,]);

        if vararg && p.at(T![...]) {
            break;
        }

        name_r(p, recovery);
    }
    m.complete(p, MultiName);
}

fn name_r(p: &mut Parser, recovery: TokenSet) -> Option<MarkerComplete> {
    if p.at(T![ident]) {
        let m = p.start();
        p.bump(T![ident]);
        Some(m.complete(p, Name))
    } else {
        p.err_recover("expected a name", recovery);
        None
    }
}

fn name_ref_r(p: &mut Parser, recovery: TokenSet) {
    if p.at(T![ident]) {
        let m = p.start();
        p.bump(T![ident]);
        m.complete(p, NameRef);
    } else {
        p.err_recover("expected a name referencer", recovery);
    }
}

fn name_ref(p: &mut Parser) -> MarkerComplete {
    let m = p.start();
    p.expect(T![ident]);
    m.complete(p, NameRef)
}

fn name(p: &mut Parser) {
    if p.at(T![ident]) {
        let m = p.start();
        p.bump(T![ident]);
        m.complete(p, Name);
    } else {
        p.error("expected a name");
    }
}

fn block(p: &mut Parser) -> MarkerComplete {
    const END: TokenSet = TS![eof, end, elseif, else, until];

    let m = p.start();
    while !p.at_ts(END) {
        stmt(p);
    }
    m.complete(p, Block)
}

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
            p.error("Nothing can be after a vararg");
        }
    }

    p.expect(T![')']);
    m.complete(p, ParamList)
}

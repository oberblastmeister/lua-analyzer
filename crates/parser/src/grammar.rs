mod expressions;
mod statements;

use crate::{
    parser::{MarkerComplete, Parser},
    SyntaxKind::*,
    TokenSet, TS,
};
use expressions::expr_single;
use statements::stmt;

pub(crate) fn root(p: &mut Parser) {
    let m = p.start();
    while !p.at(T![eof]) {
        stmt(p);
    }
    m.complete(p, Program);
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

fn name_ref(p: &mut Parser) -> MarkerComplete {
    let m = p.start();
    p.bump(T![ident]);
    m.complete(p, NameRef)
}

fn name(p: &mut Parser) -> MarkerComplete {
    let m = p.start();
    p.expect(T![ident]);
    m.complete(p, Name)
}

fn body(p: &mut Parser) -> MarkerComplete {
    const END: TokenSet = TS![eof, end, elseif, else, until];

    let m = p.start();
    while !p.at_ts(END) {
        stmt(p);
    }
    m.complete(p, Body)
}

fn param_list(p: &mut Parser) -> MarkerComplete {
    const END: TokenSet = TS![')', eof];

    let m = p.start();
    p.expect(T!['(']);
    if !p.at_ts(END) {
        multi_name(p);
    }
    p.expect(T![')']);
    m.complete(p, Paramlist)
}

use binding_powers::{precedences, Operator, LOWEST, NOT_AN_OP, NOT_AN_OP_INFIX, NOT_AN_OP_PREFIX};

use super::statements::{body, param_list};
use crate::{
    parser::{MarkerComplete, Parser},
    token_set::TokenSet,
    SyntaxKind::{self, *},
};

precedences! {
    enum LuaOp {
        #[Infix, Left]
        Or,

        #[Infix, Left]
        And,

        #[Infix, Left]
        Lt,

        #[Infix, Left]
        Gt,

        #[Infix, Left]
        LtEq,

        #[Infix, Left]
        GtEq,

        #[Infix, Left]
        NotEq,

        #[Infix, Left]
        Eq,

        #[Infix, Left]
        Concat,

        #[Infix, Left]
        Plus,

        #[Infix, Left]
        Minus,

        #[Infix, Left]
        Mul,

        #[Infix, Left]
        Div,

        #[Infix, Left]
        Mod,

        #[Prefix]
        Not,

        #[Prefix]
        Minus,

        #[Infix, Right]
        Power,
    }
}

impl From<SyntaxKind> for Option<LuaOp> {
    fn from(kind: SyntaxKind) -> Option<LuaOp> {
        use LuaOp::*;

        Some(match kind {
            T![or] => Or,
            T![and] => And,
            T![not] => Not,
            T![<] => Lt,
            T![>] => Gt,
            T![<=] => LtEq,
            T![>=] => GtEq,
            T![~=] => NotEq,
            T![==] => Eq,
            T![..] => Concat,
            T![+] => Plus,
            T![-] => Minus,
            T![*] => Mul,
            T![/] => Div,
            T![%] => Mod,
            T![^] => Power,
            _ => return None,
        })
    }
}

fn infix_binding_power(kind: SyntaxKind) -> (u8, u8) {
    <Option<LuaOp>>::from(kind)
        .map(|op| op.infix_power())
        .unwrap_or(NOT_AN_OP_INFIX)
}

fn prefix_binding_power(kind: SyntaxKind) -> ((), u8) {
    <Option<LuaOp>>::from(kind)
        .map(|op| op.prefix_power())
        .unwrap_or(NOT_AN_OP_PREFIX)
}

pub(super) fn expr(p: &mut Parser) -> MarkerComplete {
    expr_multi(p, false)
}

fn expr_multi(p: &mut Parser, in_function: bool) -> MarkerComplete {
    let m = p.start();
    expr_single(p);
    if p.at(T![,]) && !p.at(T![')']) {
        while !p.at(T![eof]) && !(in_function && p.at(T![')'])) {
            p.bump(T![,]);

            if expr_single(p).is_none() {
                break;
            }

            if !p.at(T![,]) {
                break;
            }
        }
    }
    m.complete(p, MultivalExpr)
}

pub(super) fn expr_single(p: &mut Parser) -> Option<MarkerComplete> {
    expr_bp(p, LOWEST)
}

fn expr_bp(p: &mut Parser, min_bp: u8) -> Option<MarkerComplete> {
    let mut lhs = lhs(p)?;

    loop {
        let (l_bp, r_bp) = infix_binding_power(p.current());
        if l_bp < min_bp {
            break;
        }

        // bump the operator token
        p.bump_any();

        let m = lhs.precede(p);
        expr_bp(p, r_bp);
        lhs = m.complete(p, InfixExpr);
    }

    Some(lhs)
}

fn lhs(p: &mut Parser) -> Option<MarkerComplete> {
    let (lhs, can_call) = atom_expr(p)?;
    Some(postfix_expr(p, lhs, can_call))
}

/// Returns the completed marker and whether we can do a function call on this expression
fn atom_expr(p: &mut Parser) -> Option<(MarkerComplete, bool)> {
    let ((), r_bp) = prefix_binding_power(p.current());
    if r_bp > 0 {
        let completed = prefix_expr(p, r_bp)?;
        return Some((completed, false));
    }

    if p.at_ts(LITERAL) {
        return literal(p).map(|it| (it, false));
    }

    let m = match p.current() {
        T![function] => function_expr(p),
        T!['{'] => table_expr(p),
        T![ident] => name_ref(p),
        T!['('] => paren_expr(p),
        _ => {
            p.err_recover("Expected an expression");
            return None;
        }
    };

    Some((m, true))
}

fn prefix_expr(p: &mut Parser, r_bp: u8) -> Option<MarkerComplete> {
    let m = p.start();
    p.bump_any();
    expr_bp(p, r_bp)?;
    let completed = m.complete(p, PrefixExpr);
    return Some(completed);
}

fn postfix_expr(p: &mut Parser, mut lhs: MarkerComplete, mut can_call: bool) -> MarkerComplete {
    loop {
        lhs = match p.current() {
            T!['('] if can_call => call_expr(p, lhs),
            T!['{'] if can_call => table_call_expr(p, lhs),
            T![str] if can_call => string_call_expr(p, lhs),
            T![:] if can_call => method_call_expr(p, lhs),
            T!['['] => index_expr(p, lhs),
            T![.] => dot_expr(p, lhs),
            _ => break,
        };
        can_call = true;
    }

    return lhs;
}

pub(super) fn paren_expr(p: &mut Parser) -> MarkerComplete {
    let m = p.start();
    p.expect(T!['(']);
    expr_bp(p, LOWEST);
    p.expect(T![')']);
    m.complete(p, ParenExpr)
}

pub(crate) const LITERAL: TokenSet =
    TokenSet::new(&[T![true], T![false], T![number], T![str], T![nil]]);

fn literal(p: &mut Parser) -> Option<MarkerComplete> {
    assert!(p.at_ts(LITERAL));
    let m = p.start();
    p.bump_any();
    Some(m.complete(p, Literal))
}

fn table_expr(p: &mut Parser) -> MarkerComplete {
    let m = p.start();
    p.bump(T!['{']);
    while !p.at(T!['}']) {
        table_content(p);
        if !p.accept(T![,]) {
            break;
        }
    }
    p.expect(T!['}']);
    m.complete(p, TableExpr)
}

fn table_content(p: &mut Parser) -> MarkerComplete {
    let m = p.start();
    match p.current() {
        T![ident] | T!['['] => key_value(p),
        _ => positional_value(p),
    };
    m.complete(p, TableContent)
}

fn key_value(p: &mut Parser) -> MarkerComplete {
    let m = p.start();
    table_key(p);
    p.expect(T![=]);
    expr_single(p);
    m.complete(p, KeyValue)
}

fn table_key(p: &mut Parser) -> MarkerComplete {
    let m = p.start();
    match p.current() {
        T![ident] => ident_key(p),
        T!['['] => index_key(p),
        _ => unreachable!(),
    };
    m.complete(p, TableKey)
}

fn ident_key(p: &mut Parser) -> MarkerComplete {
    let m = p.start();
    p.bump(T![ident]);
    m.complete(p, IdentKey)
}

fn index_key(p: &mut Parser) -> MarkerComplete {
    let m = p.start();
    p.bump(T!['[']);
    expr_single(p);
    p.expect(T![']']);
    m.complete(p, IndexKey)
}

fn positional_value(p: &mut Parser) -> MarkerComplete {
    let m = p.start();
    expr_single(p);
    m.complete(p, PositionalValue)
}

fn function_expr(p: &mut Parser) -> MarkerComplete {
    let m = p.start();
    p.bump(T![function]);
    param_list(p);
    body(p);
    p.expect(T![end]);
    m.complete(p, FunctionExpr)
}

pub(crate) fn name_ref(p: &mut Parser) -> MarkerComplete {
    let m = p.start();
    p.bump(T![ident]);
    m.complete(p, NameRef)
}

pub(super) fn call_expr(p: &mut Parser, lhs: MarkerComplete) -> MarkerComplete {
    let m = lhs.precede(p);
    arg_list(p);
    m.complete(p, CallExpr)
}

fn string_call_expr(p: &mut Parser, lhs: MarkerComplete) -> MarkerComplete {
    assert!(p.at(T![str]));
    let m = lhs.precede(p);
    expr_single(p);
    m.complete(p, StringCallExpr)
}

fn table_call_expr(p: &mut Parser, lhs: MarkerComplete) -> MarkerComplete {
    assert!(p.at(T!['{']));
    let m = lhs.precede(p);
    expr_single(p);
    m.complete(p, TableCallExpr)
}

fn arg_list(p: &mut Parser) -> MarkerComplete {
    let m = p.start();
    p.bump(T!['(']);
    if !p.at(T![')']) {
        expr_multi(p, true);
    }
    p.expect(T![')']);
    m.complete(p, Arglist)
}

fn index_expr(p: &mut Parser, lhs: MarkerComplete) -> MarkerComplete {
    let m = lhs.precede(p);
    p.bump(T!['[']);
    if !p.at(T![']']) {
        expr_single(p);
    }
    p.expect(T![']']);
    m.complete(p, IndexExpr)
}

fn method_call_expr(p: &mut Parser, lhs: MarkerComplete) -> MarkerComplete {
    let m = lhs.precede(p);
    p.bump(T![:]);
    name_ref(p);
    arg_list(p);
    m.complete(p, MethodCallExpr)
}

fn dot_expr(p: &mut Parser, lhs: MarkerComplete) -> MarkerComplete {
    let m = lhs.precede(p);
    p.bump(T![.]);
    p.expect(T![ident]);
    m.complete(p, DotExpr)
}

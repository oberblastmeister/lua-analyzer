use binding_powers::{precedences, Operator, LOWEST, NOT_AN_OP, NOT_AN_OP_INFIX, NOT_AN_OP_PREFIX};

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
    if p.at_ts(LITERAL) {
        return literal(p).map(|it| (it, false));
    }

    let m = match p.current() {
        T![ident] => name_ref(p),
        T!['('] => paren_expr(p),
        _ => {
            p.err_recover("Expected an expression");
            return None;
        }
    };

    Some((m, true))
}

fn postfix_expr(p: &mut Parser, mut lhs: MarkerComplete, mut can_call: bool) -> MarkerComplete {
    loop {
        lhs = match p.current() {
            T!['('] if can_call => call_expr(p, lhs),
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
    TokenSet::new(&[T![true], T![false], T![number], T![str], T![function]]);

fn literal(p: &mut Parser) -> Option<MarkerComplete> {
    assert!(p.at_ts(LITERAL));
    let m = p.start();
    p.bump_any();
    Some(m.complete(p, Literal))
}

fn name_ref(p: &mut Parser) -> MarkerComplete {
    let m = p.start();
    p.bump(T![ident]);
    m.complete(p, NameRef)
}

pub(super) fn call_expr(p: &mut Parser, lhs: MarkerComplete) -> MarkerComplete {
    let m = lhs.precede(p);
    arg_list(p);
    m.complete(p, CallExpr)
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

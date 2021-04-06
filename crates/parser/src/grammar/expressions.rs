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

pub(super) fn expr(p: &mut Parser) -> Option<MarkerComplete> {
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
    let lhs = atom_expr(p)?;
    Some(postfix_expr(p, lhs))
}

fn atom_expr(p: &mut Parser) -> Option<MarkerComplete> {
    if p.at_ts(LITERAL) {
        return literal(p);
    }

    let m = match p.current() {
        T![ident] => name_ref(p),
        T!['('] => paren_expr(p)?,
        _ => {
            p.err_recover("Expected an expression");
            return None;
        }
    };

    Some(m)
}

fn postfix_expr(p: &mut Parser, mut lhs: MarkerComplete) -> MarkerComplete {
    loop {
        lhs = match p.current() {
            T!['('] => call_expr(p, lhs),
            T!['['] => index_expr(p, lhs),
            T![:] => method_call_expr(p, lhs),
            T![.] => dot_expr(p, lhs),
            _ => break,
        }
    }

    return lhs;
}

pub(super) fn paren_expr(p: &mut Parser) -> Option<MarkerComplete> {
    let m = p.start();
    p.expect(T!['(']);
    expr_bp(p, LOWEST)?;
    p.expect(T![')']);
    let paren_expr = m.complete(p, ParenExpr);
    Some(call_expr(p, paren_expr))
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

pub(super) fn call_expr(p: &mut Parser, expr: MarkerComplete) -> MarkerComplete {
    if p.at(T!['(']) {
        let m = expr.precede(p);
        arg_list(p);
        m.complete(p, CallExpr)
    } else {
        expr
    }
}

fn index_expr(p: &mut Parser, lhs: MarkerComplete) -> MarkerComplete {
    todo!()
}

fn method_call_expr(p: &mut Parser, lhs: MarkerComplete) -> MarkerComplete {
    todo!()
}

fn dot_expr(p: &mut Parser, lhs: MarkerComplete) -> MarkerComplete {
    todo!()
}

fn arg_list(p: &mut Parser) -> MarkerComplete {
    let m = p.start();
    p.bump(T!['(']);
    p.expect(T![')']);
    m.complete(p, Arglist)
}

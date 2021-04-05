use binding_powers::precedences;

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

const LOWEST: u8 = 1;

fn infix_binding_power(kind: SyntaxKind) -> (u8, u8) {
    const NOT_AN_OP: (u8, u8) = (0, 0);

    match kind {
        T![or] | T![and] => (1, 2),
        T![<] | T![>] | T![<=] | T![>=] | T![~=] | T![==] => (3, 4),
        T![..] => (4, 5),
        T![+] | T![-] => (6, 7),
        T![*] | T![/] => (8, 9),
        T![^] => (12, 11),
        _ => NOT_AN_OP,
    }
}

fn prefix_binding_power(kind: SyntaxKind) -> ((), u8) {
    const NOT_AN_OP: ((), u8) = ((), 0);

    match kind {
        T![not] | T![-] => ((), 10),
        _ => NOT_AN_OP,
    }
}

fn postfix_binding_power(kind: SyntaxKind) -> (u8, ()) {
    const NOT_AN_OP: (u8, ()) = (0, ());

    match kind {
        T!['['] => (13, ()),
        _ => NOT_AN_OP,
    }
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
    let peek = p.nth(1);

    let m = match p.current() {
        T!['('] => paren_expr(p)?,
        T![number] => literal(p)?,
        T![str] => literal(p)?,
        T![function] => literal(p)?,
        // T![ident] if peek == T!['('] => call_expr(p),
        T![ident] => name_ref(p),
        _ => {
            p.err_recover("Expected an expression");
            return None;
        }
    };

    Some(m)
}

fn postfix_expr(
    p: &mut Parser,
    mut lhs: MarkerComplete,
    allow_calls: bool,
) -> Option<MarkerComplete> {
    todo!()
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
    if !p.at_ts(LITERAL) {
        return None;
    }
    let m = p.start();
    p.bump_any();
    Some(m.complete(p, Literal))
}

fn name_ref(p: &mut Parser) -> MarkerComplete {
    let m = p.start();
    p.bump(T![ident]);
    let expr = m.complete(p, NameRef);
    call_expr(p, expr)
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

fn arg_list(p: &mut Parser) -> MarkerComplete {
    let m = p.start();
    p.bump(T!['(']);
    p.expect(T![')']);
    m.complete(p, Arglist)
}

use accept::{Acceptor, Advancer, Any};
use binding_powers::{precedences, Operator, LOWEST, NOT_AN_OP, NOT_AN_OP_INFIX, NOT_AN_OP_PREFIX};

use super::{block, name_r, name_ref, name_unchecked, param_list, VARARG_ERROR_MSG};
use crate::{
    parser::{MarkerComplete, MarkerRegular, Parser},
    ParseError, SyntaxKind, TokenSet, TS,
};

pub(super) const EXPR_RECOVERY_SET: TokenSet = TS![local];

precedences! {
    pub enum LuaOp {
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

        #[Infix, Right]
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
        Len,

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
            T![#] => Len,
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
    <Option<LuaOp>>::from(kind).map(|op| op.infix_power()).unwrap_or(NOT_AN_OP_INFIX)
}

fn prefix_binding_power(kind: SyntaxKind) -> ((), u8) {
    <Option<LuaOp>>::from(kind).map(|op| op.prefix_power()).unwrap_or(NOT_AN_OP_PREFIX)
}

pub(super) const EXPR_FIRST: TokenSet = LHS_FIRST;

pub(super) fn expr(p: &mut Parser) -> MarkerComplete {
    expr_multi(p)
}

fn expr_multi(p: &mut Parser) -> MarkerComplete {
    let m = p.start();

    expr_single(p);

    while p.at(T![,]) {
        p.bump(T![,]);

        expr_single(p);
    }

    m.complete(p, N![MultivalExpr])
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
        p.bump(Any);

        let m = lhs.precede(p);
        expr_bp(p, r_bp);
        lhs = m.complete(p, N![InfixExpr]);
    }

    Some(lhs)
}

const LHS_FIRST: TokenSet = TS![not, #, -].union(ATOM_EXPR_FIRST);

fn lhs(p: &mut Parser) -> Option<MarkerComplete> {
    let ((), r_bp) = prefix_binding_power(p.current());
    if r_bp > NOT_AN_OP {
        let m = p.start();
        p.bump(Any);
        expr_bp(p, r_bp);
        return Some(m.complete(p, N![PrefixExpr]));
    }

    let (lhs, can_call) = atom_expr(p)?;
    Some(postfix_expr(p, lhs, can_call))
}

const ATOM_EXPR_FIRST: TokenSet = TS![function, '{', ident, '('].union(LITERAL_FIRST);

/// Returns the completed marker and whether we can do a function call on this expression
fn atom_expr(p: &mut Parser) -> Option<(MarkerComplete, bool)> {
    let ((), r_bp) = prefix_binding_power(p.current());
    if r_bp > 0 {
        let completed = prefix_expr(p, r_bp)?;
        return Some((completed, false));
    }

    if p.at(LITERAL_FIRST) {
        return literal(p).map(|it| (it, false));
    }

    let m = match p.current() {
        T![function] => function_expr(p),
        T!['{'] => table_expr(p),
        T![ident] => name_ref(p),
        T!['('] => paren_expr(p),
        _ => {
            p.err_recover("Expected an expression", EXPR_RECOVERY_SET);
            return None;
        }
    };

    Some((m, true))
}

fn prefix_expr(p: &mut Parser, r_bp: u8) -> Option<MarkerComplete> {
    let m = p.start();
    p.bump(Any);
    expr_bp(p, r_bp)?;
    let completed = m.complete(p, N![PrefixExpr]);
    return Some(completed);
}

const CALL_FIRST: TokenSet = TS!['{', '(', str];

fn postfix_expr(p: &mut Parser, mut lhs: MarkerComplete, mut can_call: bool) -> MarkerComplete {
    loop {
        lhs = match p.current() {
            _ if can_call && p.at(CALL_FIRST) => call_expr(p, lhs),
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
    m.complete(p, N![ParenExpr])
}

pub(super) const LITERAL_FIRST: TokenSet = TS![true, false, number, str, nil, ...];

fn literal(p: &mut Parser) -> Option<MarkerComplete> {
    assert!(p.at(LITERAL_FIRST));
    let m = p.start();
    p.bump(Any);
    Some(m.complete(p, N![Literal]))
}

const TABLE_SEP: TokenSet = TokenSet::new(&[T![,], T![;]]);

fn table_sep(p: &mut Parser) -> MarkerComplete {
    let m = p.start();
    p.bump_ts(TABLE_SEP);
    m.complete(p, N![TableSep])
}

const TABLE_RECOVERY: TokenSet = TS!['}', '['].union(TABLE_SEP);

fn table_expr(p: &mut Parser) -> MarkerComplete {
    let m = p.start();

    p.bump(T!['{']);

    const END: TokenSet = TS![eof, end, '}'];

    // might need to make this part faster
    while !p.at(END) {
        table_content(p);

        while !p.at(END) {
            if p.at(TABLE_SEP) {
                table_sep(p);
                break;
            } else if p.at(TABLE_RECOVERY) {
                break;
            } else {
                table_correct(p);
                continue;
            }
        }
    }

    p.expect(T!['}']);
    m.complete(p, N![TableExpr])
}

fn table_correct(p: &mut Parser) {
    p.err_until("Failed to find comma or closing bracket", TABLE_RECOVERY);
}

fn table_content(p: &mut Parser) -> MarkerComplete {
    let m = p.start();
    match p.current() {
        T![ident] => {
            if p.nth(1) == T![=] {
                let m = p.start();

                {
                    let m = p.start();
                    name_unchecked(p);
                    m.complete(p, N![TableKey]);
                }

                p.bump(T![=]);
                expr_single(p);
                m.complete(p, N![KeyValue]);
            } else {
                let m = p.start();
                expr_single(p);
                m.complete(p, N![PositionalValue]);
            }
        }
        T!['['] => {
            let m = p.start();

            {
                let m = p.start();
                index(p);
                m.complete(p, N![TableKey]);
            }

            p.expect(T![=]);
            expr_single(p);
            m.complete(p, N![KeyValue]);
        }
        _ if p.at(EXPR_FIRST) => {
            let m = p.start();
            expr_single(p);
            m.complete(p, N![PositionalValue]);
        }
        _ => {
            p.error("Expected an expression");
        }
    };
    m.complete(p, N![TableContent])
}

fn function_expr(p: &mut Parser) -> MarkerComplete {
    let m = p.start();
    p.bump(T![function]);
    param_list(p);
    block(p);
    p.expect(T![end]);
    m.complete(p, N![FunctionExpr])
}

pub(super) fn call_expr(p: &mut Parser, lhs: MarkerComplete) -> MarkerComplete {
    let m = lhs.precede(p);
    call_args(p);
    m.complete(p, N![CallExpr])
}

fn call_args(p: &mut Parser) -> MarkerComplete {
    assert!(p.at(CALL_FIRST));

    let m = p.start();

    match p.current() {
        T!['('] => {
            arg_list(p);
        }
        T!['{'] => {
            table_expr(p);
        }
        T![str] => {
            p.bump(Any);
        }
        _ => unreachable!(),
    }

    m.complete(p, N![CallArgs])
}

fn arg_list(p: &mut Parser) -> MarkerComplete {
    let m = p.start();

    p.bump(T!['(']);

    if !p.at(T![')']) {
        expr(p);
    }

    p.expect(T![')']);
    m.complete(p, N![ArgList])
}

fn index_expr(p: &mut Parser, lhs: MarkerComplete) -> MarkerComplete {
    let m = lhs.precede(p);
    index(p);
    m.complete(p, N![IndexExpr])
}

fn method_call_expr(p: &mut Parser, lhs: MarkerComplete) -> MarkerComplete {
    let m = lhs.precede(p);
    p.bump(T![:]);
    name_ref(p);
    if p.at(CALL_FIRST) {
        call_args(p);
    } else {
        p.error("Expected call arguments");
    }
    m.complete(p, N![MethodCallExpr])
}

fn dot_expr(p: &mut Parser, lhs: MarkerComplete) -> MarkerComplete {
    let m = lhs.precede(p);
    p.bump(T![.]);
    p.expect(T![ident]);
    m.complete(p, N![DotExpr])
}

fn index(p: &mut Parser) -> MarkerComplete {
    let m = p.start();
    p.bump(T!['[']);
    expr_single(p);
    p.expect(T![']']);
    m.complete(p, N![Index])
}

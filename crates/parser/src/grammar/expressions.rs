use crate::{
    parser::{CompletedMarker, Marker, MarkerComplete, Parser},
    token_set::TokenSet,
    SyntaxKind::*,
    TokenSource,
};

const LOWEST: u8 = 0;

pub(super) fn expr(p: &mut Parser) -> Option<MarkerComplete> {
    expr_bp(p, LOWEST)
}

fn expr_bp(p: &mut Parser, bp: u8) -> Option<MarkerComplete> {
    lhs(p)
}

fn lhs(p: &mut Parser) -> Option<MarkerComplete> {
    let peek = p.nth(1);
    let m = match p.current() {
        T!['('] => paren_expr(p)?,
        T![number] => literal(p)?,
        T![str] => literal(p)?,
        T![ident] if peek == T!['('] => todo!(),
        T![ident] => name_ref(p),
        _ => {
            p.err_recover("Expected an expression");
            return None;
        }
    };
    Some(m)
}

fn paren_expr(p: &mut Parser) -> Option<MarkerComplete> {
    let m = p.start();
    p.bump(T!['(']);
    expr_bp(p, LOWEST)?;
    p.expect(T![')']);
    Some(m.complete(p, ParenExpr))
}

pub(crate) const LITERAL: TokenSet = TokenSet::new(&[T![true], T![false], T![number], T![str]]);

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
    m.complete(p, NameRef)
}

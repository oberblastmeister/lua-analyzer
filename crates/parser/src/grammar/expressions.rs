use crate::{
    parser::{CompletedMarker, MarkerComplete, Parser},
    token_set::TokenSet,
    SyntaxKind::*,
    TokenSource,
};

const LOWEST: u8 = 0;

pub(super) fn expr(p: &mut Parser) {
    expr_bp(p, LOWEST);
}

fn expr_bp(p: &mut Parser, bp: u8) -> Option<MarkerComplete> {
    lhs(p)
}

fn lhs(p: &mut Parser) -> Option<MarkerComplete> {
    let peek = p.nth(1);
    Some(match p.current() {
        T![number] => literal(p)?,
        T![str] => literal(p)?,
        T![ident] if peek == T!['('] => todo!(),
        T![ident] => name_ref(p),
        _ => todo!(),
    })
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

use syntax::ast;

use crate::DefDatabase;

use super::{Body, Expander};

pub(super) fn lower(db: &dyn DefDatabase, expander: Expander, params: Option<ast::ParamList>, body: Option<ast::Block>) -> Body {
    todo!()
}

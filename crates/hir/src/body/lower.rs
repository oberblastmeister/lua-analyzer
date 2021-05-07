use la_arena::Arena;
use syntax::ast;

use super::Body;
use crate::{expr::ParamList, stmt, DefDatabase};

pub(super) fn lower(
    db: &dyn DefDatabase,
    params: Option<ast::ParamList>,
    body: Option<ast::Block>,
) -> Body {
    Collector {
        db,
        body: Body {
            exprs: Arena::default(),
            labels: Arena::default(),
            params: ParamList::default(),
            body_stmt: stmt::dummy_id(),
            block_scopes: Vec::default(),
        },
    }
    .collect(params, body)
}

struct Collector<'db> {
    db: &'db dyn DefDatabase,
    body: Body,
}

impl<'db> Collector<'db> {
    fn collect(mut self, params_list: Option<ast::ParamList>, body: Option<ast::Block>) -> Body {
        if let Some(param_list) = params_list {
        }
        todo!()
    }
}

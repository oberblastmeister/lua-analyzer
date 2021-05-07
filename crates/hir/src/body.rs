mod lower;

use std::sync::Arc;

use base_db::FileId;
use la_arena::Arena;
use syntax::ast;

use crate::{
    expr::{Expr, Label, ParamList},
    item_tree::HasSource,
    stmt::{BlockLoc, StmtId},
    with_body::WithBodyId,
    DefDatabase, Lookup,
};

pub struct Body {
    pub exprs: Arena<Expr>,
    pub labels: Arena<Label>,
    pub params: ParamList,
    pub body_stmt: StmtId,
    block_scopes: Vec<BlockLoc>,
}

impl Body {
    fn body_query(db: &dyn DefDatabase, def: WithBodyId) -> Arc<Body> {
        let (file_id, body) = match def {
            WithBodyId::ModuleId(file_id) => {
                todo!()
            }
            WithBodyId::FunctionId(f) => {
                let f = f.lookup(db);
                let src = f.source(db);
                (src.file_id, src.value.body())
            }
            WithBodyId::LocalFunctionId(f) => {
                let f = f.lookup(db);
                let src = f.source(db);
                (src.file_id, src.value.body())
            }
        };
        todo!()
    }

    fn new(
        db: &dyn DefDatabase,
        expander: Expander,
        params: Option<ast::ParamList>,
        body: Option<ast::Block>,
    ) -> Body {
        lower::lower(db, expander, params, body)
    }
}

pub struct Expander {}
// pub fn new(db: &dyn DefDatabase, file_id: FileId) -> Expander {
// }

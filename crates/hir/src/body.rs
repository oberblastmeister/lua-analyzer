mod lower;

use std::sync::Arc;

use base_db::FileId;
use la_arena::Arena;
use syntax::ast;

use crate::{
    ast_id_map::AstIdMap,
    expr::{Expr, Label, ParamList},
    item_tree::HasSource,
    stmt::{BlockLoc, StmtId},
    with_body::WithBodyId,
    DefDatabase, Lookup,
};

#[derive(Debug, Eq, PartialEq)]
pub struct Body {
    pub exprs: Arena<Expr>,
    pub labels: Arena<Label>,
    pub params: ParamList,
    pub body_stmt: StmtId,
    block_scopes: Vec<BlockLoc>,
}

impl Body {
    pub(crate) fn body_query(db: &dyn DefDatabase, def: WithBodyId) -> Arc<Body> {
        let mut params = None;

        let (file_id, body) = match def {
            WithBodyId::ModuleId(file_id) => {
                let src = db.parse(file_id).tree();
                (file_id, src.body())
            }
            WithBodyId::FunctionId(f) => {
                let f = f.lookup(db);
                let src = f.source(db);
                params = src.value.param_list();
                (src.file_id, src.value.body())
            }
            WithBodyId::LocalFunctionId(f) => {
                let f = f.lookup(db);
                let src = f.source(db);
                params = src.value.param_list();
                (src.file_id, src.value.body())
            }
        };

        let mut body = Body::new(db, params, body);
        body.shrink_to_fit();
        Arc::new(body)
    }

    fn new(
        db: &dyn DefDatabase,
        // expander: Expander,
        params: Option<ast::ParamList>,
        body: Option<ast::Block>,
    ) -> Body {
        lower::lower(db, params, body)
    }

    fn shrink_to_fit(&mut self) {
        let Self { exprs, labels, params, body_stmt: _, block_scopes } = self;
        block_scopes.shrink_to_fit();
        exprs.shrink_to_fit();
        labels.shrink_to_fit();
        params.shrink_to_fit();
    }
}

// pub struct Expander {
//     file_id: FileId,
//     ast_id_map: Arc<AstIdMap>,
// }

// impl Expander {
//     pub fn new(db: &dyn DefDatabase, file_id: FileId) -> Self { Self {  } }
// }
// pub fn new(db: &dyn DefDatabase, file_id: FileId) -> Expander {
// }

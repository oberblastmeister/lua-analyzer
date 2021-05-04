use base_db::FileId;
use la_arena::Arena;

use crate::{
    expr::{Expr, Label, ParamList},
    stmt::{BlockLoc, StmtId},
};

pub struct Body {
    pub exprs: Arena<Expr>,
    pub labels: Arena<Label>,
    pub params: ParamList,
    pub body_stmt: StmtId,
    block_scopes: Vec<BlockLoc>,
}

impl Body {
    
}

pub struct Expander {
}
    // pub fn new(db: &dyn DefDatabase, file_id: FileId) -> Expander {
    // }

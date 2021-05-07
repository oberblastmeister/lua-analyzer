use la_arena::{Idx, RawIdx};
use syntax::ast::{self, MultiName, MultivalExpr};

use crate::{
    expr::{Call, ExprId},
    item_tree::AstId,
    name::Name,
};

pub(crate) fn dummy_id() -> StmtId {
    StmtId::from_raw(RawIdx::from(!0))
}

pub type StmtId = Idx<Stmt>;

pub enum Stmt {
    Assign { lhs: MultivalExpr, rhs: MultivalExpr },
    Do(Block),
    While { condition: ExprId, body: Block },
    For { content: ForContent, body: Block },
    Call(Call),
    If { condition: ExprId, body: Block },
    Break,
    Return { expr: ExprId },
    Label { name: Name },
    Goto { name: Name },
}

pub enum ForContent {
    Numeric { name: Name, start: ExprId, end: ExprId, increment: Option<ExprId> },
    Generic { multi_name: MultiName, expr: ExprId },
}

pub struct Block {
    block_loc: BlockLoc,
    stmts: Vec<StmtId>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockLoc {
    ast_id: AstId<ast::Block>,
}

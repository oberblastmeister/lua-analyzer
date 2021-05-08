use la_arena::{Idx, RawIdx};
use syntax::ast;

use crate::{
    expr::{Call, ExprId, MultivalExpr},
    item_tree::AstId,
    name::{MultiName, Name},
};

pub(crate) fn dummy_id() -> StmtId {
    StmtId::from_raw(RawIdx::from(!0))
}

pub type StmtId = Idx<Stmt>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Stmt {
    Missing,
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

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ForContent {
    Numeric { name: Name, start: ExprId, end: ExprId, increment: Option<ExprId> },
    Generic { multi_name: MultiName, expr: ExprId },
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Block {
    block_loc: BlockLoc,
    stmts: Vec<StmtId>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockLoc {
    ast_id: AstId<ast::Block>,
}

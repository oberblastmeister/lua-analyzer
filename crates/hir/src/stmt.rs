use la_arena::Idx;

use crate::{
    expr::{Call, Expr, ExprId},
    name::{MultiName, Name},
};

pub type StmtId = Idx<Stmt>;

pub enum Stmt {
    Assign { name: MultiName, initializer: Option<ExprId> },
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
    // change to multiname
    Generic { name: Name, expr: ExprId },
}

pub struct Block {
    stmts: Vec<StmtId>,
}

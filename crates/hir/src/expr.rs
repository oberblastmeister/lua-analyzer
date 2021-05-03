use la_arena::Idx;
use ordered_float::OrderedFloat;

use crate::{
    name::{MultiName, Name},
    stmt::StmtId,
};

pub type ExprId = Idx<Expr>;

pub enum Expr {
    Missing,
    Call(Call),
    Literal(Literal),
    BinaryOp { lhs: ExprId, op: BinaryOp, rhs: ExprId },
    UnaryOp { op: UnaryOp, expr: ExprId },
    Index { base: ExprId, index: ExprId },
    Dot { base: ExprId, index: Name },
    Function { params: ParamList, body: StmtId },
    Table(Table),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ParamList {
    names: MultiName,
    vararg: bool,
}

pub struct Call {
    callee: ExprId,
    method_name: Option<Name>,
    args: CallArgs,
}

pub enum CallArgs {
    ArgList { args: Vec<ExprId> },
    Str(String),
    Table(Table),
}

pub struct BinaryOp {}

pub struct UnaryOp {}

pub struct Table {}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Literal {
    Str(String),
    Bool(bool),
    Number(OrderedFloat<f64>),
    Nil,
    Vararg,
}

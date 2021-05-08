use la_arena::Idx;
use ordered_float::OrderedFloat;

use crate::{
    name::{MultiName, Name},
    stmt::StmtId,
};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Label {
    name: Name,
}

pub type ExprId = Idx<Expr>;

#[derive(Debug, Clone, Eq, PartialEq)]
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
pub struct MultivalExpr;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
pub struct ParamList {
    pub names: MultiName,
    pub vararg: bool,
}

impl ParamList {
    pub fn shrink_to_fit(&mut self) {
        self.names.shrink_to_fit()
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Call {
    callee: ExprId,
    method_name: Option<Name>,
    args: CallArgs,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum CallArgs {
    ArgList { args: Vec<ExprId> },
    Str(String),
    Table(Table),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BinaryOp {}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UnaryOp {}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Table {}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Literal {
    Str(String),
    Bool(bool),
    Number(OrderedFloat<f64>),
    Nil,
    Vararg,
}

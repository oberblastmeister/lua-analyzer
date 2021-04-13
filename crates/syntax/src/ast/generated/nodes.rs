//! Generated file, do not edit by hand, see `xtask/src/codegen`

#![allow(dead_code)]
use crate::{
    ast::{support, AstChildren, AstNode},
    SyntaxKind, SyntaxNode, SyntaxToken, T,
};
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Program {
    pub(crate) syntax: SyntaxNode,
}
impl Program {
    pub fn stmts(&self) -> AstChildren<Stmt> {
        support::children(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AssignStmt {
    pub(crate) syntax: SyntaxNode,
}
impl AssignStmt {
    pub fn multival_expr(&self) -> Option<MultivalExpr> {
        support::child(&self.syntax)
    }
    pub fn eq_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![=])
    }
    pub fn expr(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LocalAssignStmt {
    pub(crate) syntax: SyntaxNode,
}
impl LocalAssignStmt {
    pub fn local_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![local])
    }
    pub fn multi_name(&self) -> Option<MultiName> {
        support::child(&self.syntax)
    }
    pub fn eq_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![=])
    }
    pub fn expr(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FunctionDefStmt {
    pub(crate) syntax: SyntaxNode,
}
impl FunctionDefStmt {
    pub fn local_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![local])
    }
    pub fn function_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![function])
    }
    pub fn name(&self) -> Option<Name> {
        support::child(&self.syntax)
    }
    pub fn paramlist(&self) -> Option<Paramlist> {
        support::child(&self.syntax)
    }
    pub fn body(&self) -> Option<Body> {
        support::child(&self.syntax)
    }
    pub fn end_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![end])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ForStmt {
    pub(crate) syntax: SyntaxNode,
}
impl ForStmt {
    pub fn for_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![for])
    }
    pub fn content(&self) -> Option<ForContent> {
        support::child(&self.syntax)
    }
    pub fn do_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![do])
    }
    pub fn body(&self) -> Option<Body> {
        support::child(&self.syntax)
    }
    pub fn end_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![end])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IfStmt {
    pub(crate) syntax: SyntaxNode,
}
impl IfStmt {
    pub fn if_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![if])
    }
    pub fn cond(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }
    pub fn then_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![then])
    }
    pub fn body(&self) -> Option<Body> {
        support::child(&self.syntax)
    }
    pub fn else_branch(&self) -> Option<ElseBranch> {
        support::child(&self.syntax)
    }
    pub fn else_if_branch(&self) -> Option<ElseIfBranch> {
        support::child(&self.syntax)
    }
    pub fn end_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![end])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReturnStmt {
    pub(crate) syntax: SyntaxNode,
}
impl ReturnStmt {
    pub fn return_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![return])
    }
    pub fn expr(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExprStmt {
    pub(crate) syntax: SyntaxNode,
}
impl ExprStmt {
    pub fn expr(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BreakStmt {
    pub(crate) syntax: SyntaxNode,
}
impl BreakStmt {
    pub fn break_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![break])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LabelStmt {
    pub(crate) syntax: SyntaxNode,
}
impl LabelStmt {
    pub fn start(&self) -> Option<LabelDelim> {
        support::child(&self.syntax)
    }
    pub fn name(&self) -> Option<Name> {
        support::child(&self.syntax)
    }
    pub fn finish(&self) -> Option<LabelDelim> {
        support::child(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RepeatUntilStmt {
    pub(crate) syntax: SyntaxNode,
}
impl RepeatUntilStmt {
    pub fn repeat_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![repeat])
    }
    pub fn body(&self) -> Option<Body> {
        support::child(&self.syntax)
    }
    pub fn until_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![until])
    }
    pub fn expr(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Literal {
    pub(crate) syntax: SyntaxNode,
}
impl Literal {
    pub fn number_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![number])
    }
    pub fn true_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![true])
    }
    pub fn false_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![false])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TableExpr {
    pub(crate) syntax: SyntaxNode,
}
impl TableExpr {
    pub fn l_curly_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['{'])
    }
    pub fn table_contents(&self) -> Option<TableContent> {
        support::child(&self.syntax)
    }
    pub fn r_curly_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['}'])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InfixExpr {
    pub(crate) syntax: SyntaxNode,
}
impl InfixExpr {
    pub fn lhs(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }
    pub fn op(&self) -> Option<InfixOp> {
        support::child(&self.syntax)
    }
    pub fn rhs(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PrefixExpr {
    pub(crate) syntax: SyntaxNode,
}
impl PrefixExpr {
    pub fn bang_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![!])
    }
    pub fn expr(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IndexExpr {
    pub(crate) syntax: SyntaxNode,
}
impl IndexExpr {
    pub fn base(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }
    pub fn index(&self) -> Option<Index> {
        support::child(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DotExpr {
    pub(crate) syntax: SyntaxNode,
}
impl DotExpr {
    pub fn expr(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }
    pub fn dot_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![.])
    }
    pub fn ident_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![ident])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CallExpr {
    pub(crate) syntax: SyntaxNode,
}
impl CallExpr {
    pub fn fun(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['('])
    }
    pub fn args(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![')'])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FunctionExpr {
    pub(crate) syntax: SyntaxNode,
}
impl FunctionExpr {
    pub fn function_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![function])
    }
    pub fn params(&self) -> Option<Paramlist> {
        support::child(&self.syntax)
    }
    pub fn body(&self) -> Option<Body> {
        support::child(&self.syntax)
    }
    pub fn end_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![end])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TableCallExpr {
    pub(crate) syntax: SyntaxNode,
}
impl TableCallExpr {
    pub fn fun(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }
    pub fn tbl(&self) -> Option<TableExpr> {
        support::child(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StringCallExpr {
    pub(crate) syntax: SyntaxNode,
}
impl StringCallExpr {
    pub fn fun(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MethodCallExpr {
    pub(crate) syntax: SyntaxNode,
}
impl MethodCallExpr {
    pub fn fun(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }
    pub fn colon_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![:])
    }
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['('])
    }
    pub fn args(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![')'])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NameRef {
    pub(crate) syntax: SyntaxNode,
}
impl NameRef {
    pub fn ident_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![ident])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MultivalExpr {
    pub(crate) syntax: SyntaxNode,
}
impl MultivalExpr {
    pub fn exprs(&self) -> AstChildren<Expr> {
        support::children(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Arglist {
    pub(crate) syntax: SyntaxNode,
}
impl Arglist {
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['('])
    }
    pub fn args(&self) -> AstChildren<Expr> {
        support::children(&self.syntax)
    }
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![')'])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Paramlist {
    pub(crate) syntax: SyntaxNode,
}
impl Paramlist {
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['('])
    }
    pub fn multi_name(&self) -> Option<MultiName> {
        support::child(&self.syntax)
    }
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![')'])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Body {
    pub(crate) syntax: SyntaxNode,
}
impl Body {
    pub fn stmts(&self) -> AstChildren<Stmt> {
        support::children(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParenExpr {
    pub(crate) syntax: SyntaxNode,
}
impl ParenExpr {
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['('])
    }
    pub fn expr(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![')'])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KeyValue {
    pub(crate) syntax: SyntaxNode,
}
impl KeyValue {
    pub fn key(&self) -> Option<TableKey> {
        support::child(&self.syntax)
    }
    pub fn value(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PositionalValue {
    pub(crate) syntax: SyntaxNode,
}
impl PositionalValue {
    pub fn expr(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Index {
    pub(crate) syntax: SyntaxNode,
}
impl Index {
    pub fn l_brack_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['['])
    }
    pub fn index(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }
    pub fn r_brack_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![']'])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IdentKey {
    pub(crate) syntax: SyntaxNode,
}
impl IdentKey {
    pub fn ident_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![ident])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InfixOp {
    pub(crate) syntax: SyntaxNode,
}
impl InfixOp {
    pub fn caret_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![^])
    }
    pub fn not_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![not])
    }
    pub fn asterisk_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![*])
    }
    pub fn slash_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![/])
    }
    pub fn plus_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![+])
    }
    pub fn minus_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![-])
    }
    pub fn double_dot_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![..])
    }
    pub fn lt_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![<])
    }
    pub fn gt_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![>])
    }
    pub fn lt_eq_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![<=])
    }
    pub fn gt_eq_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![>=])
    }
    pub fn not_eq_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![~=])
    }
    pub fn eq_eq_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![==])
    }
    pub fn and_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![and])
    }
    pub fn or_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![or])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LabelDelim {
    pub(crate) syntax: SyntaxNode,
}
impl LabelDelim {
    pub fn double_colon_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![::])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Name {
    pub(crate) syntax: SyntaxNode,
}
impl Name {
    pub fn ident_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![ident])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GotoStmt {
    pub(crate) syntax: SyntaxNode,
}
impl GotoStmt {
    pub fn goto_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![goto])
    }
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DoStmt {
    pub(crate) syntax: SyntaxNode,
}
impl DoStmt {
    pub fn do_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![do])
    }
    pub fn body(&self) -> Option<Body> {
        support::child(&self.syntax)
    }
    pub fn end_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![end])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MultiName {
    pub(crate) syntax: SyntaxNode,
}
impl MultiName {
    pub fn names(&self) -> AstChildren<Name> {
        support::children(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ElseBranch {
    pub(crate) syntax: SyntaxNode,
}
impl ElseBranch {
    pub fn else_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![else])
    }
    pub fn body(&self) -> Option<Body> {
        support::child(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ElseIfBranch {
    pub(crate) syntax: SyntaxNode,
}
impl ElseIfBranch {
    pub fn elseif_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![elseif])
    }
    pub fn cond(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }
    pub fn then_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![then])
    }
    pub fn body(&self) -> Option<Body> {
        support::child(&self.syntax)
    }
    pub fn else_if_branch(&self) -> Option<ElseIfBranch> {
        support::child(&self.syntax)
    }
    pub fn else_branch(&self) -> Option<ElseBranch> {
        support::child(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WhileStmt {
    pub(crate) syntax: SyntaxNode,
}
impl WhileStmt {
    pub fn while_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![while])
    }
    pub fn cond(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }
    pub fn do_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![do])
    }
    pub fn body(&self) -> Option<Body> {
        support::child(&self.syntax)
    }
    pub fn end_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![end])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NumericFor {
    pub(crate) syntax: SyntaxNode,
}
impl NumericFor {
    pub fn name(&self) -> Option<Name> {
        support::child(&self.syntax)
    }
    pub fn eq_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![=])
    }
    pub fn expr(&self) -> Option<MultivalExpr> {
        support::child(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GenericFor {
    pub(crate) syntax: SyntaxNode,
}
impl GenericFor {
    pub fn multi_name(&self) -> Option<MultiName> {
        support::child(&self.syntax)
    }
    pub fn in_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![in])
    }
    pub fn expr(&self) -> Option<MultivalExpr> {
        support::child(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Stmt {
    AssignStmt(AssignStmt),
    LocalAssignStmt(LocalAssignStmt),
    FunctionDefStmt(FunctionDefStmt),
    ForStmt(ForStmt),
    IfStmt(IfStmt),
    ReturnStmt(ReturnStmt),
    ExprStmt(ExprStmt),
    BreakStmt(BreakStmt),
    LabelStmt(LabelStmt),
    RepeatUntilStmt(RepeatUntilStmt),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expr {
    Literal(Literal),
    TableExpr(TableExpr),
    InfixExpr(InfixExpr),
    PrefixExpr(PrefixExpr),
    IndexExpr(IndexExpr),
    DotExpr(DotExpr),
    CallExpr(CallExpr),
    FunctionExpr(FunctionExpr),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TableContent {
    KeyValue(KeyValue),
    PositionalValue(PositionalValue),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TableKey {
    Index(Index),
    IdentKey(IdentKey),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ForContent {
    NumericFor(NumericFor),
    GenericFor(GenericFor),
}
impl AstNode for Program {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::Program
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AssignStmt {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::AssignStmt
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for LocalAssignStmt {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::LocalAssignStmt
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for FunctionDefStmt {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::FunctionDefStmt
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for ForStmt {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ForStmt
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for IfStmt {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::IfStmt
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for ReturnStmt {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ReturnStmt
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for ExprStmt {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ExprStmt
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for BreakStmt {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::BreakStmt
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for LabelStmt {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::LabelStmt
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for RepeatUntilStmt {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::RepeatUntilStmt
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Literal {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::Literal
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for TableExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::TableExpr
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for InfixExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::InfixExpr
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for PrefixExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::PrefixExpr
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for IndexExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::IndexExpr
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DotExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DotExpr
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CallExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CallExpr
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for FunctionExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::FunctionExpr
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for TableCallExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::TableCallExpr
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for StringCallExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::StringCallExpr
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for MethodCallExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::MethodCallExpr
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for NameRef {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::NameRef
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for MultivalExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::MultivalExpr
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Arglist {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::Arglist
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Paramlist {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::Paramlist
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Body {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::Body
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for ParenExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ParenExpr
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for KeyValue {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::KeyValue
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for PositionalValue {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::PositionalValue
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Index {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::Index
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for IdentKey {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::IdentKey
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for InfixOp {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::InfixOp
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for LabelDelim {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::LabelDelim
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Name {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::Name
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for GotoStmt {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::GotoStmt
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DoStmt {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DoStmt
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for MultiName {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::MultiName
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for ElseBranch {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ElseBranch
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for ElseIfBranch {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ElseIfBranch
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for WhileStmt {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::WhileStmt
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for NumericFor {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::NumericFor
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for GenericFor {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::GenericFor
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl From<AssignStmt> for Stmt {
    fn from(node: AssignStmt) -> Stmt {
        Stmt::AssignStmt(node)
    }
}
impl From<LocalAssignStmt> for Stmt {
    fn from(node: LocalAssignStmt) -> Stmt {
        Stmt::LocalAssignStmt(node)
    }
}
impl From<FunctionDefStmt> for Stmt {
    fn from(node: FunctionDefStmt) -> Stmt {
        Stmt::FunctionDefStmt(node)
    }
}
impl From<ForStmt> for Stmt {
    fn from(node: ForStmt) -> Stmt {
        Stmt::ForStmt(node)
    }
}
impl From<IfStmt> for Stmt {
    fn from(node: IfStmt) -> Stmt {
        Stmt::IfStmt(node)
    }
}
impl From<ReturnStmt> for Stmt {
    fn from(node: ReturnStmt) -> Stmt {
        Stmt::ReturnStmt(node)
    }
}
impl From<ExprStmt> for Stmt {
    fn from(node: ExprStmt) -> Stmt {
        Stmt::ExprStmt(node)
    }
}
impl From<BreakStmt> for Stmt {
    fn from(node: BreakStmt) -> Stmt {
        Stmt::BreakStmt(node)
    }
}
impl From<LabelStmt> for Stmt {
    fn from(node: LabelStmt) -> Stmt {
        Stmt::LabelStmt(node)
    }
}
impl From<RepeatUntilStmt> for Stmt {
    fn from(node: RepeatUntilStmt) -> Stmt {
        Stmt::RepeatUntilStmt(node)
    }
}
impl AstNode for Stmt {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            SyntaxKind::AssignStmt
            | SyntaxKind::LocalAssignStmt
            | SyntaxKind::FunctionDefStmt
            | SyntaxKind::ForStmt
            | SyntaxKind::IfStmt
            | SyntaxKind::ReturnStmt
            | SyntaxKind::ExprStmt
            | SyntaxKind::BreakStmt
            | SyntaxKind::LabelStmt
            | SyntaxKind::RepeatUntilStmt => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            SyntaxKind::AssignStmt => Stmt::AssignStmt(AssignStmt { syntax }),
            SyntaxKind::LocalAssignStmt => Stmt::LocalAssignStmt(LocalAssignStmt { syntax }),
            SyntaxKind::FunctionDefStmt => Stmt::FunctionDefStmt(FunctionDefStmt { syntax }),
            SyntaxKind::ForStmt => Stmt::ForStmt(ForStmt { syntax }),
            SyntaxKind::IfStmt => Stmt::IfStmt(IfStmt { syntax }),
            SyntaxKind::ReturnStmt => Stmt::ReturnStmt(ReturnStmt { syntax }),
            SyntaxKind::ExprStmt => Stmt::ExprStmt(ExprStmt { syntax }),
            SyntaxKind::BreakStmt => Stmt::BreakStmt(BreakStmt { syntax }),
            SyntaxKind::LabelStmt => Stmt::LabelStmt(LabelStmt { syntax }),
            SyntaxKind::RepeatUntilStmt => Stmt::RepeatUntilStmt(RepeatUntilStmt { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Stmt::AssignStmt(it) => &it.syntax,
            Stmt::LocalAssignStmt(it) => &it.syntax,
            Stmt::FunctionDefStmt(it) => &it.syntax,
            Stmt::ForStmt(it) => &it.syntax,
            Stmt::IfStmt(it) => &it.syntax,
            Stmt::ReturnStmt(it) => &it.syntax,
            Stmt::ExprStmt(it) => &it.syntax,
            Stmt::BreakStmt(it) => &it.syntax,
            Stmt::LabelStmt(it) => &it.syntax,
            Stmt::RepeatUntilStmt(it) => &it.syntax,
        }
    }
}
impl From<Literal> for Expr {
    fn from(node: Literal) -> Expr {
        Expr::Literal(node)
    }
}
impl From<TableExpr> for Expr {
    fn from(node: TableExpr) -> Expr {
        Expr::TableExpr(node)
    }
}
impl From<InfixExpr> for Expr {
    fn from(node: InfixExpr) -> Expr {
        Expr::InfixExpr(node)
    }
}
impl From<PrefixExpr> for Expr {
    fn from(node: PrefixExpr) -> Expr {
        Expr::PrefixExpr(node)
    }
}
impl From<IndexExpr> for Expr {
    fn from(node: IndexExpr) -> Expr {
        Expr::IndexExpr(node)
    }
}
impl From<DotExpr> for Expr {
    fn from(node: DotExpr) -> Expr {
        Expr::DotExpr(node)
    }
}
impl From<CallExpr> for Expr {
    fn from(node: CallExpr) -> Expr {
        Expr::CallExpr(node)
    }
}
impl From<FunctionExpr> for Expr {
    fn from(node: FunctionExpr) -> Expr {
        Expr::FunctionExpr(node)
    }
}
impl AstNode for Expr {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            SyntaxKind::Literal
            | SyntaxKind::TableExpr
            | SyntaxKind::InfixExpr
            | SyntaxKind::PrefixExpr
            | SyntaxKind::IndexExpr
            | SyntaxKind::DotExpr
            | SyntaxKind::CallExpr
            | SyntaxKind::FunctionExpr => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            SyntaxKind::Literal => Expr::Literal(Literal { syntax }),
            SyntaxKind::TableExpr => Expr::TableExpr(TableExpr { syntax }),
            SyntaxKind::InfixExpr => Expr::InfixExpr(InfixExpr { syntax }),
            SyntaxKind::PrefixExpr => Expr::PrefixExpr(PrefixExpr { syntax }),
            SyntaxKind::IndexExpr => Expr::IndexExpr(IndexExpr { syntax }),
            SyntaxKind::DotExpr => Expr::DotExpr(DotExpr { syntax }),
            SyntaxKind::CallExpr => Expr::CallExpr(CallExpr { syntax }),
            SyntaxKind::FunctionExpr => Expr::FunctionExpr(FunctionExpr { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Expr::Literal(it) => &it.syntax,
            Expr::TableExpr(it) => &it.syntax,
            Expr::InfixExpr(it) => &it.syntax,
            Expr::PrefixExpr(it) => &it.syntax,
            Expr::IndexExpr(it) => &it.syntax,
            Expr::DotExpr(it) => &it.syntax,
            Expr::CallExpr(it) => &it.syntax,
            Expr::FunctionExpr(it) => &it.syntax,
        }
    }
}
impl From<KeyValue> for TableContent {
    fn from(node: KeyValue) -> TableContent {
        TableContent::KeyValue(node)
    }
}
impl From<PositionalValue> for TableContent {
    fn from(node: PositionalValue) -> TableContent {
        TableContent::PositionalValue(node)
    }
}
impl AstNode for TableContent {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            SyntaxKind::KeyValue | SyntaxKind::PositionalValue => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            SyntaxKind::KeyValue => TableContent::KeyValue(KeyValue { syntax }),
            SyntaxKind::PositionalValue => {
                TableContent::PositionalValue(PositionalValue { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            TableContent::KeyValue(it) => &it.syntax,
            TableContent::PositionalValue(it) => &it.syntax,
        }
    }
}
impl From<Index> for TableKey {
    fn from(node: Index) -> TableKey {
        TableKey::Index(node)
    }
}
impl From<IdentKey> for TableKey {
    fn from(node: IdentKey) -> TableKey {
        TableKey::IdentKey(node)
    }
}
impl AstNode for TableKey {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            SyntaxKind::Index | SyntaxKind::IdentKey => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            SyntaxKind::Index => TableKey::Index(Index { syntax }),
            SyntaxKind::IdentKey => TableKey::IdentKey(IdentKey { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            TableKey::Index(it) => &it.syntax,
            TableKey::IdentKey(it) => &it.syntax,
        }
    }
}
impl From<NumericFor> for ForContent {
    fn from(node: NumericFor) -> ForContent {
        ForContent::NumericFor(node)
    }
}
impl From<GenericFor> for ForContent {
    fn from(node: GenericFor) -> ForContent {
        ForContent::GenericFor(node)
    }
}
impl AstNode for ForContent {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            SyntaxKind::NumericFor | SyntaxKind::GenericFor => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            SyntaxKind::NumericFor => ForContent::NumericFor(NumericFor { syntax }),
            SyntaxKind::GenericFor => ForContent::GenericFor(GenericFor { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            ForContent::NumericFor(it) => &it.syntax,
            ForContent::GenericFor(it) => &it.syntax,
        }
    }
}
impl std::fmt::Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TableContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TableKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for ForContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AssignStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for LocalAssignStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for FunctionDefStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for ForStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for IfStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for ReturnStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for ExprStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for BreakStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for LabelStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for RepeatUntilStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TableExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for InfixExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PrefixExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for IndexExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for DotExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CallExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for FunctionExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TableCallExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for StringCallExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MethodCallExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for NameRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MultivalExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Arglist {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Paramlist {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Body {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for ParenExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for KeyValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PositionalValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Index {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for IdentKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for InfixOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for LabelDelim {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GotoStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for DoStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MultiName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for ElseBranch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for ElseIfBranch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for WhileStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for NumericFor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GenericFor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}

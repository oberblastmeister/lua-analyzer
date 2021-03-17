//! Generated file, do not edit by hand, see `xtask/src/codegen`

#![allow(dead_code)]
use crate::{
    ast::{self, support, AstChildren, AstNode},
    SyntaxKind::{self, *},
    SyntaxNode, SyntaxToken, T,
};
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AssignStmt {
    pub(crate) syntax: SyntaxNode,
}
impl AssignStmt {
    pub fn modifier(&self) -> Option<Modifier> {
        support::child(&self.syntax)
    }
    pub fn pat(&self) -> Option<Pat> {
        support::child(&self.syntax)
    }
    pub fn eq_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![=])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FunctionStmt {
    pub(crate) syntax: SyntaxNode,
}
impl FunctionStmt {
    pub fn modifier(&self) -> Option<Modifier> {
        support::child(&self.syntax)
    }
    pub fn function_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![function])
    }
    pub fn name(&self) -> Option<Name> {
        support::child(&self.syntax)
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
    pub fn for_content(&self) -> Option<ForContent> {
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
    pub fn end_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![end])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LitExpr {
    pub(crate) syntax: SyntaxNode,
}
impl LitExpr {
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
    pub fn name(&self) -> Option<Name> {
        support::child(&self.syntax)
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
pub struct TableItem {
    pub(crate) syntax: SyntaxNode,
}
impl TableItem {
    pub fn expr(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IndexKey {
    pub(crate) syntax: SyntaxNode,
}
impl IndexKey {
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
pub struct LiteralKey {
    pub(crate) syntax: SyntaxNode,
}
impl LiteralKey {
    pub fn ident_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![ident])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InfixOp {
    pub(crate) syntax: SyntaxNode,
}
impl InfixOp {
    pub fn plus_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![+])
    }
    pub fn minus_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![-])
    }
    pub fn slash_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![/])
    }
    pub fn asterisk_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![*])
    }
    pub fn eq_eq_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![==])
    }
    pub fn not_eq_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![~=])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Modifier {
    pub(crate) syntax: SyntaxNode,
}
impl Modifier {
    pub fn local_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![local])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Parameters {
    pub(crate) syntax: SyntaxNode,
}
impl Parameters {
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['('])
    }
    pub fn names(&self) -> AstChildren<Name> {
        support::children(&self.syntax)
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
    pub fn then_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![then])
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
    pub fn ident_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![ident])
    }
    pub fn eq_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![=])
    }
    pub fn start(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }
    pub fn end(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }
    pub fn inc(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GenericFor {
    pub(crate) syntax: SyntaxNode,
}
impl GenericFor {
    pub fn pat(&self) -> Option<Pat> {
        support::child(&self.syntax)
    }
    pub fn in_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![in])
    }
    pub fn expr(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Pat {
    pub(crate) syntax: SyntaxNode,
}
impl Pat {
    pub fn names(&self) -> AstChildren<Name> {
        support::children(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Stmt {
    AssignStmt(AssignStmt),
    FunctionStmt(FunctionStmt),
    ForStmt(ForStmt),
    IfStmt(IfStmt),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expr {
    LitExpr(LitExpr),
    TableExpr(TableExpr),
    InfixExpr(InfixExpr),
    PrefixExpr(PrefixExpr),
    IndexExpr(IndexExpr),
    DotExpr(DotExpr),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TableContent {
    KeyValue(KeyValue),
    TableItem(TableItem),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TableKey {
    IndexKey(IndexKey),
    LiteralKey(LiteralKey),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ForContent {
    NumericFor(NumericFor),
    GenericFor(GenericFor),
}
impl AstNode for AssignStmt {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == AssignStmt
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
impl AstNode for FunctionStmt {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == FunctionStmt
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
        kind == ForStmt
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
        kind == IfStmt
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
impl AstNode for LitExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == LitExpr
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
        kind == TableExpr
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
        kind == InfixExpr
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
        kind == PrefixExpr
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
        kind == IndexExpr
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
        kind == DotExpr
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
        kind == Name
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
        kind == KeyValue
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
impl AstNode for TableItem {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TableItem
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
impl AstNode for IndexKey {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == IndexKey
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
impl AstNode for LiteralKey {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == LiteralKey
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
        kind == InfixOp
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
impl AstNode for Modifier {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == Modifier
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
impl AstNode for Parameters {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == Parameters
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
        kind == Body
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
        kind == WhileStmt
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
        kind == NumericFor
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
        kind == GenericFor
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
impl AstNode for Pat {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == Pat
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
impl From<FunctionStmt> for Stmt {
    fn from(node: FunctionStmt) -> Stmt {
        Stmt::FunctionStmt(node)
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
impl AstNode for Stmt {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            AssignStmt | FunctionStmt | ForStmt | IfStmt => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            AssignStmt => Stmt::AssignStmt(AssignStmt { syntax }),
            FunctionStmt => Stmt::FunctionStmt(FunctionStmt { syntax }),
            ForStmt => Stmt::ForStmt(ForStmt { syntax }),
            IfStmt => Stmt::IfStmt(IfStmt { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Stmt::AssignStmt(it) => &it.syntax,
            Stmt::FunctionStmt(it) => &it.syntax,
            Stmt::ForStmt(it) => &it.syntax,
            Stmt::IfStmt(it) => &it.syntax,
        }
    }
}
impl From<LitExpr> for Expr {
    fn from(node: LitExpr) -> Expr {
        Expr::LitExpr(node)
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
impl AstNode for Expr {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            LitExpr | TableExpr | InfixExpr | PrefixExpr | IndexExpr | DotExpr => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            LitExpr => Expr::LitExpr(LitExpr { syntax }),
            TableExpr => Expr::TableExpr(TableExpr { syntax }),
            InfixExpr => Expr::InfixExpr(InfixExpr { syntax }),
            PrefixExpr => Expr::PrefixExpr(PrefixExpr { syntax }),
            IndexExpr => Expr::IndexExpr(IndexExpr { syntax }),
            DotExpr => Expr::DotExpr(DotExpr { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Expr::LitExpr(it) => &it.syntax,
            Expr::TableExpr(it) => &it.syntax,
            Expr::InfixExpr(it) => &it.syntax,
            Expr::PrefixExpr(it) => &it.syntax,
            Expr::IndexExpr(it) => &it.syntax,
            Expr::DotExpr(it) => &it.syntax,
        }
    }
}
impl From<KeyValue> for TableContent {
    fn from(node: KeyValue) -> TableContent {
        TableContent::KeyValue(node)
    }
}
impl From<TableItem> for TableContent {
    fn from(node: TableItem) -> TableContent {
        TableContent::TableItem(node)
    }
}
impl AstNode for TableContent {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            KeyValue | TableItem => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            KeyValue => TableContent::KeyValue(KeyValue { syntax }),
            TableItem => TableContent::TableItem(TableItem { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            TableContent::KeyValue(it) => &it.syntax,
            TableContent::TableItem(it) => &it.syntax,
        }
    }
}
impl From<IndexKey> for TableKey {
    fn from(node: IndexKey) -> TableKey {
        TableKey::IndexKey(node)
    }
}
impl From<LiteralKey> for TableKey {
    fn from(node: LiteralKey) -> TableKey {
        TableKey::LiteralKey(node)
    }
}
impl AstNode for TableKey {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            IndexKey | LiteralKey => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            IndexKey => TableKey::IndexKey(IndexKey { syntax }),
            LiteralKey => TableKey::LiteralKey(LiteralKey { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            TableKey::IndexKey(it) => &it.syntax,
            TableKey::LiteralKey(it) => &it.syntax,
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
            NumericFor | GenericFor => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            NumericFor => ForContent::NumericFor(NumericFor { syntax }),
            GenericFor => ForContent::GenericFor(GenericFor { syntax }),
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
impl std::fmt::Display for AssignStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for FunctionStmt {
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
impl std::fmt::Display for LitExpr {
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
impl std::fmt::Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for KeyValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TableItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for IndexKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for LiteralKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for InfixOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Modifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Parameters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Body {
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
impl std::fmt::Display for Pat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}

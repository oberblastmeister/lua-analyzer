//! Generated file, do not edit by hand, see `xtask/src/codegen`

use crate::{
    ast::{self, support, AstChildren, AstNode},
    SyntaxKind::{self, *},
    SyntaxNode, SyntaxToken, T,
};
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Assignment {
    pub(crate) syntax: SyntaxNode,
}
impl Assignment {
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
pub struct Function {
    pub(crate) syntax: SyntaxNode,
}
impl Function {
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
pub struct For {
    pub(crate) syntax: SyntaxNode,
}
impl For {
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
pub struct If {
    pub(crate) syntax: SyntaxNode,
}
impl If {
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
pub struct Table {
    pub(crate) syntax: SyntaxNode,
}
impl Table {
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
pub struct Name {
    pub(crate) syntax: SyntaxNode,
}
impl Name {
    pub fn ident_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![ident])
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
    Assignment(Assignment),
    Function(Function),
    For(For),
    If(If),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expr {
    Literal(Literal),
    Table(Table),
    InfixExpr(InfixExpr),
    PrefixExpr(PrefixExpr),
    IndexExpr(IndexExpr),
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
impl AstNode for Assignment {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == Assignment
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
impl AstNode for Function {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == Function
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
impl AstNode for For {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == For
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
impl AstNode for If {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == If
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
        kind == Literal
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
impl AstNode for Table {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == Table
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
impl From<Assignment> for Stmt {
    fn from(node: Assignment) -> Stmt {
        Stmt::Assignment(node)
    }
}
impl From<Function> for Stmt {
    fn from(node: Function) -> Stmt {
        Stmt::Function(node)
    }
}
impl From<For> for Stmt {
    fn from(node: For) -> Stmt {
        Stmt::For(node)
    }
}
impl From<If> for Stmt {
    fn from(node: If) -> Stmt {
        Stmt::If(node)
    }
}
impl AstNode for Stmt {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            Assignment | Function | For | If => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            Assignment => Stmt::Assignment(Assignment { syntax }),
            Function => Stmt::Function(Function { syntax }),
            For => Stmt::For(For { syntax }),
            If => Stmt::If(If { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Stmt::Assignment(it) => &it.syntax,
            Stmt::Function(it) => &it.syntax,
            Stmt::For(it) => &it.syntax,
            Stmt::If(it) => &it.syntax,
        }
    }
}
impl From<Literal> for Expr {
    fn from(node: Literal) -> Expr {
        Expr::Literal(node)
    }
}
impl From<Table> for Expr {
    fn from(node: Table) -> Expr {
        Expr::Table(node)
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
impl AstNode for Expr {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            Literal | Table | InfixExpr | PrefixExpr | IndexExpr => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            Literal => Expr::Literal(Literal { syntax }),
            Table => Expr::Table(Table { syntax }),
            InfixExpr => Expr::InfixExpr(InfixExpr { syntax }),
            PrefixExpr => Expr::PrefixExpr(PrefixExpr { syntax }),
            IndexExpr => Expr::IndexExpr(IndexExpr { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Expr::Literal(it) => &it.syntax,
            Expr::Table(it) => &it.syntax,
            Expr::InfixExpr(it) => &it.syntax,
            Expr::PrefixExpr(it) => &it.syntax,
            Expr::IndexExpr(it) => &it.syntax,
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
impl std::fmt::Display for Assignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for For {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for If {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Table {
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
impl std::fmt::Display for Name {
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

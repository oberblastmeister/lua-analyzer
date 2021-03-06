//! Generated file, do not edit by hand, see `xtask/src/codegen`

#![allow(bad_style, missing_docs, unreachable_pub)]
#[doc = r" The kind of syntax node, e.g. Ident, `UseKw`, or `Struct`."]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(u16)]
pub enum SyntaxKind {
    #[doc(hidden)]
    Tombstone,
    #[doc(hidden)]
    Eof,
    #[doc(hiddent)]
    Unknown,
    Bang,
    Hash,
    Modulo,
    LParen,
    RParen,
    Asterisk,
    Plus,
    Comma,
    Minus,
    Dot,
    DoubleDot,
    TripleDot,
    Slash,
    Colon,
    DoubleColon,
    Semicolon,
    Lt,
    LtEq,
    Eq,
    EqEq,
    Gt,
    GtEq,
    LBracket,
    RBracket,
    Caret,
    LBrace,
    RBrace,
    NotEq,
    FunctionKw,
    LocalKw,
    TrueKw,
    FalseKw,
    IfKw,
    ThenKw,
    ElseKw,
    ElseifKw,
    WhileKw,
    ForKw,
    InKw,
    BreakKw,
    DoKw,
    GotoKw,
    AndKw,
    OrKw,
    NotKw,
    ReturnKw,
    EndKw,
    RepeatKw,
    UntilKw,
    NilKw,
    Number,
    Str,
    True,
    False,
    Error,
    Ident,
    Shebang,
    Name,
    NameRef,
    MultiName,
    SourceFile,
    Block,
    AssignStmt,
    LocalAssignStmt,
    LocalFunctionDefStmt,
    FunctionDefStmt,
    ForStmt,
    IfStmt,
    ReturnStmt,
    WhileStmt,
    ExprStmt,
    BreakStmt,
    RepeatUntilStmt,
    LabelStmt,
    GotoStmt,
    LabelDelim,
    DoStmt,
    ParamList,
    FunctionMethod,
    FunctionStatic,
    IndexPath,
    MultivalExpr,
    ElseBranch,
    ElseIfBranch,
    NumericFor,
    GenericFor,
    Literal,
    TableExpr,
    InfixExpr,
    PrefixExpr,
    IndexExpr,
    DotExpr,
    FunctionExpr,
    CallExpr,
    MethodCallExpr,
    CallArgs,
    ArgList,
    TableSep,
    ParenExpr,
    KeyValue,
    PositionalValue,
    Index,
    IdentKey,
    Stmt,
    StmtItem,
    FunctionDefContent,
    Expr,
    ForContent,
    TableContent,
    TableKey,
    Comment,
    Whitespace,
    #[doc(hidden)]
    __LAST,
}
use self::SyntaxKind::*;
impl SyntaxKind {
    pub fn is_trivia(&self) -> bool {
        matches!(self, Comment | Whitespace)
    }
    pub fn is_keyword(&self) -> bool {
        matches!(
            self,
            FunctionKw
                | LocalKw
                | TrueKw
                | FalseKw
                | IfKw
                | ThenKw
                | ElseKw
                | ElseifKw
                | WhileKw
                | ForKw
                | InKw
                | BreakKw
                | DoKw
                | GotoKw
                | AndKw
                | OrKw
                | NotKw
                | ReturnKw
                | EndKw
                | RepeatKw
                | UntilKw
                | NilKw
        )
    }
    pub fn is_punct(&self) -> bool {
        matches!(
            self,
            Bang | Hash
                | Modulo
                | LParen
                | RParen
                | Asterisk
                | Plus
                | Comma
                | Minus
                | Dot
                | DoubleDot
                | TripleDot
                | Slash
                | Colon
                | DoubleColon
                | Semicolon
                | Lt
                | LtEq
                | Eq
                | EqEq
                | Gt
                | GtEq
                | LBracket
                | RBracket
                | Caret
                | LBrace
                | RBrace
                | NotEq
        )
    }
    pub fn is_literal(&self) -> bool {
        matches!(self, Number | Str | True | False)
    }
    pub fn from_keyword(ident: &str) -> Option<SyntaxKind> {
        let kw = match ident {
            "function" => FunctionKw,
            "local" => LocalKw,
            "true" => TrueKw,
            "false" => FalseKw,
            "if" => IfKw,
            "then" => ThenKw,
            "else" => ElseKw,
            "elseif" => ElseifKw,
            "while" => WhileKw,
            "for" => ForKw,
            "in" => InKw,
            "break" => BreakKw,
            "do" => DoKw,
            "goto" => GotoKw,
            "and" => AndKw,
            "or" => OrKw,
            "not" => NotKw,
            "return" => ReturnKw,
            "end" => EndKw,
            "repeat" => RepeatKw,
            "until" => UntilKw,
            "nil" => NilKw,
            _ => return None,
        };
        Some(kw)
    }
}
#[doc = r" A helper macro to get the token"]
#[macro_export]
macro_rules ! T { [!] => { $ crate :: SyntaxKind :: Bang } ; [#] => { $ crate :: SyntaxKind :: Hash } ; [%] => { $ crate :: SyntaxKind :: Modulo } ; ['('] => { $ crate :: SyntaxKind :: LParen } ; [')'] => { $ crate :: SyntaxKind :: RParen } ; [*] => { $ crate :: SyntaxKind :: Asterisk } ; [+] => { $ crate :: SyntaxKind :: Plus } ; [,] => { $ crate :: SyntaxKind :: Comma } ; [-] => { $ crate :: SyntaxKind :: Minus } ; [.] => { $ crate :: SyntaxKind :: Dot } ; [..] => { $ crate :: SyntaxKind :: DoubleDot } ; [...] => { $ crate :: SyntaxKind :: TripleDot } ; [/] => { $ crate :: SyntaxKind :: Slash } ; [:] => { $ crate :: SyntaxKind :: Colon } ; [::] => { $ crate :: SyntaxKind :: DoubleColon } ; [;] => { $ crate :: SyntaxKind :: Semicolon } ; [<] => { $ crate :: SyntaxKind :: Lt } ; [<=] => { $ crate :: SyntaxKind :: LtEq } ; [=] => { $ crate :: SyntaxKind :: Eq } ; [==] => { $ crate :: SyntaxKind :: EqEq } ; [>] => { $ crate :: SyntaxKind :: Gt } ; [>=] => { $ crate :: SyntaxKind :: GtEq } ; ['['] => { $ crate :: SyntaxKind :: LBracket } ; [']'] => { $ crate :: SyntaxKind :: RBracket } ; [^] => { $ crate :: SyntaxKind :: Caret } ; ['{'] => { $ crate :: SyntaxKind :: LBrace } ; ['}'] => { $ crate :: SyntaxKind :: RBrace } ; [~=] => { $ crate :: SyntaxKind :: NotEq } ; [function] => { $ crate :: SyntaxKind :: FunctionKw } ; [local] => { $ crate :: SyntaxKind :: LocalKw } ; [true] => { $ crate :: SyntaxKind :: TrueKw } ; [false] => { $ crate :: SyntaxKind :: FalseKw } ; [if] => { $ crate :: SyntaxKind :: IfKw } ; [then] => { $ crate :: SyntaxKind :: ThenKw } ; [else] => { $ crate :: SyntaxKind :: ElseKw } ; [elseif] => { $ crate :: SyntaxKind :: ElseifKw } ; [while] => { $ crate :: SyntaxKind :: WhileKw } ; [for] => { $ crate :: SyntaxKind :: ForKw } ; [in] => { $ crate :: SyntaxKind :: InKw } ; [break] => { $ crate :: SyntaxKind :: BreakKw } ; [do] => { $ crate :: SyntaxKind :: DoKw } ; [goto] => { $ crate :: SyntaxKind :: GotoKw } ; [and] => { $ crate :: SyntaxKind :: AndKw } ; [or] => { $ crate :: SyntaxKind :: OrKw } ; [not] => { $ crate :: SyntaxKind :: NotKw } ; [return] => { $ crate :: SyntaxKind :: ReturnKw } ; [end] => { $ crate :: SyntaxKind :: EndKw } ; [repeat] => { $ crate :: SyntaxKind :: RepeatKw } ; [until] => { $ crate :: SyntaxKind :: UntilKw } ; [nil] => { $ crate :: SyntaxKind :: NilKw } ; [number] => { $ crate :: SyntaxKind :: Number } ; [str] => { $ crate :: SyntaxKind :: Str } ; [true] => { $ crate :: SyntaxKind :: True } ; [false] => { $ crate :: SyntaxKind :: False } ; [error] => { $ crate :: SyntaxKind :: Error } ; [ident] => { $ crate :: SyntaxKind :: Ident } ; [shebang] => { $ crate :: SyntaxKind :: Shebang } ; [comment] => { $ crate :: SyntaxKind :: Comment } ; [whitespace] => { $ crate :: SyntaxKind :: Whitespace } ; [__] => { $ crate :: SyntaxKind :: Tombstone } ; [eof] => { $ crate :: SyntaxKind :: Eof } ; [unknown] => { $ crate :: SyntaxKind :: Unknown } ; }
#[doc = r" A helper macro to get the node"]
#[macro_export]
macro_rules ! N { [Name] => { $ crate :: SyntaxKind :: Name } ; [NameRef] => { $ crate :: SyntaxKind :: NameRef } ; [MultiName] => { $ crate :: SyntaxKind :: MultiName } ; [SourceFile] => { $ crate :: SyntaxKind :: SourceFile } ; [Block] => { $ crate :: SyntaxKind :: Block } ; [AssignStmt] => { $ crate :: SyntaxKind :: AssignStmt } ; [LocalAssignStmt] => { $ crate :: SyntaxKind :: LocalAssignStmt } ; [LocalFunctionDefStmt] => { $ crate :: SyntaxKind :: LocalFunctionDefStmt } ; [FunctionDefStmt] => { $ crate :: SyntaxKind :: FunctionDefStmt } ; [ForStmt] => { $ crate :: SyntaxKind :: ForStmt } ; [IfStmt] => { $ crate :: SyntaxKind :: IfStmt } ; [ReturnStmt] => { $ crate :: SyntaxKind :: ReturnStmt } ; [WhileStmt] => { $ crate :: SyntaxKind :: WhileStmt } ; [ExprStmt] => { $ crate :: SyntaxKind :: ExprStmt } ; [BreakStmt] => { $ crate :: SyntaxKind :: BreakStmt } ; [RepeatUntilStmt] => { $ crate :: SyntaxKind :: RepeatUntilStmt } ; [LabelStmt] => { $ crate :: SyntaxKind :: LabelStmt } ; [GotoStmt] => { $ crate :: SyntaxKind :: GotoStmt } ; [LabelDelim] => { $ crate :: SyntaxKind :: LabelDelim } ; [DoStmt] => { $ crate :: SyntaxKind :: DoStmt } ; [ParamList] => { $ crate :: SyntaxKind :: ParamList } ; [FunctionMethod] => { $ crate :: SyntaxKind :: FunctionMethod } ; [FunctionStatic] => { $ crate :: SyntaxKind :: FunctionStatic } ; [IndexPath] => { $ crate :: SyntaxKind :: IndexPath } ; [MultivalExpr] => { $ crate :: SyntaxKind :: MultivalExpr } ; [ElseBranch] => { $ crate :: SyntaxKind :: ElseBranch } ; [ElseIfBranch] => { $ crate :: SyntaxKind :: ElseIfBranch } ; [NumericFor] => { $ crate :: SyntaxKind :: NumericFor } ; [GenericFor] => { $ crate :: SyntaxKind :: GenericFor } ; [Literal] => { $ crate :: SyntaxKind :: Literal } ; [TableExpr] => { $ crate :: SyntaxKind :: TableExpr } ; [InfixExpr] => { $ crate :: SyntaxKind :: InfixExpr } ; [PrefixExpr] => { $ crate :: SyntaxKind :: PrefixExpr } ; [IndexExpr] => { $ crate :: SyntaxKind :: IndexExpr } ; [DotExpr] => { $ crate :: SyntaxKind :: DotExpr } ; [FunctionExpr] => { $ crate :: SyntaxKind :: FunctionExpr } ; [CallExpr] => { $ crate :: SyntaxKind :: CallExpr } ; [MethodCallExpr] => { $ crate :: SyntaxKind :: MethodCallExpr } ; [CallArgs] => { $ crate :: SyntaxKind :: CallArgs } ; [ArgList] => { $ crate :: SyntaxKind :: ArgList } ; [TableSep] => { $ crate :: SyntaxKind :: TableSep } ; [ParenExpr] => { $ crate :: SyntaxKind :: ParenExpr } ; [KeyValue] => { $ crate :: SyntaxKind :: KeyValue } ; [PositionalValue] => { $ crate :: SyntaxKind :: PositionalValue } ; [Index] => { $ crate :: SyntaxKind :: Index } ; [IdentKey] => { $ crate :: SyntaxKind :: IdentKey } ; [Stmt] => { $ crate :: SyntaxKind :: Stmt } ; [StmtItem] => { $ crate :: SyntaxKind :: StmtItem } ; [FunctionDefContent] => { $ crate :: SyntaxKind :: FunctionDefContent } ; [Expr] => { $ crate :: SyntaxKind :: Expr } ; [ForContent] => { $ crate :: SyntaxKind :: ForContent } ; [TableContent] => { $ crate :: SyntaxKind :: TableContent } ; [TableKey] => { $ crate :: SyntaxKind :: TableKey } ; }

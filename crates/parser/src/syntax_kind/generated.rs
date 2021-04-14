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
    Error,
    Ident,
    Shebang,
    Program,
    AssignStmt,
    LocalAssignStmt,
    FunctionDefStmt,
    ForStmt,
    IfStmt,
    ReturnStmt,
    WhileStmt,
    ExprStmt,
    BreakStmt,
    LabelStmt,
    RepeatUntilStmt,
    Literal,
    TableExpr,
    InfixExpr,
    PrefixExpr,
    IndexExpr,
    DotExpr,
    FunctionExpr,
    CallExpr,
    TableCallExpr,
    StringCallExpr,
    MethodCallExpr,
    NameRef,
    MultivalExpr,
    Arglist,
    Paramlist,
    Body,
    ParenExpr,
    KeyValue,
    PositionalValue,
    Index,
    IdentKey,
    LabelDelim,
    Name,
    GotoStmt,
    DoStmt,
    MultiName,
    ElseBranch,
    ElseIfBranch,
    NumericFor,
    GenericFor,
    Stmt,
    Expr,
    TableContent,
    TableKey,
    ForContent,
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
        matches!(self, Number | Str)
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
#[doc = r" A helper macro to get the SyntaxKind"]
#[macro_export]
macro_rules ! T { [!] => { $ crate :: SyntaxKind :: Bang } ; [#] => { $ crate :: SyntaxKind :: Hash } ; [%] => { $ crate :: SyntaxKind :: Modulo } ; ['('] => { $ crate :: SyntaxKind :: LParen } ; [')'] => { $ crate :: SyntaxKind :: RParen } ; [*] => { $ crate :: SyntaxKind :: Asterisk } ; [+] => { $ crate :: SyntaxKind :: Plus } ; [,] => { $ crate :: SyntaxKind :: Comma } ; [-] => { $ crate :: SyntaxKind :: Minus } ; [.] => { $ crate :: SyntaxKind :: Dot } ; [..] => { $ crate :: SyntaxKind :: DoubleDot } ; [...] => { $ crate :: SyntaxKind :: TripleDot } ; [/] => { $ crate :: SyntaxKind :: Slash } ; [:] => { $ crate :: SyntaxKind :: Colon } ; [::] => { $ crate :: SyntaxKind :: DoubleColon } ; [;] => { $ crate :: SyntaxKind :: Semicolon } ; [<] => { $ crate :: SyntaxKind :: Lt } ; [<=] => { $ crate :: SyntaxKind :: LtEq } ; [=] => { $ crate :: SyntaxKind :: Eq } ; [==] => { $ crate :: SyntaxKind :: EqEq } ; [>] => { $ crate :: SyntaxKind :: Gt } ; [>=] => { $ crate :: SyntaxKind :: GtEq } ; ['['] => { $ crate :: SyntaxKind :: LBracket } ; [']'] => { $ crate :: SyntaxKind :: RBracket } ; [^] => { $ crate :: SyntaxKind :: Caret } ; ['{'] => { $ crate :: SyntaxKind :: LBrace } ; ['}'] => { $ crate :: SyntaxKind :: RBrace } ; [~=] => { $ crate :: SyntaxKind :: NotEq } ; [function] => { $ crate :: SyntaxKind :: FunctionKw } ; [local] => { $ crate :: SyntaxKind :: LocalKw } ; [true] => { $ crate :: SyntaxKind :: TrueKw } ; [false] => { $ crate :: SyntaxKind :: FalseKw } ; [if] => { $ crate :: SyntaxKind :: IfKw } ; [then] => { $ crate :: SyntaxKind :: ThenKw } ; [else] => { $ crate :: SyntaxKind :: ElseKw } ; [elseif] => { $ crate :: SyntaxKind :: ElseifKw } ; [while] => { $ crate :: SyntaxKind :: WhileKw } ; [for] => { $ crate :: SyntaxKind :: ForKw } ; [in] => { $ crate :: SyntaxKind :: InKw } ; [break] => { $ crate :: SyntaxKind :: BreakKw } ; [do] => { $ crate :: SyntaxKind :: DoKw } ; [goto] => { $ crate :: SyntaxKind :: GotoKw } ; [and] => { $ crate :: SyntaxKind :: AndKw } ; [or] => { $ crate :: SyntaxKind :: OrKw } ; [not] => { $ crate :: SyntaxKind :: NotKw } ; [return] => { $ crate :: SyntaxKind :: ReturnKw } ; [end] => { $ crate :: SyntaxKind :: EndKw } ; [repeat] => { $ crate :: SyntaxKind :: RepeatKw } ; [until] => { $ crate :: SyntaxKind :: UntilKw } ; [nil] => { $ crate :: SyntaxKind :: NilKw } ; [number] => { $ crate :: SyntaxKind :: Number } ; [str] => { $ crate :: SyntaxKind :: Str } ; [error] => { $ crate :: SyntaxKind :: Error } ; [ident] => { $ crate :: SyntaxKind :: Ident } ; [shebang] => { $ crate :: SyntaxKind :: Shebang } ; [comment] => { $ crate :: SyntaxKind :: Comment } ; [whitespace] => { $ crate :: SyntaxKind :: Whitespace } ; [__] => { $ crate :: SyntaxKind :: Tombstone } ; [eof] => { $ crate :: SyntaxKind :: Eof } ; [unknown] => { $ crate :: SyntaxKind :: Unknown } ; }

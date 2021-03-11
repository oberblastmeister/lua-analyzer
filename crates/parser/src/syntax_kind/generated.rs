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
    LParen,
    RParen,
    Asterisk,
    Plus,
    Comma,
    Minus,
    Dot,
    Slash,
    Colon,
    Semicolon,
    Eq,
    EqEq,
    LBracket,
    RBracket,
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
    WhileKw,
    ForKw,
    BreakKw,
    DoKw,
    GotoKw,
    AndKw,
    OrKw,
    ReturnKw,
    NumberLit,
    StringLit,
    Error,
    Ident,
    Shebang,
    Assignment,
    Function,
    For,
    If,
    Literal,
    Table,
    InfixExpr,
    PrefixExpr,
    IndexExpr,
    KeyValue,
    TableItem,
    IndexKey,
    LiteralKey,
    InfixOp,
    Modifier,
    Name,
    Parameters,
    Body,
    WhileStmt,
    NumericFor,
    GenericFor,
    Pat,
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
                | WhileKw
                | ForKw
                | BreakKw
                | DoKw
                | GotoKw
                | AndKw
                | OrKw
                | ReturnKw
        )
    }
    pub fn is_punct(&self) -> bool {
        matches!(
            self,
            LParen
                | RParen
                | Asterisk
                | Plus
                | Comma
                | Minus
                | Dot
                | Slash
                | Colon
                | Semicolon
                | Eq
                | EqEq
                | LBracket
                | RBracket
                | LBrace
                | RBrace
                | NotEq
        )
    }
    pub fn is_literal(&self) -> bool {
        matches!(self, NumberLit | StringLit)
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
            "while" => WhileKw,
            "for" => ForKw,
            "break" => BreakKw,
            "do" => DoKw,
            "goto" => GotoKw,
            "and" => AndKw,
            "or" => OrKw,
            "return" => ReturnKw,
            _ => return None,
        };
        Some(kw)
    }
}
#[doc = r" A helper macro to get the SyntaxKind"]
#[macro_export]
macro_rules ! T { ['('] => { $ crate :: SyntaxKind :: LParen } ; [')'] => { $ crate :: SyntaxKind :: RParen } ; [*] => { $ crate :: SyntaxKind :: Asterisk } ; [+] => { $ crate :: SyntaxKind :: Plus } ; [,] => { $ crate :: SyntaxKind :: Comma } ; [-] => { $ crate :: SyntaxKind :: Minus } ; [.] => { $ crate :: SyntaxKind :: Dot } ; [/] => { $ crate :: SyntaxKind :: Slash } ; [:] => { $ crate :: SyntaxKind :: Colon } ; [;] => { $ crate :: SyntaxKind :: Semicolon } ; [=] => { $ crate :: SyntaxKind :: Eq } ; [==] => { $ crate :: SyntaxKind :: EqEq } ; ['['] => { $ crate :: SyntaxKind :: LBracket } ; [']'] => { $ crate :: SyntaxKind :: RBracket } ; ['{'] => { $ crate :: SyntaxKind :: LBrace } ; ['}'] => { $ crate :: SyntaxKind :: RBrace } ; [~=] => { $ crate :: SyntaxKind :: NotEq } ; [function] => { $ crate :: SyntaxKind :: FunctionKw } ; [local] => { $ crate :: SyntaxKind :: LocalKw } ; [true] => { $ crate :: SyntaxKind :: TrueKw } ; [false] => { $ crate :: SyntaxKind :: FalseKw } ; [if] => { $ crate :: SyntaxKind :: IfKw } ; [then] => { $ crate :: SyntaxKind :: ThenKw } ; [else] => { $ crate :: SyntaxKind :: ElseKw } ; [while] => { $ crate :: SyntaxKind :: WhileKw } ; [for] => { $ crate :: SyntaxKind :: ForKw } ; [break] => { $ crate :: SyntaxKind :: BreakKw } ; [do] => { $ crate :: SyntaxKind :: DoKw } ; [goto] => { $ crate :: SyntaxKind :: GotoKw } ; [and] => { $ crate :: SyntaxKind :: AndKw } ; [or] => { $ crate :: SyntaxKind :: OrKw } ; [return] => { $ crate :: SyntaxKind :: ReturnKw } ; [Number] => { $ crate :: SyntaxKind :: NumberLit } ; [String] => { $ crate :: SyntaxKind :: StringLit } ; [error] => { $ crate :: SyntaxKind :: Error } ; [ident] => { $ crate :: SyntaxKind :: Ident } ; [shebang] => { $ crate :: SyntaxKind :: Shebang } ; [comment] => { $ crate :: SyntaxKind :: Comment } ; [whitespace] => { $ crate :: SyntaxKind :: Whitespace } ; [__] => { $ crate :: SyntaxKind :: Tombstone } ; [eof] => { $ crate :: SyntaxKind :: Eof } ; }

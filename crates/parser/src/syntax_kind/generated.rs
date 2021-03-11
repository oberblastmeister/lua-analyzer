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
    EqEq,
    NotEq,
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Comma,
    Dot,
    Semicolon,
    Colon,
    Plus,
    Minus,
    Asterisk,
    Slash,
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
    Whitespace,
    Comment,
    Shebang,
    #[doc(hidden)]
    __LAST,
}
#[doc = r" A helper macro to get the SyntaxKind"]
#[macro_export]
macro_rules ! T { [==] => { $ crate :: SyntaxKind :: EqEq } ; [~=] => { $ crate :: SyntaxKind :: NotEq } ; ['('] => { $ crate :: SyntaxKind :: LParen } ; [')'] => { $ crate :: SyntaxKind :: RParen } ; ['{'] => { $ crate :: SyntaxKind :: LBrace } ; ['}'] => { $ crate :: SyntaxKind :: RBrace } ; ['['] => { $ crate :: SyntaxKind :: LBracket } ; [']'] => { $ crate :: SyntaxKind :: RBracket } ; [,] => { $ crate :: SyntaxKind :: Comma } ; [.] => { $ crate :: SyntaxKind :: Dot } ; [;] => { $ crate :: SyntaxKind :: Semicolon } ; [:] => { $ crate :: SyntaxKind :: Colon } ; [+] => { $ crate :: SyntaxKind :: Plus } ; [-] => { $ crate :: SyntaxKind :: Minus } ; [*] => { $ crate :: SyntaxKind :: Asterisk } ; [/] => { $ crate :: SyntaxKind :: Slash } ; [function] => { $ crate :: SyntaxKind :: FunctionKw } ; [local] => { $ crate :: SyntaxKind :: LocalKw } ; [true] => { $ crate :: SyntaxKind :: TrueKw } ; [false] => { $ crate :: SyntaxKind :: FalseKw } ; [if] => { $ crate :: SyntaxKind :: IfKw } ; [then] => { $ crate :: SyntaxKind :: ThenKw } ; [else] => { $ crate :: SyntaxKind :: ElseKw } ; [while] => { $ crate :: SyntaxKind :: WhileKw } ; [for] => { $ crate :: SyntaxKind :: ForKw } ; [break] => { $ crate :: SyntaxKind :: BreakKw } ; [do] => { $ crate :: SyntaxKind :: DoKw } ; [goto] => { $ crate :: SyntaxKind :: GotoKw } ; [and] => { $ crate :: SyntaxKind :: AndKw } ; [or] => { $ crate :: SyntaxKind :: OrKw } ; [return] => { $ crate :: SyntaxKind :: ReturnKw } ; [Number] => { $ crate :: SyntaxKind :: NumberLit } ; [String] => { $ crate :: SyntaxKind :: StringLit } ; [ident] => { $ crate :: SyntaxKind :: Ident } ; [__] => { $ crate :: SyntaxKind :: Tombstone } ; [eof] => { $ crate :: SyntaxKind :: Eof } ; }

mod accept;
pub mod ast;
#[doc(hidden)]
pub mod fuzz;
mod lexer;
mod parsing;
mod ptr;
mod syntax_node;
#[cfg(test)]
mod tests;
mod token_text;
mod validation;

pub use parser::{SyntaxKind, Token, T, N};
pub use lexer::{tokenize, tokenize_iter, lex_first_syntax_kind};
pub use ptr::{AstPtr, SyntaxNodePtr};
pub use rowan::{TextRange, TextSize, WalkEvent};
pub use syntax_node::{
    SyntaxElement, SyntaxElementChildren, SyntaxError, SyntaxNode, SyntaxNodeChildren, SyntaxToken,
};
use text_edit::Indel;
pub use token_text::TokenText;

use std::{marker::PhantomData, sync::Arc};

use ast::{AstNode, ParamList};
use rowan::GreenNode;

/// `Parse` is the result of the parsing: a syntax tree and a collection of
/// errors.
///
/// Note that we always produce a syntax tree, even for completely invalid
/// files.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Parse<T> {
    green: GreenNode,
    errors: Arc<Vec<SyntaxError>>,
    _ty: PhantomData<fn() -> T>,
}

impl<T> Parse<T> {
    fn new(green: GreenNode, errors: Vec<SyntaxError>) -> Parse<T> {
        Parse { green, errors: Arc::new(errors), _ty: PhantomData }
    }

    pub fn syntax_node(&self) -> SyntaxNode {
        SyntaxNode::new_root(self.green.clone())
    }
}

impl<T: AstNode> Parse<T> {
    pub fn to_syntax(self) -> Parse<SyntaxNode> {
        Parse { green: self.green, errors: self.errors, _ty: PhantomData }
    }

    pub fn tree(&self) -> T {
        T::cast(self.syntax_node()).unwrap()
    }

    pub fn errors(&self) -> &[SyntaxError] {
        &*self.errors
    }

    pub fn ok(self) -> Result<T, Arc<Vec<SyntaxError>>> {
        if self.errors.is_empty() {
            Ok(self.tree())
        } else {
            Err(self.errors)
        }
    }
}

impl Parse<SyntaxNode> {
    pub fn cast<N: AstNode>(self) -> Option<Parse<N>> {
        if N::cast(self.syntax_node()).is_some() {
            Some(Parse { green: self.green, errors: self.errors, _ty: PhantomData })
        } else {
            None
        }
    }
}

fn format_errors(errors: &[SyntaxError]) -> String {
    let mut s = String::new();
    s.push_str(
        "
=============================
Errors:
=============================",
    );
    s.push('\n');
    s.push_str(&format!("{:#?}", errors));
    s
}

impl ast::Expr {
    pub fn is_call(&self) -> bool {
        matches!(self, ast::Expr::CallExpr(_) | ast::Expr::MethodCallExpr(_))
    }
}

pub use ast::SourceFile;

impl SourceFile {
    pub fn parse(text: &str) -> Parse<SourceFile> {
        let (green, mut errors) = parsing::parse_text(text);
        let root = SyntaxNode::new_root(green.clone());

        errors.extend(validation::validate(&root));

        assert_eq!(root.kind(), SyntaxKind::SourceFile);

        Parse { green, errors: Arc::new(errors), _ty: PhantomData }
    }
}

impl Parse<SourceFile> {
    pub fn reparse(&self, indel: &Indel) -> Parse<SourceFile> {
        self.incremental_reparse(indel).unwrap_or_else(|| self.full_reparse(indel))
    }

    fn incremental_reparse(&self, indel: &Indel) -> Option<Parse<SourceFile>> {
        // FIXME: validation errors are not handled here
        parsing::incremental_reparse(self.tree().syntax(), indel, self.errors.to_vec()).map(
            |(green_node, errors, _reparsed_range)| Parse {
                green: green_node,
                errors: Arc::new(errors),
                _ty: PhantomData,
            },
        )
    }
    fn full_reparse(&self, indel: &Indel) -> Parse<SourceFile> {
        let mut text = self.tree().syntax().text().to_string();
        indel.apply(&mut text);
        SourceFile::parse(&text)
    }

    pub fn debug_dump(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!("{:#?}", self.syntax_node()));
        s.push_str(&format_errors(self.errors()));
        s
    }
}

/// Matches a `SyntaxNode` against an `ast` type.
///
/// # Example:
///
/// ```ignore
/// match_ast! {
///     match node {
///         ast::CallExpr(it) => { ... },
///         ast::MethodCallExpr(it) => { ... },
///         ast::AssignStmt(it) => { ... },
///         _ => None,
///     }
/// }
/// ```
///
/// ```ignore
/// match_ast! {
///     match node {
///         ast::CallExpr(it) | ast::MethodCallExpr(it) | ast::AssignStmt(it) => { ... }
///     }
/// }
/// ```
#[macro_export]
macro_rules! match_ast {
    (match $node:ident { $($tt:tt)* }) => { match_ast!(match ($node) { $($tt)* }) };

    (match ($node:expr) {
        $( ast::$ast:ident($it:ident) => $res:expr, )*
        _ => $catch_all:expr $(,)?
    }) => {{
        $( if let Some($it) = ast::$ast::cast($node.clone()) { $res } else )*
        { $catch_all }
    }};

    (match ($node:expr) {
        $( ast::$ast:ident($it:ident) )|* => $res:expr,
        _ => $catch_all:expr $(,)?
    }) => {{
        $( if let Some($it) = ast::$ast::cast($node.clone()) { $res } else )*
        { $catch_all }
    }};

    (match ($node:expr) {
        $( ast::$ast:ident(_) )|* => $res:expr,
        _ => $catch_all:expr $(,)?
    }) => {{
        $( if let Some(_) = ast::$ast::cast($node.clone()) { $res } else )*
        { $catch_all }
    }};
}

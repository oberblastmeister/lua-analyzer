pub mod ast;
#[doc(hidden)]
pub mod fuzz;
mod accept;
mod lexer;
mod parsing;
mod syntax_node;
#[cfg(test)]
mod tests;
mod validation;
mod ptr;

pub use ptr::{SyntaxNodePtr, AstPtr};
pub use parser::{SyntaxKind, Token, T};
pub use syntax_node::{
    SyntaxElement, SyntaxElementChildren, SyntaxError, SyntaxNode, SyntaxNodeChildren, SyntaxToken,
};
pub use rowan::{TextSize, TextRange, WalkEvent};

use std::{marker::PhantomData, sync::Arc};

use ast::AstNode;
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
        Parse {
            green,
            errors: Arc::new(errors),
            _ty: PhantomData,
        }
    }

    pub fn syntax_node(&self) -> SyntaxNode {
        SyntaxNode::new_root(self.green.clone())
    }
}

impl<T: AstNode> Parse<T> {
    pub fn to_syntax(self) -> Parse<SyntaxNode> {
        Parse {
            green: self.green,
            errors: self.errors,
            _ty: PhantomData,
        }
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
            Some(Parse {
                green: self.green,
                errors: self.errors,
                _ty: PhantomData,
            })
        } else {
            None
        }
    }
}

pub use ast::SourceFile;

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
        matches!(
            self,
            ast::Expr::CallExpr(_)
                | ast::Expr::MethodCallExpr(_)
                | ast::Expr::StringCallExpr(_)
                | ast::Expr::TableCallExpr(_),
        )
    }
}

impl SourceFile {
    pub fn parse(text: &str) -> Parse<SourceFile> {
        let (green, mut errors) = parsing::parse_text(text);
        let root = SyntaxNode::new_root(green.clone());

        errors.extend(validation::validate(&root));

        assert_eq!(root.kind(), SyntaxKind::SourceFile);

        Parse {
            green,
            errors: Arc::new(errors),
            _ty: PhantomData,
        }
    }
}

impl Parse<SourceFile> {
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
        $( ast::$ast:ident($it:ident) => $res:tt $(,)? )*
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

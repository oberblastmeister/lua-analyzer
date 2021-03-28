pub mod ast;
mod parsing;
mod syntax_node;

use std::{marker::PhantomData, sync::Arc};

use ast::AstNode;
pub use parser::{SyntaxKind, T};
use rowan::GreenNode;

pub use syntax_node::{
    SyntaxElement, SyntaxElementChildren, SyntaxError, SyntaxNode, SyntaxNodeChildren, SyntaxToken,
};

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

use ast::Program;

impl Program {
    pub fn parse(text: &str) -> Parse<Program> {
        let (green, errors) = parsing::parse_text(text);
        let root = SyntaxNode::new_root(green.clone());

        assert_eq!(root.kind(), SyntaxKind::Program);
        Parse {
            green,
            errors: Arc::new(errors),
            _ty: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn syntax() {
        insta::glob!("snapshot_inputs/parser/*.txt", |path| {
            let input = fs::read_to_string(path).unwrap();
            let suffix = path.file_stem().unwrap().to_str().unwrap();
            insta::with_settings!({snapshot_suffix => suffix}, {
                insta::assert_debug_snapshot!(Program::parse(&input))
            })
        })
    }
}

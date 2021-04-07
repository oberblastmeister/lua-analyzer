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
    use core::fmt;
    use std::fs;

    fn format_errors(errors: &[SyntaxError]) -> String {
        let mut s = String::new();
        s.push_str(
            "
=============================
Errors:
=============================
",
        );
        s.push_str(&format!("{:#?}", errors));
        s
    }

    fn dump_parse<T: AstNode + fmt::Debug>(parse: Parse<T>) -> String {
        let mut s = String::new();
        s.push_str(&format!("{:#?}", parse.syntax_node()));
        s.push_str(
            "
=============================
Errors:
=============================
",
        );
        s.push_str(&format!("{:#?}", parse.errors()));
        s
    }

    fn dump_parse_no_errors<T: AstNode + fmt::Debug>(parse: Parse<T>) -> String {
        let s = format!("{:#?}", parse.syntax_node());
        if !parse.errors().is_empty() {
            panic!("Should not have any errors {}", format_errors(parse.errors()))
        }
        s
    }

    macro_rules! test_success {
        ($name:literal) => {
            #[test]
            fn $name() {
                insta::with_settings!(
                    {
                        snapshot_suffix => $name,
                        snapshot_path => concat!("snapshots/successes", $name, ".lua"),
                    },
                    {
                        insta::assert_snapshot!(dump_parse_no_errors(Program::parse(&input)))
                    }
                )
            }
        };
    }

    macro_rules! test_fail {
        ($path:literal) => {
            insta::with_settings!(
                {
                    snapshot_suffix => suffix,
                    snapshot_path => concat!("snapshots/fails", $path),
                },
                {
                    insta::assert_snapshot!(dump_parse_no_errors(Program::parse(&input)))
                }
            )
        };
    }

    #[test]
    fn successes() {
        insta::glob!("snapshot_inputs/successes/*.lua", |path| {
            let input = fs::read_to_string(path).unwrap();
            let suffix = path.file_stem().unwrap().to_str().unwrap();
            insta::with_settings!(
                {
                    snapshot_suffix => suffix,
                    snapshot_path => "snapshots/successes",
                },
                {
                    insta::assert_snapshot!(dump_parse_no_errors(Program::parse(&input)))
                }
            )
        })
    }

    #[test]
    fn fails() {
        insta::glob!("snapshot_inputs/fails/*.lua", |path| {
            let input = fs::read_to_string(path).unwrap();
            let suffix = path.file_stem().unwrap().to_str().unwrap();
            insta::with_settings!(
                {
                    snapshot_suffix => suffix,
                    snapshot_path => "snapshots/fails",
                },
                {
                    insta::assert_snapshot!(dump_parse(Program::parse(&input)))
                }
            )
        })
    }

    #[test]
    fn nothing() {
        insta::assert_debug_snapshot!(Program::parse("").syntax_node(), @"Program@0..0
");
    }
}

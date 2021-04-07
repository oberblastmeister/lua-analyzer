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
            panic!(
                "Should not have any errors {}",
                format_errors(parse.errors())
            )
        }
        s
    }

    macro_rules! get_insta_path {
        ($dir:literal, $name:ident, $closure:expr) => {{
            let single_path_glob =
                concat!("snapshot_inputs/", $dir, "/", stringify!($name), ".lua");
            insta::glob!(single_path_glob, $closure)
        }};
    }

    macro_rules! do_test {
        ("successes", $name:ident) => {
            #[test]
            fn $name() {
                get_insta_path!("successes", $name, |path| {
                    use std::fs;

                    let input = fs::read_to_string(path)
                        .unwrap_or_else(|_| panic!("Failed to read path {} to a string", path.display()));
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
        };
        ("fails", $name:ident) => {
            #[test]
            fn $name() {
                get_insta_path!("fails", $name, |path| {
                    use std::fs;

                    let input = fs::read_to_string(path)
                        .unwrap_or_else(|_| panic!("Failed to read path {} to a string", path.display()));
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
        }
    }

    macro_rules! test_successes {
        ($($name:ident),+ $(,)?) => {
            $(
                do_test!("successes", $name);
             )+
        };
    }

    macro_rules! test_fails {
        ($($name:ident),+ $(,)?) => {
            $(
                do_test!("fails", $name);
             )+
        };
    }

    test_successes![
        assign,
        comments,
        function_call,
        function_call_multi,
        function_def,
        function_literal,
        higher_order_function,
        simple_math,
    ];

    test_fails![can_call, cannot_call_literal, missing_paren];

    #[test]
    fn nothing() {
        insta::assert_debug_snapshot!(Program::parse("").syntax_node(), @"Program@0..0
");
    }
}

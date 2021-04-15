use std::str::{CharIndices, Chars};

use rowan::{TextLen, TextRange, TextSize};

use crate::{
    ast::{self, AstNode},
    match_ast, SyntaxError, SyntaxNode, T,
};

/// A helper macro to like the ? operator but pushes to acc when there is an error.
macro_rules! try_acc {
    ($acc:expr, $expr:expr $(,)?) => {
        match $expr {
            Ok(it) => it,
            Err(e) => {
                $acc.push(e);
                return;
            }
        }
    };
}

pub(crate) fn validate(root: &SyntaxNode) -> Vec<SyntaxError> {
    let mut errors = Vec::new();
    let acc = &mut errors;

    for node in root.descendants() {
        println!("Got node {:#?}", node);
        // NOTE: make sure not to match on enums because they will always cast properly
        // and therefore they will always match, making stuff under unreachable
        match_ast! {
            match node {
                ast::ExprStmt(it) |
                ast::Literal(it)
                    | ast::AssignStmt(it)
                    => {
                        println!("Validating {:#?}", it);
                        it.validate(acc)
                    },
                _ => (),
            }
        }
    }

    errors
}

/// A trait to validate parts of an ast
trait Validate: AstNode {
    /// Validate this part of the ast
    fn validate(self, acc: &mut Vec<SyntaxError>);
}

impl Validate for ast::ExprStmt {
    fn validate(self, acc: &mut Vec<SyntaxError>) {
        let multival_expr = try_acc!(
            acc,
            self.multival_expr().ok_or_else(|| SyntaxError::new(
                "Must have an expression".to_string(),
                self.range()
            ))
        );

        let expr_count = multival_expr
            .exprs()
            .map(|expr| {
                if !expr.is_call() {
                    acc.push(SyntaxError::new(
                        "Expression statements can only be call expressions".to_string(),
                        expr.range(),
                    ));
                }
            })
            .count();

        if expr_count != 1 {
            acc.push(SyntaxError::new(
                format!(
                    "Only one expression can be present in an expression statement, got {}",
                    expr_count
                ),
                self.range(),
            ));
        }
    }
}

impl Validate for ast::AssignStmt {
    fn validate(self, acc: &mut Vec<SyntaxError>) {
        let lhs = try_acc!(
            acc,
            self.lhs().ok_or_else(|| SyntaxError::new(
                "Must have at least one expression for the left hand side".to_string(),
                self.range()
            ))
        );

        let rhs = try_acc!(
            acc,
            self.rhs().ok_or_else(|| SyntaxError::new(
                "Must have a least one expression for the right hand side".to_string(),
                self.range()
            ))
        );

        for expr in lhs.exprs() {
            match expr {
                ast::Expr::CallExpr(_)
                | ast::Expr::TableCallExpr(_)
                | ast::Expr::StringCallExpr(_)
                | ast::Expr::MethodCallExpr(_)
                | ast::Expr::DotExpr(_)
                | ast::Expr::IndexExpr(_) => (),
                _ => acc.push(SyntaxError::new(
                    "Can only assign to a function call or index expression".to_string(),
                    expr.range(),
                )),
            }
        }
    }
}

impl Validate for ast::Expr {
    fn validate(self, acc: &mut Vec<SyntaxError>) {
        // TODO: should we validate if we can call literals
    }
}

impl Validate for ast::Literal {
    fn validate(self, acc: &mut Vec<SyntaxError>) {
        let token = self.token();
        let text = token.text();
        match self.kind() {
            ast::LiteralKind::Str(s) => {
                let (start, unquoted) = unquote(&text).expect("String was not properly quoted");
                unescape(unquoted, start, acc)
            }
            ast::LiteralKind::Number(_) => (),
            ast::LiteralKind::Bool(_) => (),
        }
    }
}

fn unquote(text: &str) -> Option<(TextSize, &str)> {
    let delimit = text.chars().next()?;

    if delimit == '\'' || delimit == '\"' {
        assert!(text.ends_with(delimit));
        text.get(1..text.len() - 1).map(|it| (1.into(), it))
    } else if delimit == '[' {
        todo!();
    } else {
        None
    }
}

const ESCAPE_MSG: &str = "Invalid escape sequence";

fn unescape(s: &str, start: TextSize, acc: &mut Vec<SyntaxError>) {
    let text_size = s.text_len();
    let mut chars = s.chars();

    // to avoid borrowing issues
    while let Some(c) = chars.next() {
        unescape_once(c, &mut chars, text_size, start, acc);
    }
}

fn unescape_once(
    first: char,
    chars: &mut Chars<'_>,
    text_size: TextSize,
    offset: TextSize,
    acc: &mut Vec<SyntaxError>,
) {
    let pos = text_size - chars.as_str().text_len() - first.text_len();

    let make_range = |second: Option<TextSize>| {
        TextRange::at(pos, TextSize::from(1) + second.unwrap_or(0.into())) + offset
    };

    if first == '\\' {
        match chars.next() {
            None => acc.push(SyntaxError::new(ESCAPE_MSG.to_string(), make_range(None))),
            Some(second) => match second {
                'a' | 'b' | 'f' | 'n' | 'r' | 't' | 'v' | '\\' | '\"' | '\'' | '[' | ']' => (),
                _ => acc.push(SyntaxError::new(
                    ESCAPE_MSG.to_string(),
                    make_range(Some(second.text_len())),
                )),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::array;

    fn check_unescape_once(s: &str, expected_errors: Vec<SyntaxError>) {
        let mut chars = s.chars();
        let first = chars.next().unwrap();
        let mut actual_errors = Vec::new();
        unescape_once(
            first,
            &mut chars,
            s.text_len(),
            0.into(),
            &mut actual_errors,
        );
        assert_eq!(expected_errors, actual_errors);
    }

    fn check_unescape_str(s: &str, expected_ranges: Vec<TextRange>) {
        let mut actual_errors = vec![];
        unescape(s, 0.into(), &mut actual_errors);
        let actual_ranges = actual_errors
            .into_iter()
            .map(|e| e.range())
            .collect::<Vec<_>>();
        assert_eq!(actual_ranges, expected_ranges);
    }

    #[test]
    fn no_escapes() {
        check_unescape_once("a", vec![]);
    }

    #[test]
    fn valid_escapes() {
        let escapes = [
            r"\a", r"\b", r"\f", r"\n", r"\r", r"\t", r"\v", r"\\", r#"\""#, r"\'", r"\[", r"\]",
        ];

        for escape in escapes.iter() {
            check_unescape_once(escape, vec![]);
        }
    }

    #[test]
    fn text_range_smoke() {
        let res = TextRange::new(3.into(), 9.into()) + TextSize::from(4);
        assert_eq!(TextRange::new(7.into(), 13.into()), res)
    }

    #[test]
    fn invalid_escapes() {
        let one = TextRange::new(0.into(), 1.into());
        let two = TextRange::new(0.into(), 2.into());

        let escapes = [
            (r"\", one),
            (r"\w", two),
            (r"\u", two),
            (r"\p", two),
            (r"\l", two),
            (r"\{", two),
        ];

        for (escape, range) in escapes.iter() {
            check_unescape_once(
                escape,
                vec![SyntaxError::new(ESCAPE_MSG.to_string(), *range)],
            );
        }
    }

    #[test]
    fn escape_nothing() {
        check_unescape_str("", vec![])
    }

    #[test]
    fn valid_escapes_string() {
        let strs = [r"\a aasdfas\tas asdf\r\n", r#"\t\\asdf"#, r"\[\]as"];

        for s in strs.iter() {
            check_unescape_str(s, vec![])
        }
    }

    #[test]
    fn invalid_escapes_string() {
        macro_rules! r {
            ($s:expr, $e:expr) => {
                TextRange::new($s.into(), $e.into())
            };
        }

        let strs = [
            (r"\uasd\qs\", vec![r!(0, 2), r!(5, 7), r!(8, 9)]),
            (r"\", vec![r!(0, 1)]),
            (r"  ads\w", vec![r!(5, 7)]),
        ];

        for (s, ranges) in array::IntoIter::new(strs) {
            check_unescape_str(s, ranges);
        }
    }

    #[test]
    fn smoke_escape() {
        let mut chars = r"\w\t".chars();
        assert_eq!(chars.next().unwrap(), '\\');
        assert_eq!(chars.next().unwrap(), 'w');
        assert_eq!(chars.next().unwrap(), '\\');
        assert_eq!(chars.next().unwrap(), 't');
    }

    #[test]
    fn unquote_str() {
        assert_eq!(unquote(r#""a string""#), Some((1.into(), "a string")));
        assert_eq!(unquote(r#""""#), Some((1.into(), "")));
        assert_eq!(unquote(r"'a string'"), Some((1.into(), "a string")));
        assert_eq!(unquote(r"''"), Some((1.into(), "")));
        assert_eq!(unquote(r""), None);
        assert_eq!(unquote(r"'"), None);
    }
}

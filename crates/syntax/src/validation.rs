use crate::{
    ast::{self, AstNode},
    match_ast, SyntaxError, SyntaxNode,
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
        match_ast! {
            match node {
                ast::ExprStmt(it)
                    | ast::LocalAssignStmt(it)
                    | ast::AssignStmt(it)
                    | ast::Literal(it) => it.validate(acc),
                _ => (),
            }
        }
    }

    errors
}

pub trait Validate {
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

impl Validate for ast::LocalAssignStmt {
    fn validate(self, acc: &mut Vec<SyntaxError>) {
        let multi_name = try_acc!(
            acc,
            self.multi_name().ok_or_else(|| SyntaxError::new(
                "Must have at least one name".to_string(),
                self.range()
            ))
        );

        let multi_expr = if let Some(it) = self.multival_expr() {
            it
        } else {
            return;
        };

        if self.eq_token().is_none() {
            return;
        }

        let names_len = multi_name.names().count();
        let exprs_len = multi_expr.exprs().count();

        if exprs_len != names_len {
            acc.push(SyntaxError::new(
                format!(
                    "The multival length was not the same as the amount of names, got {}",
                    names_len
                ),
                multi_name.range(),
            ));

            acc.push(SyntaxError::new(
                format!(
                    "The multival length was not the same as the amount of names, got {}",
                    exprs_len
                ),
                multi_expr.range(),
            ))
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

        let lhs_count = lhs.exprs().map(|expr| {
            let node = expr.syntax();
            match_ast! {
                match node {
                    ast::CallExpr(_)
                        | ast::TableCallExpr(_)
                        | ast::StringCallExpr(_)
                        | ast::MethodCallExpr(_)
                        | ast::DotExpr(_)
                        | ast::IndexExpr(_) => (),
                    _ => acc.push(SyntaxError::new("Can only assign to a function call or index expression".to_string(), expr.range()))
                }
            }
        }).count();

        let rhs_count = rhs.exprs().count();

        if lhs_count != rhs_count {
            let mut add_error = |multival: ast::MultivalExpr| {
                acc.push(SyntaxError::new("The multival length was not the same for the right and the left side of assignment".to_string(), multival.range()))
            };

            add_error(lhs);
            add_error(rhs);
        }
    }
}

impl Validate for ast::Literal {
    fn validate(self, acc: &mut Vec<SyntaxError>) {
        // TODO: handle escaped strings
    }
}

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
        // TODO: handle escaped strings
    }
}

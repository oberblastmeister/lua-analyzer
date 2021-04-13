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
                    | ast::AssignStmt(it) => it.validate(acc),
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
        let expr = self.expr().expect("Should be expression");
        match expr {
            ast::Expr::CallExpr(_) => (),
            _ => acc.push(SyntaxError::new(
                "Expression statements can only be call expressions".to_string(),
                expr.range(),
            )),
        }
    }
}

impl Validate for ast::LocalAssignStmt {
    fn validate(self, acc: &mut Vec<SyntaxError>) {
        let multi_name = try_acc!(
            acc,
            self.multi_name().ok_or(SyntaxError::new(
                "Must have at least one name".to_string(),
                self.range()
            ))
        );

        let multi_expr = try_acc!(
            acc,
            self.multival_expr().ok_or(SyntaxError::new(
                "Must have a least one expression".to_string(),
                self.range(),
            ))
        );

        let names_len = multi_name.names().count();
        let exprs_len = multi_expr.exprs().count();

        if exprs_len != names_len {
            acc.push(SyntaxError::new(
                "The multival length was not the same as the amount of names".to_string(),
                multi_expr.range(),
            ))
        }
    }
}

impl Validate for ast::AssignStmt {
    fn validate(self, acc: &mut Vec<SyntaxError>) {
        let multival_expr = try_acc!(
            acc,
            self.multival_expr().ok_or_else(|| SyntaxError::new(
                "Must have at least one expression".to_string(),
                self.range()
            ))
        );
    }
}

// fn validate_assign_stmt(assign_stmt: ast::AssignStmt, acc: &mut Vec<SyntaxError>)

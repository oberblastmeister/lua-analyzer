use crate::ast::{self, support, AstNode};

impl ast::Stmt {
    pub fn is_block_like(&self) -> bool {
        matches!(
            self,
            ast::Stmt::IfStmt(_)
                | ast::Stmt::ForStmt(_)
                | ast::Stmt::WhileStmt(_)
                | ast::Stmt::FunctionDefStmt(_)
                | ast::Stmt::RepeatUntilStmt(_)
        )
    }
}

impl ast::AssignStmt {
    pub fn lhs(&self) -> Option<ast::MultivalExpr> {
        self.sub_multival_exprs().0
    }

    pub fn rhs(&self) -> Option<ast::MultivalExpr> {
        self.sub_multival_exprs().1
    }

    pub fn sub_multival_exprs(&self) -> (Option<ast::MultivalExpr>, Option<ast::MultivalExpr>) {
        let mut children = support::children(self.syntax());
        let first = children.next();
        let second = children.next();
        (first, second)
    }
}

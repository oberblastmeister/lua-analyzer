use crate::{
    ast::{self, support, AstNode, AstToken},
    SyntaxToken, T,
};
use parser::LuaOp;

impl ast::InfixExpr {
    pub fn op_details(&self) -> Option<(SyntaxToken, LuaOp)> {
        self.syntax()
            .children_with_tokens()
            .filter_map(|it| it.into_token())
            .find_map(|c| {
                let kind = c.kind();
                if kind == T![not] {
                    None
                } else {
                    <Option<LuaOp>>::from(kind).map(|op| (c, op))
                }
            })
    }

    pub fn op_kind(&self) -> Option<LuaOp> {
        self.op_details().map(|t| t.1)
    }

    pub fn op_token(&self) -> Option<SyntaxToken> {
        self.op_details().map(|t| t.0)
    }

    pub fn lhs(&self) -> Option<ast::Expr> {
        self.sub_exprs().0
    }

    pub fn rhs(&self) -> Option<ast::Expr> {
        self.sub_exprs().1
    }

    pub fn sub_exprs(&self) -> (Option<ast::Expr>, Option<ast::Expr>) {
        let mut children = support::children(self.syntax());
        let first = children.next();
        let second = children.next();
        (first, second)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum LiteralKind {
    Str(ast::Str),
    Number(ast::Number),
    Bool(bool),
    Nil,
    Vararg,
}

impl ast::Literal {
    pub fn token(&self) -> SyntaxToken {
        self.syntax()
            .children_with_tokens()
            .find(|e| !e.kind().is_trivia())
            .and_then(|e| e.into_token())
            .unwrap()
    }

    pub fn kind(&self) -> LiteralKind {
        let token = self.token();

        if let Some(t) = ast::Number::cast(token.clone()) {
            return LiteralKind::Number(t);
        }

        if let Some(t) = ast::Str::cast(token.clone()) {
            return LiteralKind::Str(t);
        }

        match token.kind() {
            T![true] => LiteralKind::Bool(true),
            T![false] => LiteralKind::Bool(false),
            T![nil] => LiteralKind::Nil,
            T![...] => LiteralKind::Vararg,
            _ => unreachable!(),
        }
    }
}

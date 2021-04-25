use crate::{ast, SyntaxNode, TokenText};

use super::AstNode;

impl ast::Name {
    pub fn text(&self) -> TokenText {
        text_of_first_token(self.syntax())
    }
}

impl ast::NameRef {
    pub fn text(&self) -> TokenText {
        text_of_first_token(self.syntax())
    }
}

fn text_of_first_token(node: &SyntaxNode) -> TokenText {
    let first_token =
        node.green().children().next().and_then(|it| it.into_token()).unwrap().to_owned();

    TokenText(first_token)
}

use text_edit::Indel;

use crate::{
    syntax_node::NodeOrToken, GreenNode, SyntaxElement, SyntaxError, SyntaxNode, TextRange, N, T,
};

pub(crate) fn incremental_reparse(
    node: &SyntaxNode,
    edit: &Indel,
    errors: Vec<SyntaxError>,
) -> Option<(GreenNode, Vec<SyntaxError>, TextRange)> {
    todo!()
}

// fn repase_token(
//     root: &SyntaxNode,
//     edit: &Indel,
// ) -> Option<(GreenNode, Vec<SyntaxError>, TextRange)> {
//     let prev_token = root.covering_element(edit.delete).as_token()?.clone();
//     let prev_token_kind = prev_token.kind();
//     match prev_token_kind {
//         T![whitespace] | T![comment] | T![ident] | T![str] => {
//             if prev_token_kind == T![whitespace] || prev_token_kind == T![comment] {
//                 // removing a new line may extends previous token
//                 let deleted_range = edit.delete - prev_token.text_range().start();
//                 if prev_token.text()[deleted_range].contains('\n') {
//                     return None;
//                 }
//             }
//         }
//         _ => todo!(),
//     }
// }

// fn get_text_after_edit(element: SyntaxElement, edit: &Indel) -> String {
//     let edit = Indel::replace(edit.delete - element.text_range().start(), edit.insert.clone());

//     let mut text = match element {
//         NodeOrToken::Token(token) => token.text().to_string(),
//         NodeOrToken::Node(node) => node.text().to_string(),
//     };
//     edit.apply(&mut text);
//     text
// }

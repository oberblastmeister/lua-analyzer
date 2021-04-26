use text_edit::Indel;

use crate::{GreenNode, SyntaxError, SyntaxNode, TextRange};

pub(crate) fn incremental_reparse(
    node: &SyntaxNode,
    edit: &Indel,
    errors: Vec<SyntaxError>,
) -> Option<(GreenNode, Vec<SyntaxError>, TextRange)> {
    todo!()
}

fn repase_token(
    root: &SyntaxNode,
    edit: &Indel,
) -> Option<(GreenNode, Vec<SyntaxError>, TextRange)> {
    let prev_token = root.covering_element(edit.delete).as_token()?.clone();
    let prev_token_kind = prev_token.kind();
    todo!()
    // match prev_token_kind {

    // }
}

use anyhow::Result;
use lsp_types::{CompletionItem, CompletionItemKind, CompletionResponse, Diagnostic};
use vfs::FileId;

use crate::global_state::GlobalStateSnapshot;

pub(crate) fn handle_completion(
    snap: GlobalStateSnapshot,
    params: lsp_types::CompletionParams,
) -> Result<Option<lsp_types::CompletionResponse>> {
    let items = vec![
        completion_item("first".to_string()),
        completion_item("second".to_string()),
        completion_item("third".to_string()),
        completion_item("hello world!".to_string()),
    ];

    let response = CompletionResponse::Array(items);
    Ok(Some(response))
}

fn completion_item(s: String) -> CompletionItem {
    CompletionItem {
        label: s,
        kind: Some(CompletionItemKind::Function),
        ..Default::default()
    }
}

pub(crate) fn publish_diagnostics(snap: GlobalStateSnapshot, file_id: FileId) -> Result<Vec<Diagnostic>> {
    todo!()
}

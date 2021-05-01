use anyhow::Result;
use lsp_types::{CompletionItem, CompletionItemKind, CompletionResponse, Diagnostic};
use vfs::FileId;

use crate::{global_state::GlobalStateSnapshot, to_proto};

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
    CompletionItem { label: s, kind: Some(CompletionItemKind::Function), ..Default::default() }
}

pub(crate) fn publish_diagnostics(
    snap: &GlobalStateSnapshot,
    file_id: FileId,
) -> Result<Vec<Diagnostic>> {
    let line_index = snap.file_line_index(file_id)?;

    let diagnostics: Vec<Diagnostic> = snap
        .analysis
        .diagnostics(file_id)?
        .into_iter()
        .map(|d| Diagnostic {
            range: to_proto::range(&line_index, d.range),
            severity: Some(to_proto::diagnostic_severity(d.severity)),
            code: None,
            code_description: None,
            source: Some("lua-analyzer".to_string()),
            message: d.message,
            related_information: None,
            tags: None,
            data: None,
        })
        .collect();
    Ok(diagnostics)
}

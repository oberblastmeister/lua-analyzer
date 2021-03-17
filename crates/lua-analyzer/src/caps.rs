use lsp_types::{
    ServerCapabilities, TextDocumentSyncCapability, TextDocumentSyncKind, TextDocumentSyncOptions,
};

pub fn server_capabilities() -> ServerCapabilities {
    ServerCapabilities {
        text_document_sync: Some(TextDocumentSyncCapability::Options(
            TextDocumentSyncOptions {
                open_close: Some(true),
                change: Some(TextDocumentSyncKind::Incremental),
                ..Default::default()
            },
        )),
        ..Default::default()
    }
}

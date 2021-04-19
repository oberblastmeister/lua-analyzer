use std::mem;

use ide::FileId;
use rustc_hash::{FxHashMap, FxHashSet};

#[derive(Debug, Default, Clone)]
pub(crate) struct DiagnosticCollection {
    // FIXME: should be FxHashMap<FileId, Vec<ra_id::Diagnostic>>
    pub(crate) native: FxHashMap<FileId, Vec<lsp_types::Diagnostic>>,
    // FIXME: should be Vec<flycheck::Diagnostic>
    pub(crate) check: FxHashMap<FileId, Vec<lsp_types::Diagnostic>>,
    changes: FxHashSet<FileId>,
}

impl DiagnosticCollection {
    pub(crate) fn set_native_diagnostics(
        &mut self,
        file_id: FileId,
        diagnostics: Vec<lsp_types::Diagnostic>,
    ) {
        if let Some(existing_diagnostics) = self.native.get(&file_id) {
            if existing_diagnostics.len() == diagnostics.len()
                && diagnostics
                    .iter()
                    .zip(existing_diagnostics)
                    .all(|(new, existing)| are_diagnostics_equal(new, existing))
            {
                return;
            }
        }

        self.native.insert(file_id, diagnostics);
        self.changes.insert(file_id);
    }

    pub(crate) fn take_changes(&mut self) -> Option<FxHashSet<FileId>> {
        if self.changes.is_empty() {
            return None;
        }
        Some(mem::take(&mut self.changes))
    }

    pub(crate) fn diagnostics_for(
        &self,
        file_id: FileId,
    ) -> impl Iterator<Item = &lsp_types::Diagnostic> {
        let native = self.native.get(&file_id).into_iter().flatten();
        let check = self.check.get(&file_id).into_iter().flatten();
        native.chain(check)
    }
}

fn are_diagnostics_equal(left: &lsp_types::Diagnostic, right: &lsp_types::Diagnostic) -> bool {
    left.source == right.source
        && left.severity == right.severity
        && left.range == right.range
        && left.message == right.message
}

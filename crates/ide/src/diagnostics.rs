use hir::Semantics;
use ide_db::{base_db::SourceDatabase, RootDatabase};
use syntax::TextRange;

use crate::FileId;

#[derive(Debug)]
pub struct Diagnostic {
    // pub name: Option<String>,
    pub message: String,
    pub range: TextRange,
    pub severity: Severity,
}

impl Diagnostic {
    fn error(range: TextRange, message: String) -> Self {
        Self {
            message,
            range,
            severity: Severity::Error,
        }
    }

    fn hint(range: TextRange, message: String) -> Self {
        Self {
            message,
            range,
            severity: Severity::WeakWarning,
        }
    }
}

pub(crate) fn diagnostics(db: &RootDatabase, file_id: FileId) -> Vec<Diagnostic> {
    let sema = Semantics::new(db);
    let parse = db.parse(file_id);
    let mut res = Vec::new();
    // [#34344] Only take first 128 errors to prevent slowing down editor/ide, the number 128 is chosen arbitrarily.
    res.extend(
        parse
            .errors()
            .iter()
            .take(128)
            .map(|err| Diagnostic::error(err.range(), format!("Syntax Error: {}", err))),
    );
    res
}

#[derive(Debug, Copy, Clone)]
pub enum Severity {
    Error,
    WeakWarning,
}

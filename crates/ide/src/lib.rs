mod diagnostics;

use std::sync::Arc;

pub use ide_db::{
    base_db::{Change, FileId},
    line_index::{LineCol, LineColUtf16, LineIndex},
    LineIndexDatabase,
};
pub use diagnostics::Severity;

use ide_db::{
    base_db::{salsa, Canceled, CheckCanceled},
    RootDatabase,
};
use salsa::{ParallelDatabase, Snapshot};

use diagnostics::Diagnostic;

pub type Cancelable<T> = Result<T, Canceled>;

pub struct AnalysisHost {
    db: RootDatabase,
}

impl AnalysisHost {
    pub fn new() -> AnalysisHost {
        AnalysisHost {
            db: RootDatabase::new(),
        }
    }

    /// Returns a snapshot of the current state, which you can query for
    /// semantic information.
    pub fn analysis(&self) -> Analysis {
        Analysis {
            db: self.db.snapshot(),
        }
    }

    /// Applies changes to the current state of the world. If there are
    /// outstanding snapshots, they will be canceled.
    pub fn apply_change(&mut self, change: Change) {
        self.db.apply_change(change)
    }
}

pub struct Analysis {
    db: Snapshot<RootDatabase>,
}

impl Analysis {
    /// Performs an operation on that may be Canceled.
    fn with_db<F, T>(&self, f: F) -> Cancelable<T>
    where
        F: FnOnce(&RootDatabase) -> T + std::panic::UnwindSafe,
    {
        self.db.catch_canceled(f)
    }

    pub fn file_line_index(&self, file_id: FileId) -> Cancelable<Arc<LineIndex>> {
        self.with_db(|db| db.line_index(file_id))
    }

    pub fn diagnostics(&self, file_id: FileId) -> Cancelable<Vec<Diagnostic>> {
        self.with_db(|db| diagnostics::diagnostics(db, file_id))
    }
}

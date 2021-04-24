pub mod line_index;

use std::{fmt, sync::Arc};

pub use base_db;

use base_db::{
    salsa::{self, Database},
    Canceled, Change, CheckCanceled, FileId, SourceDatabase, Upcast,
};
use hir::{AstDatabase, DefDatabase, HirDatabase};
use line_index::LineIndex;

#[salsa::database(
    LineIndexDatabaseStorage,
    base_db::SourceDatabaseStorage,
    hir::InternDatabaseStorage,
    hir::AstDatabaseStorage,
    hir::HirDatabaseStorage,
    hir::DefDatabaseStorage
)]
pub struct RootDatabase {
    storage: salsa::Storage<RootDatabase>,
}

impl Upcast<dyn SourceDatabase> for RootDatabase {
    fn upcast(&self) -> &(dyn SourceDatabase + 'static) {
        &*self
    }
}

impl Upcast<dyn DefDatabase> for RootDatabase {
    fn upcast(&self) -> &(dyn DefDatabase + 'static) {
        &*self
    }
}

impl Upcast<dyn HirDatabase> for RootDatabase {
    fn upcast(&self) -> &(dyn HirDatabase + 'static) {
        &*self
    }
}

impl Upcast<dyn AstDatabase> for RootDatabase {
    fn upcast(&self) -> &(dyn AstDatabase + 'static) {
        &*self
    }
}

impl RootDatabase {
    pub fn request_cancellation(&mut self) {
        self.salsa_runtime_mut().synthetic_write(salsa::Durability::LOW);
    }

    pub fn apply_change(&mut self, change: Change) {
        self.request_cancellation();
        change.apply(self);
    }
}

impl fmt::Debug for RootDatabase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RootDatabase").finish()
    }
}

impl salsa::Database for RootDatabase {
    fn on_propagated_panic(&self) -> ! {
        Canceled::throw()
    }

    fn salsa_event(&self, event: salsa::Event) {
        match event.kind {
            salsa::EventKind::DidValidateMemoizedValue { .. }
            | salsa::EventKind::WillExecute { .. } => {
                self.check_canceled();
            }
            _ => (),
        }
    }
}

impl salsa::ParallelDatabase for RootDatabase {
    fn snapshot(&self) -> salsa::Snapshot<RootDatabase> {
        salsa::Snapshot::new(RootDatabase { storage: self.storage.snapshot() })
    }
}

impl RootDatabase {
    pub fn new() -> RootDatabase {
        RootDatabase { storage: salsa::Storage::default() }
    }
}

#[salsa::query_group(LineIndexDatabaseStorage)]
pub trait LineIndexDatabase: base_db::SourceDatabase + CheckCanceled {
    fn line_index(&self, file_id: FileId) -> Arc<LineIndex>;
}

fn line_index(db: &dyn LineIndexDatabase, file_id: FileId) -> Arc<LineIndex> {
    let text = db.file_text(file_id);
    Arc::new(LineIndex::new(&*text))
}

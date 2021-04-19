pub mod line_index;

use std::fmt;

pub use base_db;

use base_db::{
    salsa::{self, Database},
    Canceled, Change, CheckCanceled,
};

#[salsa::database(base_db::SourceDatabaseStorage)]
pub struct RootDatabase {
    storage: salsa::Storage<RootDatabase>,
}

impl RootDatabase {
    pub fn request_cancellation(&mut self) {
        self.salsa_runtime_mut()
            .synthetic_write(salsa::Durability::LOW);
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
        salsa::Snapshot::new(RootDatabase {
            storage: self.storage.snapshot(),
        })
    }
}

impl RootDatabase {
    pub fn new() -> RootDatabase {
        RootDatabase {
            storage: salsa::Storage::default(),
        }
    }
}

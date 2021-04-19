pub mod line_index;

pub use base_db;

use base_db::{salsa, Canceled, CheckCanceled};

#[salsa::database]
pub struct RootDatabase {
    storage: salsa::Storage<RootDatabase>,
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

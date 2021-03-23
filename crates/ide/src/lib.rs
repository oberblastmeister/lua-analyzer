use ide_db::{
    base_db::{Canceled, CheckCanceled},
    RootDatabase,
};
use salsa::{ParallelDatabase, Snapshot};

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
}

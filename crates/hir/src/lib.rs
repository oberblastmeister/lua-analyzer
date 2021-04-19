mod semantics;

pub use semantics::Semantics;

use base_db::{salsa, SourceDatabase, Upcast};

#[salsa::query_group(HirDatabaseStorage)]
pub trait HirDatabase: DefDatabase + Upcast<dyn DefDatabase> {
    fn place(&self, n: u32) -> u32;
}

fn place(db: &dyn HirDatabase, n: u32) -> u32 {
    0
}

#[salsa::query_group(DefDatabaseStorage)]
pub trait DefDatabase: SourceDatabase + Upcast<dyn SourceDatabase> {
    fn another(&self, n: u32) -> u32;
}

fn another(db: &dyn DefDatabase, n: u32) -> u32 {
    0
}

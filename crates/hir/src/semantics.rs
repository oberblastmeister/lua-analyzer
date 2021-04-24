use crate::HirDatabase;

use base_db::{FileId, SourceDatabase};
use syntax::{ast, SourceFile};

pub struct Semantics<'db> {
    pub db: &'db dyn HirDatabase,
}

impl<'db> Semantics<'db> {
    pub fn new(db: &'db dyn HirDatabase) -> Semantics<'_> {
        Semantics { db }
    }

    pub fn parse(&self, file_id: FileId) -> ast::SourceFile {
        self.db.parse(file_id).tree()
    }
}

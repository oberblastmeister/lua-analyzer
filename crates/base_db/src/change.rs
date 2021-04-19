use std::sync::Arc;

use vfs::FileId;

use crate::SourceDatabase;

/// Encapsulate a bunch of raw `.set` calls on the database.
#[derive(Debug, Default)]
pub struct Change {
    pub files_changed: Vec<(FileId, Option<Arc<String>>)>,
}

impl Change {
    pub fn new() -> Change {
        Change::default()
    }

    pub fn apply(self, db: &mut dyn SourceDatabase) {
        for (file_id, text) in self.files_changed {
            // XXX: can't actually remove the file, just reset the text
            let text = text.unwrap_or_default();
            db.set_file_text(file_id, text)
        }
    }

    pub fn change_file(&mut self, file_id: FileId, new_text: Option<Arc<String>>) {
        self.files_changed.push((file_id, new_text))
    }
}

use std::path::{Path, PathBuf};

use rustc_hash::FxHashMap;

use crate::FileId;

#[derive(Default)]
pub(crate) struct PathInterner {
    /// path to id
    map: FxHashMap<PathBuf, FileId>,
    /// id to path
    vec: Vec<PathBuf>,
}

impl PathInterner {
    pub(crate) fn get(&self, path: &Path) -> Option<FileId> {
        self.map.get(path).copied()
    }

    pub(crate) fn intern(&mut self, path: PathBuf) -> FileId {
        if let Some(id) = self.get(&path) {
            return id;
        }

        let id = FileId(self.vec.len() as u32);
        self.map.insert(path.clone(), id);
        self.vec.push(path);
        id
    }

    pub(crate) fn lookup(&self, id: FileId) -> &Path {
        &self.vec[id.0 as usize]
    }
}

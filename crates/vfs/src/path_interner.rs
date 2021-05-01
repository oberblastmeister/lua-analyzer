use std::path::{Path, PathBuf};

use crate::FileId;

use rustc_hash::FxHashMap;
use stdx::paths::{AbsPath, AbsPathBuf};

#[derive(Default)]
pub(crate) struct PathInterner {
    /// path to id
    map: FxHashMap<AbsPathBuf, FileId>,
    /// id to path
    vec: Vec<AbsPathBuf>,
}

impl PathInterner {
    pub(crate) fn get(&self, path: &AbsPath) -> Option<FileId> {
        self.map.get(path).copied()
    }

    pub(crate) fn intern(&mut self, path: AbsPathBuf) -> FileId {
        if let Some(id) = self.get(&path) {
            return id;
        }

        let id = FileId(self.vec.len() as u32);
        self.map.insert(path.clone(), id);
        self.vec.push(path);
        id
    }

    pub(crate) fn lookup(&self, id: FileId) -> &AbsPath {
        &self.vec[id.0 as usize]
    }
}

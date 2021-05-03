use std::hash::BuildHasherDefault;

use indexmap::IndexSet;
use rustc_hash::FxHasher;
use stdx::paths::{AbsPath, AbsPathBuf};

use crate::FileId;

type FxIndexSet<T> = IndexSet<T, BuildHasherDefault<FxHasher>>;

#[derive(Default)]
pub(crate) struct PathInterner {
    map: FxIndexSet<AbsPathBuf>,
}

impl PathInterner {
    pub(crate) fn get(&self, path: &AbsPath) -> Option<FileId> {
        self.map.get_index_of(path).map(|i| FileId(i as u32))
    }

    pub(crate) fn intern(&mut self, path: AbsPathBuf) -> FileId {
        let (id, _added) = self.map.insert_full(path);
        assert!(id < u32::MAX as usize);
        FileId(id as u32)
    }

    pub(crate) fn lookup(&self, id: FileId) -> &AbsPath {
        self.map.get_index(id.0 as usize).unwrap()
    }
}

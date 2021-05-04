use std::{mem, path::Path, sync::Arc};

use stdx::paths::AbsPathBuf;

use crate::{config::Config, global_state::GlobalState};

impl GlobalState {
    pub(crate) fn update_configuration(&mut self, config: Config) -> bool {
        let old_config = mem::replace(&mut self.config, Arc::new(config));
        // TODO: reload workspace here
        true
    }

    pub(crate) fn load_workspace(&mut self) {
        let library: Vec<_> = self
            .config
            .library()
            .into_iter()
            .map(|s| vfs::handle::Entry::new(AbsPathBuf::assert(&**s)))
            .collect();
        // TODO: register dynamic cap

        self.vfs_config_version += 1;
        self.loader
            .handle
            .set_config(vfs::handle::Config { load: library, version: self.vfs_config_version });
    }
}

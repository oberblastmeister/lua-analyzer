use std::{mem, sync::Arc};

use crate::{config::Config, global_state::GlobalState};

impl GlobalState {
    pub(crate) fn update_configuration(&mut self, config: Config) -> bool {
        let old_config = mem::replace(&mut self.config, Arc::new(config));
        // TODO: reload workspace here
        true
    }
}

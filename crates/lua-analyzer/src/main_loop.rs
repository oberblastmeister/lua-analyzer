use anyhow::Result;

use crossbeam_channel::Receiver;
use lsp_server::{Connection, Notification};

use crate::global_state::GlobalState;

pub fn main_loop(connection: Connection) -> Result<()> {
    Ok(())
}

impl GlobalState {
    fn run(mut self, inbox: Receiver<lsp_server::Message>) -> Result<()> {
        Ok(())
    }

    fn on_notification(&mut self, not: Notification) -> Result<()> {
        Ok(())
    }
}

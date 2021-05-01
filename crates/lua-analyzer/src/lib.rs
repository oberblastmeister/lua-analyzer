mod caps;
pub mod config;
mod diagnostics;
mod dispatch;
mod document;
mod from_proto;
mod global_state;
mod handlers;
mod lsp_utils;
mod main_loop;
mod reload;
mod thread_pool;
mod to_proto;

use anyhow::{anyhow, Result};
use serde::de::DeserializeOwned;

pub use caps::server_capabilities;
pub use main_loop::main_loop;

pub fn from_json<T: DeserializeOwned>(what: &'static str, json: serde_json::Value) -> Result<T> {
    let res = serde_path_to_error::deserialize(&json)
        .map_err(|e| anyhow!("Failed to deserialize {}: {}; {}", what, e, json))?;
    Ok(res)
}

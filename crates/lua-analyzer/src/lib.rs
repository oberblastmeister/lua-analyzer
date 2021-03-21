mod caps;
mod dispatch;
mod from_proto;
mod global_state;
mod main_loop;
mod thread_pool;

use anyhow::{anyhow, Result};
use serde::de::DeserializeOwned;

pub use main_loop::main_loop;

pub fn from_json<T: DeserializeOwned>(what: &'static str, json: serde_json::Value) -> Result<T> {
    let res = serde_path_to_error::deserialize(&json)
        .map_err(|e| anyhow!("Failed to deserialize {}: {}; {}", what, e, json))?;
    Ok(res)
}

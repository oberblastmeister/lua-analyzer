use std::path::Path;
use std::{env, path::PathBuf};

pub fn xtask_root() -> PathBuf {
    Path::new(
        &env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| env!("CARGO_MANIFEST_DIR").to_owned()),
    )
    .to_path_buf()
}

pub fn project_root() -> PathBuf {
    xtask_root()
    .ancestors()
    .nth(1)
    .unwrap()
    .to_path_buf()
}

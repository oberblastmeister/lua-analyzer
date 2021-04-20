use anyhow::Result;
use std::path::Path;
use std::{env, path::PathBuf};
use xshell::cmd;
use xshell::{read_file, write_file};

pub fn xtask_root() -> PathBuf {
    Path::new(
        &env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| env!("CARGO_MANIFEST_DIR").to_owned()),
    )
    .to_path_buf()
}

pub fn project_root() -> PathBuf {
    xtask_root().ancestors().nth(1).unwrap().to_path_buf()
}

pub const PREAMBLE: &str = "Generated file, do not edit by hand, see `xtask/src/codegen`";

pub fn reformat(text: &str) -> Result<String> {
    let stdout = cmd!("rustfmt").stdin(text).read()?;
    Ok(format!("//! {}\n\n{}\n", PREAMBLE, stdout))
}

/// A helper to update file on disk if it has changed.
/// With verify = false,
pub fn update(path: &Path, contents: &str) -> Result<()> {
    fn normalize(s: &str) -> String {
        s.replace("\r\n", "\n")
    }

    match read_file(path) {
        Ok(old_contents) if normalize(&old_contents) == normalize(contents) => {
            return Ok(());
        }
        _ => (),
    }

    eprintln!("updating {}", path.display());
    write_file(path, contents)?;
    Ok(())
}

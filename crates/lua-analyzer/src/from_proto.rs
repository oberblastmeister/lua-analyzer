use std::path::PathBuf;

use anyhow::{anyhow, Context, Result};

pub(crate) fn path(url: &lsp_types::Url) -> Result<PathBuf> {
    let path = url
        .to_file_path()
        .map_err(|()| anyhow!("url is not a file"))?;

    if !path.is_absolute() {
        panic!("The path {} was not absolute", path.display());
    }

    Ok(path)
}

use std::convert::TryFrom;
use std::path::PathBuf;

use anyhow::{anyhow, Context, Result};

use ide::{LineColUtf16, LineIndex};
use stdx::paths::AbsPathBuf;
use syntax::{TextRange, TextSize};

pub(crate) fn abs_path(url: &lsp_types::Url) -> Result<AbsPathBuf> {
    let path = url.to_file_path().map_err(|()| anyhow!("url is not a file"))?;
    Ok(AbsPathBuf::try_from(path).unwrap())
}

pub(crate) fn offset(line_index: &LineIndex, position: lsp_types::Position) -> TextSize {
    let line_col = {
        let line_col = LineColUtf16 { line: position.line as u32, col: position.character as u32 };
        line_index.to_utf8(line_col)
    };
    line_index.offset(line_col)
}

pub(crate) fn text_range(line_index: &LineIndex, range: lsp_types::Range) -> TextRange {
    let start = offset(line_index, range.start);
    let end = offset(line_index, range.end);
    TextRange::new(start, end)
}

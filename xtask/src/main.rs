mod codegen;
mod utils;

use anyhow::Result;

fn main() -> Result<()> {
    codegen::run()?;

    Ok(())
}

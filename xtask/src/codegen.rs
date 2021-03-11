mod gen_syntax;

use anyhow::Result;

pub fn run() -> Result<()> {
    gen_syntax::run()?;

    Ok(())
}

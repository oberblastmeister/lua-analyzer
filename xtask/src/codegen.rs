mod gen_syntax;

use anyhow::Result;

use crate::flags;

impl flags::Codegen {
    pub fn run(self) -> Result<()> {
        gen_syntax::run()?;

        Ok(())
    }
}

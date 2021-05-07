use anyhow::{bail, Result};
use xshell::{cmd, pushd, pushenv, rm_rf};

use crate::flags;

impl flags::FuzzTests {
    pub fn run(self) -> Result<()> {
        let _d = pushd("./crates/syntax");

        if self.reset {
            let _d = pushd("./fuzz");
            rm_rf("artifacts")?;
            rm_rf("corpus")?;
            return Ok(());
        }

        let _e = pushenv("RUSTUP_TOOLCHAIN", "nightly");
        if cmd!("cargo fuzz --help").read().is_err() {
            cmd!("cargo install cargo-fuzz").run()?;
        };

        let out = cmd!("rustc --version").read()?;
        if !out.contains("nightly") {
            bail!("fuzz tests require nightly rustc")
        }

        let target = if self.lexer { "lexer" } else { "parser" };

        cmd!("cargo fuzz run {target}").run()?;

        Ok(())
    }
}

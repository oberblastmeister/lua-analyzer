use anyhow::{bail, Result};
use xshell::{cmd, pushd, pushenv};

use crate::flags;

impl flags::FuzzTests {
    pub fn run(self) -> Result<()> {
        let _d = pushd("./crates/syntax");
        let _e = pushenv("RUSTUP_TOOLCHAIN", "nightly");
        if cmd!("cargo fuzz --help").read().is_err() {
            cmd!("cargo install cargo-fuzz").run()?;
        };

        let out = cmd!("rustc --version").read()?;
        if !out.contains("nightly") {
            bail!("fuzz tests require nightly rustc")
        }

        cmd!("cargo fuzz run parser").run()?;
        Ok(())
    }
}

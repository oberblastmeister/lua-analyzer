mod codegen;
mod dist;
mod flags;
mod fuzz_tests;
mod utils;

use anyhow::Result;
use xshell::pushd;

fn main() -> Result<()> {
    use flags::{Xtask, XtaskCmd::*};

    let _d = pushd(utils::project_root())?;

    let flags = Xtask::from_env()?;

    match flags.subcommand {
        Help(_) => println!("{}", Xtask::HELP),
        Codegen(cmd) => cmd.run()?,
        FuzzTests(cmd) => cmd.run()?,
    };

    Ok(())
}

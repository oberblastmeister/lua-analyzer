use std::{
    fs::{self, File},
    process,
};

use anyhow::{Context, Result};
use directories_next::ProjectDirs;
use log::info;
use lsp_server::Connection;
use simplelog::{Config, LevelFilter, WriteLogger};

fn main() {
    if let Err(err) = try_main() {
        log::error!("Unexpected error: {}", err);
        eprintln!("{}", err);
        process::exit(1);
    }
}

fn try_main() -> Result<()> {
    setup_logging()?;

    run_server()?;

    Ok(())
}

fn run_server() -> Result<()> {
    info!("Server will start");

    let (connection, io_threads) = Connection::stdio();

    lua_analyzer::main_loop(connection)?;

    io_threads.join()?;
    log::info!("server did shut down");

    Ok(())
}

fn setup_logging() -> Result<()> {
    let proj_dirs = ProjectDirs::from("rs", "Open Source", "Lua Analyzer")
        .expect("Failed to find project dirs");
    let data_dir = proj_dirs.data_dir();
    fs::create_dir_all(data_dir).context("Failed to create data dir")?;
    let file_path = proj_dirs.data_dir().join("server.log");
    let file = File::create(file_path).context("Failed to create log file")?;
    WriteLogger::init(LevelFilter::Debug, Config::default(), file)?;

    Ok(())
}

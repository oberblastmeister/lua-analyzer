use std::{
    convert::TryFrom,
    env,
    fs::{self, File},
    process,
};

use anyhow::{Context, Result};
use directories_next::ProjectDirs;
use log::info;
use lsp_server::Connection;
use lua_analyzer::{config::Config, from_json};
use simplelog::{LevelFilter, WriteLogger};
use stdx::paths::AbsPathBuf;

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

    let (initialize_id, initialize_params) = connection.initialize_start()?;
    log::info!("InitializeParams: {}", initialize_params);
    let initialize_params =
        from_json::<lsp_types::InitializeParams>("InitializeParams", initialize_params)?;

    let server_capabilities = lua_analyzer::server_capabilities();

    let initialize_result = lsp_types::InitializeResult {
        capabilities: server_capabilities,
        server_info: None,
    };
    let initialize_result = serde_json::to_value(initialize_result).unwrap();

    connection.initialize_finish(initialize_id, initialize_result)?;

    let config = {
        let root_path = match initialize_params
            .root_uri
            .and_then(|it| it.to_file_path().ok())
            .and_then(|it| AbsPathBuf::try_from(it).ok())
        {
            Some(it) => it,
            None => {
                let cwd = env::current_dir()?;
                AbsPathBuf::assert(cwd)
            }
        };

        let mut config = Config::new(root_path, initialize_params.capabilities);
        if let Some(json) = initialize_params.initialization_options {
            config.update(json);
        }

        log::info!("Got config: {:#?}", config);

        config
    };

    lua_analyzer::main_loop(config, connection)?;

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
    WriteLogger::init(LevelFilter::Debug, simplelog::Config::default(), file)?;

    Ok(())
}

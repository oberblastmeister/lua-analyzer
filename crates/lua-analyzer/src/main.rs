use std::io::Result;
use log::info;
use lsp_server::Connection;

fn main() {
    println!("Hello, world!");
}

fn try_main() -> Result<()> {
    setup_logging();

    Ok(())
}

fn run_server() -> Result<()> {
    info!("Server will start");

    let (connection, io_threads) = Connection::stdio();

    lua_analyzer::main_loop(connection);

    io_threads.join()?;
    log::info!("server did shut down");

    Ok(())
}

fn setup_logging() {
}

// main.rs

mod commands;
mod config;
mod db;
mod errors;
mod models;

use clap::Parser;
use commands::Cli;
use db::Db;
use errors::AppError;
use log::info;

fn main() {
    env_logger::init();
    info!("App started!");

    let cli = Cli::parse();

    if let Err(e) = run(cli) {
        eprintln!("Error : {e}");
        std::process::exit(1);
    }
}

fn run(cli: Cli) -> Result<(), AppError> {
    let db = Db::new()?;
    cli.command.execute(&db)
}

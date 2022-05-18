use std::process;

use clap::Parser;
use cli::{Cli, Commands};

mod cli;
mod error;
mod service;
mod transaction;

fn main() {
    let cli = Cli::parse();

    let ledger_file_path = match cli.global_args().ledger_file() {
        Ok(path) => path,
        Err(err) => {
            eprintln!("{}", err);
            process::exit(err.into())
        }
    };

    let result = match cli.commands() {
		Commands::New => service::ledger::create_new_ledger(ledger_file_path.as_path()),
		Commands::Accounts => service::ledger::read_ledger(ledger_file_path.as_path())
    };

    if let Err(err) = result {
        eprintln!("{}", err);
        process::exit(err.into())
    }
}

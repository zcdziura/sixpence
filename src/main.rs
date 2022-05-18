use std::process;

use clap::Parser;
use cli::{Cli, Commands};

mod cli;
mod error;
mod ledger;
mod service;

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
		Commands::Accounts => Ok(())
    };

    if let Err(err) = result {
        eprintln!("{}", err);
        process::exit(err.into())
    }
}

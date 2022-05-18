use std::process;

use clap::Parser;
use cli::Cli;

use crate::cli::{process_ledger_commands, Commands};

mod cli;
mod error;
mod ledger;

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
        Commands::Ledger(ledger_commands) => {
            process_ledger_commands(ledger_commands, ledger_file_path.as_path())
        }
    };

    if let Err(err) = result {
        eprintln!("{}", err);
        process::exit(err.into())
    }
}

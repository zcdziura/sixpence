use std::process;

use accounting::Accounting;
use clap::Parser;
use cli::{Cli, Commands};
use lazy_static::lazy_static;
use regex::Regex;

mod account;
mod cli;
mod commands;
mod error;
mod transaction;

lazy_static! {
    static ref TIMESTAMP_CHECK_RE: Regex =
        Regex::new(r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}-\d{2}:\d{2}").unwrap();
    static ref ACCOUNTING: Accounting = Accounting::new_from("$", 2);
}

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
        Commands::New => commands::new_ledger(ledger_file_path.as_path()),
        Commands::Accounts => commands::display_accounts(ledger_file_path.as_path()),
    };

    if let Err(err) = result {
        eprintln!("{}", err);
        process::exit(err.into())
    }
}

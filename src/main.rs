use std::process;

use accounting::Accounting;
use clap::Parser;
use cli::{Cli, Commands};
use lazy_static::lazy_static;

mod account;
mod cli;
mod commands;
mod error;
mod services;
mod transaction;

lazy_static! {
    static ref ACCOUNTING: Accounting = Accounting::new("$", 2, ",", ".", "{v}", "({v})", "—");
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
        Commands::Transaction(opts) => commands::new_transaction(ledger_file_path.as_path(), opts),
    };

    if let Err(err) = result {
        eprintln!("{}", err);
        process::exit(err.into())
    }
}

mod account;
mod error;
mod opts;
mod transaction;

use std::process;

use account::create_new_account;
use opts::{transaction::TransactionOpts, AccountOpts, Commands, Opts};
use transaction::create_new_transaction;

#[paw::main]
fn main(args: Opts) {
    let accounts_file = match args.accounts_file() {
        Ok(file) => file,
        Err(error) => {
            eprintln!("{}", error);
            process::exit(error.into());
        }
    };

    let ledger_file = match args.ledger_file() {
        Ok(file) => file,
        Err(error) => {
            eprintln!("{}", error);
            process::exit(error.into());
        }
    };

    match args.commands() {
        Commands::Account(opts) => match opts {
            AccountOpts::NewAccount(new_account_opts) => {
                let result = create_new_account(
                    accounts_file.as_path(),
                    new_account_opts.name(),
                    new_account_opts.account_type(),
                );

                match result {
                    Ok(_) => {}
                    Err(error) => {
                        eprintln!("{}", error);
                        std::process::exit(error.into());
                    }
                }
            }
        },
        Commands::Transaction(opts) => match opts {
            TransactionOpts::NewTransaction(new_transaction_opts) => {
                let result = create_new_transaction(
                    accounts_file.as_path(),
                    ledger_file.as_path(),
                    new_transaction_opts,
                );

                match result {
                    Ok(_) => {}
                    Err(error) => {
                        eprintln!("{}", error);
                        std::process::exit(error.into());
                    }
                }
            }
        },
    }
}

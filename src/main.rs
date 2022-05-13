use std::process;

use clap::Parser;
use cli::Cli;

// mod account;
mod cli;
mod error;
// mod transaction;

fn main() {
    let cli = Cli::parse();

    let ledger_file = match cli.global_args().ledger_file() {
        Ok(ledger_file) => ledger_file,
        Err(err) => {
            eprintln!("{}", err);
            process::exit(err.into())
        }
    };

    // let accounts_file = match args_old.accounts_file() {
    //     Ok(file) => file,
    //     Err(error) => {
    //         eprintln!("{}", error);
    //         process::exit(error.into());
    //     }
    // };

    // let ledger_file = match args_old.ledger_file() {
    //     Ok(file) => file,
    //     Err(error) => {
    //         eprintln!("{}", error);
    //         process::exit(error.into());
    //     }
    // };

    // match args_old.commands() {
    //     Commands::Account(opts) => match opts {
    //         AccountOpts::NewAccount(new_account_opts) => {
    //             let result = create_new_account(
    //                 accounts_file.as_path(),
    //                 new_account_opts.name(),
    //                 new_account_opts.account_type(),
    //             );

    //             match result {
    //                 Ok(_) => {}
    //                 Err(error) => {
    //                     eprintln!("{}", error);
    //                     process::exit(error.into());
    //                 }
    //             }
    //         }
    //         AccountOpts::ListAccounts(opts) => {
    //             let result = list_accounts(accounts_file.as_path(), opts.account_type());

    //             match result {
    //                 Ok(_) => {}
    //                 Err(error) => {
    //                     eprintln!("{}", error);
    //                     process::exit(error.into());
    //                 }
    //             }
    //         }
    //         AccountOpts::EditAccount(opts) => {
    //             let result = edit_account(opts.name(), opts.new_name(), opts.new_account_type());

    //             match result {
    //                 Ok(_) => {}
    //                 Err(error) => {
    //                     eprintln!("{}", error);
    //                     process::exit(error.into());
    //                 }
    //             }
    //         }
    //     },
    //     Commands::Transaction(opts) => match opts {
    //         TransactionOpts::NewTransaction(new_transaction_opts) => {
    //             let result = create_new_transaction(
    //                 accounts_file.as_path(),
    //                 ledger_file.as_path(),
    //                 new_transaction_opts,
    //             );

    //             match result {
    //                 Ok(_) => {}
    //                 Err(error) => {
    //                     eprintln!("{}", error);
    //                     process::exit(error.into());
    //                 }
    //             }
    //         }
    //     },
    // }
}

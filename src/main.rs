mod account;
mod error;
mod opts;
mod transaction;

use account::create_new_account;
use opts::{transaction::TransactionOpts, AccountOpts, Commands, Opts};
use transaction::validate_new_transaction_opts;

#[paw::main]
fn main(args: Opts) {
    let accounts_file = match args.accounts_file() {
        Ok(file) => file,
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };

    match args.commands() {
        Commands::Account(opts) => match opts {
            AccountOpts::NewAccount(new_account_opts) => {
                let result = create_new_account(
                    accounts_file,
                    new_account_opts.name(),
                    new_account_opts.account_type(),
                );

                match result {
                    Ok(_) => {}
                    Err(error) => {
                        eprintln!("{}", error);
                        std::process::exit(2);
                    }
                }
            }
        },
        Commands::Transaction(opts) => match opts {
            TransactionOpts::NewTransaction(new_transaction_opts) => {
                validate_new_transaction_opts(accounts_file, new_transaction_opts);
            }
        },
    }
}

mod account;
mod error;
mod opts;

use account::create_new_account;
use opts::{AccountOpts, Commands, Opts};

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
    }
}

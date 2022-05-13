use structopt::StructOpt;

use std::{
    fs::{DirBuilder, File},
    io,
    path::{Path, PathBuf},
};

use crate::error::Error;
use platform_dirs::AppDirs;

use super::{account::AccountOpts, transaction::TransactionOpts};

#[derive(Debug, StructOpt)]
pub struct Opts {
    /// Specify an alternate accounts file
    #[structopt(short, long = "accounts")]
    accounts_file: Option<PathBuf>,

    /// Specify an alternate ledger file
    #[structopt(short, long = "ledger")]
    ledger_file: Option<PathBuf>,

    #[structopt(subcommand)]
    cmds: Commands,
}

impl Opts {
    pub fn accounts_file(&self) -> Result<PathBuf, Error> {
        create_data_file_if_not_exists(self.accounts_file.clone(), "accounts.dat")
    }

    pub fn ledger_file(&self) -> Result<PathBuf, Error> {
        create_data_file_if_not_exists(self.ledger_file.clone(), "ledger.dat")
    }

    pub fn commands(&self) -> &Commands {
        &self.cmds
    }
}

fn create_data_file_if_not_exists(
    path: Option<PathBuf>,
    default_file_name: &str,
) -> Result<PathBuf, Error> {
    let path = path.unwrap_or_else(|| {
        AppDirs::new(Some(env!("CARGO_PKG_NAME")), true)
            .map(|app_dirs| app_dirs.data_dir.as_path().join(default_file_name))
            .unwrap()
    });

    if !path.exists() {
        create_full_file_path(path.as_path()).map_err(|error| {
            Error::data_file(
                format!("Unable to create data file: {}.", default_file_name),
                error,
            )
        })?;
    }

    Ok(path)
}

fn create_full_file_path(path: &Path) -> io::Result<()> {
    let parent_dir = path.parent().unwrap();
    let _ = DirBuilder::new().recursive(true).create(parent_dir)?;
    let _ = File::create(path)?;

    Ok(())
}

#[derive(Debug, StructOpt)]
pub enum Commands {
    /// Create, edit, or delete accounts
    #[structopt(name = "acct")]
    Account(AccountOpts),

    /// Create, edit, or delete transactions
    #[structopt(name = "txn")]
    Transaction(TransactionOpts),
}

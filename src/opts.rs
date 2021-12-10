pub mod account;
pub mod transaction;

use std::{
    fs::{DirBuilder, File},
    io,
    path::{Path, PathBuf},
};

use platform_dirs::AppDirs;
use structopt::StructOpt;

use crate::error::{Error, ErrorKind};

pub use self::account::AccountOpts;
use self::transaction::TransactionOpts;

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
        let accounts_file_path = self.accounts_file.clone().unwrap_or_else(|| {
            AppDirs::new(Some(env!("CARGO_PKG_NAME")), true)
                .map(|app_dirs| app_dirs.data_dir.as_path().join("accounts.dat"))
                .unwrap()
        });

        if !accounts_file_path.exists() {
            create_full_file_path(accounts_file_path.as_path()).map_err(|err| {
                Box::new(ErrorKind::AccountsFile(
                    "Unable to create accounts file".into(),
                    err,
                ))
            })?;
        }

        Ok(accounts_file_path)
    }

    pub fn ledger_file(&self) -> Result<PathBuf, Error> {
        let ledger_file_path = self.ledger_file.clone().unwrap_or_else(|| {
            AppDirs::new(Some(env!("CARGO_PKG_NAME")), true)
                .map(|app_dirs| app_dirs.data_dir.as_path().join("ledger.dat"))
                .unwrap()
        });

        if !ledger_file_path.exists() {
            create_full_file_path(ledger_file_path.as_path()).map_err(|err| {
                Box::new(ErrorKind::LedgerFile(
                    "Unable to create ledger file".into(),
                    err,
                ))
            })?;
        }

        Ok(ledger_file_path)
    }

    pub fn commands(&self) -> &Commands {
        &self.cmds
    }
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

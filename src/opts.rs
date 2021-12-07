pub mod account;

use std::{
    fs::{DirBuilder, File},
    path::PathBuf,
};

use platform_dirs::AppDirs;
use structopt::StructOpt;

use crate::error::{Error, ErrorKind};

pub use self::account::AccountOpts;

#[derive(Debug, StructOpt)]
pub struct Opts {
    /// Specify an alternate accounts file
    #[structopt(short, long = "accounts")]
    accounts_file: Option<PathBuf>,

    #[structopt(subcommand)]
    cmds: Commands,
}

impl Opts {
    /// Returns the full path of the accounts file
    /// If Ok(PathBuf) is returned, then the given path is guaranteed to exist
    pub fn accounts_file(&self) -> Result<PathBuf, Error> {
        let accounts_file_path = if self.accounts_file.is_some() {
            self.accounts_file.clone().unwrap()
        } else {
            AppDirs::new(Some(env!("CARGO_PKG_NAME")), true)
                .map(|app_dirs| {
                    let mut data_dir = app_dirs.data_dir;
                    data_dir.push("accounts.dat");
                    data_dir
                })
                .unwrap()
        };

        if !accounts_file_path.exists() {
            let parent_dir = accounts_file_path.parent().unwrap();
            let _ = DirBuilder::new().recursive(true).create(parent_dir)?;
            let _ = File::create(&accounts_file_path).map_err(|err| {
                Box::new(ErrorKind::AccountsFile(
                    "Unable to create accounts file".into(),
                    err,
                ))
            })?;
        }

        Ok(accounts_file_path)
    }

    pub fn commands(&self) -> &Commands {
        &self.cmds
    }
}

#[derive(Debug, StructOpt)]
pub enum Commands {
    /// Create, edit, or delete accounts
    #[structopt(name = "acct")]
    Account(AccountOpts),
}

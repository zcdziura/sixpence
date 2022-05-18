use std::path::PathBuf;

use clap::{Args, ValueHint};

use crate::error::Error;

#[derive(Args, Debug)]
pub struct GlobalArgs {
    /// Specify an alternate ledger file
    #[clap(short = 'f', long = "file", value_name = "PATH", value_hint = ValueHint::DirPath, global = true)]
    ledger_file: Option<PathBuf>,
}

impl GlobalArgs {
    pub fn ledger_file(&self) -> Result<PathBuf, Error> {
        match self.ledger_file.as_ref() {
            Some(path) => match path.exists() {
                true => Ok(path.clone()),
                false => Err(Error::ledger_file_not_found(path)),
            },
            None => Ok(dirs::data_local_dir()
                .map(|dir| {
                    let mut dir = dir;
                    dir.push(format!("{}/ledger.dat", env!("CARGO_PKG_NAME")));
                    dir
                })
                .unwrap()),
        }
    }
}

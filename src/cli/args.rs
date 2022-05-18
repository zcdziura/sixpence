use std::path::PathBuf;

use clap::{Args, ValueHint};

use crate::error::Error;

#[derive(Args, Debug)]
pub struct GlobalArgs {
    /// The path to the ledger file
    #[clap(short = 'f', long = "file", value_name = "PATH", value_hint = ValueHint::DirPath, global = true)]
    ledger_file: Option<PathBuf>,
}

impl GlobalArgs {
    pub fn ledger_file(&self) -> Result<PathBuf, Error> {
        match self.ledger_file.as_ref() {
            Some(path) => match path.exists() {
                true => Ok(path.clone()),
                false => Err(Error::ledger_file_not_found(PathBuf::from(path))),
            },
            None => Ok(dirs::data_local_dir()
                .map(|path| {
                    let mut path = path;
					path.push(env!("CARGO_PKG_NAME"));
                    path.push("ledger.dat");
                    path
                })
                .unwrap()),
        }
    }
}

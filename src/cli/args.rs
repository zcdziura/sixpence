use std::{
    fs::{DirBuilder, File},
    io,
    path::{Path, PathBuf},
};

use clap::{Args, ValueHint};
use platform_dirs::AppDirs;

use crate::error::Error;

#[derive(Args, Debug)]
pub struct GlobalArgs {
    /// Specify an alternate ledger file
    #[clap(short, long = "ledger", value_name = "FILE", value_hint = ValueHint::DirPath)]
    ledger_file: Option<PathBuf>,
}

impl GlobalArgs {
    pub fn ledger_file(&self) -> Result<PathBuf, Error> {
        let path = match self.ledger_file.as_ref() {
            Some(path) => {
                if !path.exists() {
                    create_full_file_path(path.as_path())?
                } else {
                    path.clone()
                }
            }
            None => AppDirs::new(Some(env!("CARGO_PKG_NAME")), true)
                .map(|app_dirs| app_dirs.data_dir.as_path().join("ledger.dat"))
                .unwrap(),
        };

        Ok(path)
    }
}

fn create_full_file_path(path: &Path) -> io::Result<PathBuf> {
    let parent_dir = path.parent().unwrap();
    let _ = DirBuilder::new().recursive(true).create(parent_dir)?;
    let _ = File::create(path)?;

    Ok(PathBuf::from(path))
}

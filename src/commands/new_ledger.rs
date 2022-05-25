use std::{
    fs::{DirBuilder, OpenOptions},
    path::Path,
};

use crate::error::Error;

pub fn new_ledger<'p>(ledger_file_path: &'p Path) -> Result<(), Error> {
    let parent = ledger_file_path
        .parent()
        .ok_or(Error::invalid_ledger_file(ledger_file_path.to_owned()))?;
    DirBuilder::new().recursive(true).create(parent)?;

    OpenOptions::new()
        .create(true)
        .write(true)
        .open(ledger_file_path)?;

    println!(
        "New ledger file created at: {}",
        ledger_file_path.to_str().unwrap()
    );

    Ok(())
}

use std::{path::Path, fs::{DirBuilder, OpenOptions}};

use crate::error::Error;

pub fn create_new_ledger<'p>(ledger_file_path: &'p Path) -> Result<(), Error> {
    let parent = ledger_file_path
        .parent()
        .ok_or(Error::invalid_ledger_file(ledger_file_path))?;
    DirBuilder::new().recursive(true).create(parent)?;

    OpenOptions::new()
        .create(true)
        .write(true)
        .open(ledger_file_path)?;

    println!("New ledger file created at: {}", ledger_file_path.to_str().unwrap());

    Ok(())
}
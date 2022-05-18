use std::{
    fs::{DirBuilder, OpenOptions},
    path::Path,
};

use clap::Subcommand;

use crate::error::Error;

#[derive(Debug, Subcommand)]

pub enum LedgerCommands {
    /// Generate a new, empty ledger
    New,
}

pub fn process<'c, 'p>(
    ledger_commands: &'c LedgerCommands,
    ledger_file_path: &'p Path,
) -> Result<(), Error<'p>> {
    match ledger_commands {
        LedgerCommands::New => create_ledger_file(ledger_file_path),
    }
}

fn create_ledger_file<'p>(ledger_file_path: &'p Path) -> Result<(), Error> {
    let parent = ledger_file_path
        .parent()
        .ok_or(Error::invalid_ledger_file(ledger_file_path))?;
    DirBuilder::new().recursive(true).create(parent)?;

    OpenOptions::new()
        .create(true)
        .write(true)
        .open(ledger_file_path)?;

    println!("New ledger file created at: {:?}", ledger_file_path);

    Ok(())
}

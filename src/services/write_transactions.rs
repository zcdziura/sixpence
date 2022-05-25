use std::{fs::OpenOptions, io::Write, path::Path};

use crate::{error::Error, transaction::Transaction};

pub fn write_transactions<'p>(
    ledger_file_path: &'p Path,
    transactions: &[Transaction],
) -> Result<(), Error> {
    let mut file = OpenOptions::new().write(true).open(ledger_file_path)?;
    let transactions = transactions
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<String>>()
        .join("\n\n");

    write!(file, "{}", transactions)?;

    Ok(())
}

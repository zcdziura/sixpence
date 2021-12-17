use std::{fs, path::Path};

use crate::error::Error;

use super::Transaction;

pub fn save_transaction(path: &Path, transaction: &Transaction) -> Result<(), Error> {
    let buffer = bincode::serialize(transaction)?;
    let _ = fs::write(path, buffer)?;

    Ok(())
}

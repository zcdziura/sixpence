use std::{fs, path::Path};

use crate::error::Error;

use super::Account;

pub fn save_accounts(path: &Path, accounts: &[Account]) -> Result<(), Error> {
    let buffer = bincode::serialize(accounts)?;
    let _ = fs::write(path, buffer)?;

    Ok(())
}

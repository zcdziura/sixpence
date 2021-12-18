use std::{fs, path::Path};

use crate::error::Error;

use super::Account;

pub fn read_accounts_from_file(path: &Path) -> Result<Vec<Account>, Error> {
    let buffer = fs::read(path)?;
    let deencoded_buffer: Vec<Account> = if buffer.is_empty() {
        Vec::new()
    } else {
        bincode::deserialize(&buffer[..])?
    };

    Ok(deencoded_buffer)
}

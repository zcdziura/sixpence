use std::{
    fs::{self, File},
    io::Read,
    path::Path,
};

use crate::{error::Error, transaction::Transaction};

pub fn read_transactions<'p>(ledger_file_path: &'p Path) -> Result<Vec<Transaction>, Error> {
    let mut transactions = Vec::<Transaction>::new();
    let mut buffer = Vec::<u8>::with_capacity(64);
    let mut is_end_of_line = false;
    let ledger_file = File::open(ledger_file_path)?;

    // WARNING! Clever code incoming!
    if fs::metadata(ledger_file_path).map_or(false, |metadata| match metadata.len() {
        0 => false,
        _ => true,
    }) {
        let mut byte_iter = ledger_file.bytes().peekable();
        while let Some(res) = byte_iter.next() {
            match res {
                Ok(byte) => {
                    if (byte == ('\n' as u8) && is_end_of_line) || byte_iter.peek().is_none() {
                        if byte_iter.peek().is_none() {
                            buffer.push(byte);
                        }

                        match String::from_utf8(buffer.clone())
                            .map_err(|_| Error::corrupted_ledger_file())
                            .and_then(Transaction::try_from)
                        {
                            Ok(transaction) => {
                                buffer.clear();
                                is_end_of_line = false;
                                transactions.push(transaction);
                            }
                            Err(err) => {
                                return Err(err);
                            }
                        }
                    } else if byte == ('\n' as u8) && !is_end_of_line {
                        buffer.push(byte);
                        is_end_of_line = true;
                    } else {
                        buffer.push(byte);
                        is_end_of_line = false;
                    }
                }
                Err(error) => return Err(Error::from(error)),
            }
        }
    }

    Ok(transactions)
}

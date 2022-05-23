use std::{fs::File, io::Read, path::Path};

use crate::{error::Error, transaction::Transaction};

pub fn read_transactions<'p>(ledger_file_path: &'p Path) -> Result<Vec<Transaction>, Error> {
    let mut transactions = Vec::<Transaction>::new();
    let mut buffer = Vec::<u8>::with_capacity(64);
    let mut is_end_of_line = false;
    let ledger_file = File::open(ledger_file_path)?;
    for byte_result in ledger_file.bytes() {
        match byte_result {
            Ok(byte) => {
                if byte != ('\n' as u8) {
                    is_end_of_line = false;
                    buffer.push(byte);
                } else {
                    if !is_end_of_line {
                        is_end_of_line = true;
                        buffer.push(byte);
                    } else {
                        match String::from_utf8(buffer.clone())
                            .map_err(|_| Error::corrupted_ledger_file())
                            .and_then(Transaction::try_from)
                        {
                            Ok(transaction) => {
                                buffer.clear();
                                transactions.push(transaction);
                            }
                            Err(err) => {
                                return Err(err);
                            }
                        }
                    }
                }
            }
            Err(error) => return Err(Error::from(error)),
        }
    }

    Ok(transactions)
}

use std::path::{Path, PathBuf};

use crate::{
    cli::TransactionOpts,
    error::Error,
    services::{read_transactions, write_transactions},
    transaction::Transaction,
};

pub fn new_transaction<'p>(
    ledger_file_path: &'p Path,
    opts: &TransactionOpts,
) -> Result<(), Error> {
    if !ledger_file_path.exists() {
        return Err(Error::ledger_file_not_found(PathBuf::from(
            ledger_file_path,
        )));
    }

    let mut transactions = read_transactions(ledger_file_path)?;
    let new_transaction = Transaction::try_from(opts)?;

    transactions.push(new_transaction);
    transactions.sort();

    write_transactions(ledger_file_path, transactions.as_slice())?;

    Ok(())
}

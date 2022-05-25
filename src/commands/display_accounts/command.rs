use std::path::{Path, PathBuf};

use crate::{error::Error, services::read_transactions};

use super::reconcile_accounts::reconcile_accounts;

pub fn command<'p>(ledger_file_path: &'p Path) -> Result<(), Error> {
    if !ledger_file_path.exists() {
        return Err(Error::ledger_file_not_found(PathBuf::from(
            ledger_file_path,
        )));
    }

    let transactions = read_transactions(ledger_file_path)?;
    let accounts = reconcile_accounts(transactions.as_slice());

    let starting_padding = accounts
        .iter()
        .map(|account| account.name().len())
        .fold(0, |a, b| if a >= b { a } else { b })
        + 4;

    accounts.iter().for_each(|account| {
        println!(
            "{:.<width$}{}",
            account.name(),
            account.format_value_as_currency(),
            width = starting_padding
        )
    });

    Ok(())
}

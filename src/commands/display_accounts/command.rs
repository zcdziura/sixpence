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
    if transactions.is_empty() {
        println!("No transactions found in the ledger.");
        return Ok(());
    }

    let accounts = reconcile_accounts(transactions.as_slice());

    let (longest_account_length, longest_value_length) = accounts
        .iter()
        .map(|account| {
            (
                account.name().len(),
                account.format_value_as_currency().len(),
            )
        })
        .fold((0, 0), |(account_a, amount_a), (account_b, amount_b)| {
            let account = if account_a >= account_b {
                account_a
            } else {
                account_b
            };

            let amount = if amount_a >= amount_b {
                amount_a
            } else {
                amount_b
            };

            (account, amount)
        });

    accounts.iter().for_each(|account| {
        println!(
            "{:.<first_width$}{:^second_width$}",
            account.name(),
            account.format_value_as_currency(),
            first_width = longest_account_length + 4,
            second_width = longest_value_length
        )
    });

    Ok(())
}

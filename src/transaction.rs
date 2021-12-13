// use std::path::PathBuf;

use std::{collections::BTreeSet, ops::Neg, path::PathBuf};

use chrono::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    account::read_accounts_from_file,
    error::{Error, ErrorKind},
    opts::transaction::NewTransactionOpts,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct Transaction {
    timestamp: DateTime<Utc>,
    debits: Vec<(String, usize)>,
    credits: Vec<(String, usize)>,
}

pub fn validate_new_transaction_opts(
    accounts_file_path: PathBuf,
    opts: &NewTransactionOpts,
) -> Result<(), Error> {
    let accounts_in_transaction = extract_accounts_in_transaction(opts);
    let existing_accounts = load_existing_accounts(&accounts_file_path);
    validate_accounts_exist(&existing_accounts, &accounts_in_transaction)?;

    let (total_debits, total_credits) = extract_total_debits_and_credits(opts);
    validate_values_balance(total_debits, total_credits)?;

    Ok(())
}

fn extract_accounts_in_transaction(opts: &NewTransactionOpts) -> BTreeSet<&String> {
    opts.debits()
        .iter()
        .chain(opts.credits().iter())
        .map(|pair| pair.0)
        .collect::<BTreeSet<_>>()
}

fn load_existing_accounts(accounts_file_path: &PathBuf) -> BTreeSet<String> {
    let accounts_file = read_accounts_from_file(accounts_file_path).unwrap();
    let account_names = accounts_file
        .into_iter()
        .map(|account| account.name())
        .collect::<BTreeSet<_>>();

    account_names
}

fn validate_accounts_exist(
    existing_accounts: &BTreeSet<String>,
    accounts: &BTreeSet<&String>,
) -> Result<(), Error> {
    if accounts.is_subset(existing_accounts) {
        Ok(())
    } else {
        let difference = accounts
            .difference(&existing_accounts)
            .take(1)
            .collect::<Vec<_>>()
            .first()
            .unwrap()
            .to_string();
        Err(Box::new(ErrorKind::UnknownAccount(difference)))
    }
}

fn extract_total_debits_and_credits(opts: &NewTransactionOpts) -> (isize, isize) {
    opts.debits()
        .into_iter()
        .zip(opts.credits().into_iter())
        .map(|((_, debit_val), (_, credit_val))| (debit_val, credit_val))
        .fold(
            (0_isize, 0_isize),
            |(acc_debit_val, acc_credit_val), (cur_debit_val, cur_credit_val): (&isize, &isize)| {
                (
                    acc_debit_val + cur_debit_val,
                    acc_credit_val + cur_credit_val,
                )
            },
        )
}

fn validate_values_balance(total_debits: isize, total_credits: isize) -> Result<(), Error> {
    let total_debits = if total_debits == 0 && total_credits != 0 {
        total_credits.neg()
    } else {
        total_debits
    };

    let total_credits = if total_credits == 0 && total_debits != 0 {
        total_debits.neg()
    } else {
        total_credits
    };

    match total_debits + total_credits {
        0 => Ok(()),
        _ => Err(Box::new(ErrorKind::UnbalancedTransaction(
            total_debits,
            total_credits,
        ))),
    }
}

// pub fn create_new_transaction(
// 	path: PathBuf,
// 	debits: Vec<(String, usize)>,
// 	credits: Vec<(String, usize)>,
// ) {

// }

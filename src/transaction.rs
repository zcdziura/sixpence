// use std::path::PathBuf;

use std::{
    collections::BTreeSet,
    path::{Path, PathBuf},
};

use chrono::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{account::read_accounts_from_file, opts::transaction::NewTransactionOpts};

#[derive(Debug, Deserialize, Serialize)]
pub struct Transaction {
    timestamp: DateTime<Utc>,
    debits: Vec<(String, usize)>,
    credits: Vec<(String, usize)>,
}

pub fn validate_new_transaction_opts(path: PathBuf, opts: &NewTransactionOpts) {
    let accounts_in_transaction = opts
        .debits()
        .iter()
        .chain(opts.credits().iter())
        .map(|pair| pair.0)
        .collect::<Vec<_>>();

    check_accounts_exist(path.as_path(), accounts_in_transaction.as_ref());

    let (total_debits, total_credits) = opts
        .debits()
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
        );
}

fn check_accounts_exist(path: &Path, accounts: &[&String]) {
	let existing_accounts = read_accounts_from_file(path).unwrap();
    let existing_accounts = existing_accounts
        .iter()
        .map(|account| account.name())
        .collect::<BTreeSet<_>>();

    let accounts = BTreeSet::from_iter(accounts.iter().cloned());
    let intersection = existing_accounts
        .intersection(&accounts)
        .cloned()
        .collect::<BTreeSet<_>>();

    if accounts == intersection {
        println!("Hooray!");
    } else {
        println!("Boo...")
    }
}

// pub fn create_new_transaction(
// 	path: PathBuf,
// 	debits: Vec<(String, usize)>,
// 	credits: Vec<(String, usize)>,
// ) {

// }

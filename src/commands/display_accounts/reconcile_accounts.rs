use itertools::Itertools;

use crate::{
    account::Account,
    transaction::{Entry, Transaction},
};

pub fn reconcile_accounts(transactions: &[Transaction]) -> Vec<Account> {
    let mut accounts = Vec::<Account>::new();
    let grouped_entries = transactions
        .iter()
        .flat_map(|transaction| transaction.entries())
        .group_by(|entry| entry.account());

    for (_, entries) in grouped_entries.into_iter() {
        let account = Account::from(entries.collect::<Vec<&Entry>>().as_slice());
        accounts.push(account);
    }

    accounts.sort();
    accounts
}

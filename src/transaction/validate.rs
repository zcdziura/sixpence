use std::{collections::HashSet, ops::Neg, path::Path, rc::Rc};

use crate::{
    account::read_accounts_from_file,
    args_old::transaction::{NewTransactionOpts, RecurringPeriod},
    error::Error,
};

pub fn validate_new_transaction_opts(
    accounts_file_path: &Path,
    opts: &NewTransactionOpts,
) -> Result<
    (
        Vec<(Rc<String>, isize)>,
        Vec<(Rc<String>, isize)>,
        RecurringPeriod,
    ),
    Error,
> {
    let (debits, credits) = opts.categorize_accounts_and_values()?;
    let accounts_in_transaction = get_account_names(&debits, &credits);
    let saved_accounts = load_saved_accounts(accounts_file_path)?;
    validate_accounts_exist(&saved_accounts, &accounts_in_transaction)?;

    let (total_debits, total_credits) = extract_total_debits_and_credits(&debits, &credits);
    validate_values_balance(total_debits, total_credits)?;

    Ok((debits, credits, opts.recurring_period()))
}

fn get_account_names(
    debits: &[(Rc<String>, isize)],
    credits: &[(Rc<String>, isize)],
) -> HashSet<Rc<String>> {
    debits
        .iter()
        .map(|pair| pair.0.clone())
        .chain(credits.iter().map(|pair| pair.0.clone()))
        .collect::<HashSet<_>>()
}

fn load_saved_accounts(accounts_file_path: &Path) -> Result<HashSet<Rc<String>>, Error> {
    Ok(read_accounts_from_file(accounts_file_path)?
        .into_iter()
        .map(|account| account.name())
        .collect::<HashSet<_>>())
}

fn validate_accounts_exist(
    existing_accounts: &HashSet<Rc<String>>,
    accounts: &HashSet<Rc<String>>,
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
            .as_str();

        Err(Error::unknown_account(difference))
    }
}

fn extract_total_debits_and_credits(
    debits: &[(Rc<String>, isize)],
    credits: &[(Rc<String>, isize)],
) -> (isize, isize) {
    debits
        .into_iter()
        .map(|pair| pair.1)
        .zip(credits.into_iter().map(|pair| pair.1))
        .fold(
            (0_isize, 0_isize),
            |(acc_debit_val, acc_credit_val), (cur_debit_val, cur_credit_val): (isize, isize)| {
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
        _ => Err(Error::unbalanced_transaction(total_debits, total_credits)),
    }
}

use super::{Account, AccountType};

pub fn filter_accounts_by_account_type(
    accounts: Vec<Account>,
    account_type: &AccountType,
) -> Vec<Account> {
    accounts
        .into_iter()
        .filter(|account| account.account_type() == *account_type)
        .collect()
}

mod display;
mod filter;
mod read;
mod save;

use std::{path::Path, rc::Rc, str::FromStr};

use serde::{Deserialize, Serialize};

use crate::error::Error;

pub use display::display_accounts;
pub use filter::filter_accounts_by_account_type;
pub use read::read_accounts_from_file;
pub use save::save_accounts;

#[derive(Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Account {
    name: Rc<String>,
    account_type: AccountType,
}

impl Account {
    pub fn new(name: String, account_type: AccountType) -> Self {
        Self {
            name: Rc::new(name),
            account_type,
        }
    }

    pub fn name(&self) -> Rc<String> {
        self.name.clone()
    }

    pub fn account_type(&self) -> AccountType {
        self.account_type
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum AccountType {
    Asset,
    Equity,
    Expense,
    Liability,
    Revenue,
}

impl AccountType {
    pub fn len(&self) -> usize {
        match self {
            Self::Asset => "asset",
            Self::Equity => "equity",
            Self::Expense => "expense",
            Self::Liability => "liability",
            Self::Revenue => "revenue",
        }
        .len()
    }
}

impl FromStr for AccountType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "asset" => Ok(Self::Asset),
            "equity" => Ok(Self::Equity),
            "expense" => Ok(Self::Expense),
            "liability" => Ok(Self::Liability),
            "revenue" => Ok(Self::Revenue),
            _ => Err(Error::account_type()),
        }
    }
}

impl ToString for AccountType {
    fn to_string(&self) -> String {
        match self {
            Self::Asset => "Asset",
            Self::Equity => "Equity",
            Self::Expense => "Expense",
            Self::Liability => "Liability",
            Self::Revenue => "Revenue",
        }
        .to_string()
    }
}

pub fn create_new_account(
    path: &Path,
    name: String,
    account_type: AccountType,
) -> Result<(), Error> {
    let mut accounts = read_accounts_from_file(path)?;
    let new_account = Account::new(name, account_type);
    accounts.push(new_account);
    accounts.sort();

    let accounts_vec = accounts.into_iter().collect::<Vec<_>>();
    save_accounts(path, accounts_vec.as_slice())?;

    Ok(())
}

pub fn list_accounts(path: &Path, account_type: Option<AccountType>) -> Result<(), Error> {
    let accounts = read_accounts_from_file(path)?;

    let mut filtered_accounts = if account_type.is_none() {
        accounts
    } else {
        filter_accounts_by_account_type(accounts, account_type.as_ref().unwrap())
    };

    filtered_accounts.sort_by_key(|account| (account.account_type(), account.name()));

    display_accounts(filtered_accounts.as_slice());

    Ok(())
}

pub fn edit_account(
    _account_name: &str,
    _new_name: Option<&str>,
    _new_account_type: Option<AccountType>,
) -> Result<(), Error> {
    Ok(())
}

use std::{collections::BTreeSet, fs, path::Path, rc::Rc, str::FromStr};

use serde::{Deserialize, Serialize};

use crate::error::{Error, ErrorKind};

#[derive(Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
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
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub enum AccountType {
    Asset,
    Equity,
    Expense,
    Liability,
    Revenue,
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
            _ => Err(Error::from(ErrorKind::AccountType)),
        }
    }
}

pub fn create_new_account(
    path: &Path,
    name: String,
    account_type: AccountType,
) -> Result<(), Error> {
    let mut accounts = read_accounts_from_file(path)?;
    let new_account = Account::new(name, account_type);
    accounts.insert(new_account);

    let accounts_vec = accounts.into_iter().collect::<Vec<_>>();
    save_accounts_to_file(path, accounts_vec.as_slice())?;

    Ok(())
}

pub fn read_accounts_from_file(path: &Path) -> Result<BTreeSet<Account>, Error> {
    let buffer = fs::read(path)?;
    let deencoded_buffer: Vec<Account> = if buffer.is_empty() {
        Vec::new()
    } else {
        bincode::deserialize(&buffer[..])?
    };

    Ok(deencoded_buffer.into_iter().collect::<BTreeSet<_>>())
}

fn save_accounts_to_file(path: &Path, accounts: &[Account]) -> Result<(), Error> {
    let buffer = bincode::serialize(accounts)?;
    let _ = fs::write(path, buffer)?;

    Ok(())
}

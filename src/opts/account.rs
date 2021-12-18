use structopt::StructOpt;

use crate::account::AccountType;

#[derive(Debug, StructOpt)]
pub enum AccountOpts {
    /// Create a new account
    #[structopt(name = "new")]
    NewAccount(NewAccountOpts),

    /// List all saved accounts
    #[structopt(name = "list")]
    ListAccounts(ListAccountOpts),
}

#[derive(Debug, StructOpt)]
pub struct NewAccountOpts {
    /// Name of account; e.g. "Checking Account", "Paycheck", "Electric Bill"
    #[structopt(short, long)]
    name: String,

    /// Type of account: [asset | equity | expense | liability | revenue]
    #[structopt(short = "t", long = "type")]
    account_type: AccountType,
}

impl NewAccountOpts {
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn account_type(&self) -> AccountType {
        self.account_type
    }
}

#[derive(Debug, StructOpt)]
pub struct ListAccountOpts {
    /// Filter by account type: [asset | equity | expense | liability | revenue]
    #[structopt(short = "t", long = "type")]
    account_type: Option<AccountType>,
}

impl ListAccountOpts {
    pub fn account_type(&self) -> Option<AccountType> {
        self.account_type
    }
}

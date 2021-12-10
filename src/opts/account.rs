use structopt::StructOpt;

use crate::account::AccountType;

#[derive(Debug, StructOpt)]
pub enum AccountOpts {
    /// Create a new account
    #[structopt(name = "new")]
    NewAccount(NewAccountOpts),
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
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn account_type(&self) -> AccountType {
        self.account_type
    }
}

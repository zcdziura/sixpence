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

	/// Edit an existing account
	EditAccount(EditAccoutOpts)
}

#[derive(Debug, StructOpt)]
pub struct NewAccountOpts {
    /// Name of account; e.g. "Checking Account", "Paycheck", "Electric Bill"
    #[structopt()]
    name: String,

    /// Type of account: [asset | equity | expense | liability | revenue]
    #[structopt()]
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

#[derive(Debug, StructOpt)]
pub struct EditAccoutOpts {
	/// The account to edit
	#[structopt()]
	name: String,

	/// New name for the account
	#[structopt(short = "n", long = "name")]
	new_name: Option<String>,

	/// New type for the account
	#[structopt(short = "t", long = "type")]
	new_account_type: Option<AccountType>
}

impl EditAccoutOpts {
	pub fn name(&self) -> &str {
		self.name.as_ref()
	}

	pub fn new_name(&self) -> Option<&str> {
		self.new_name.as_ref().map(|name| name.as_str())
	}

	pub fn new_account_type(&self) -> Option<AccountType> {
		self.new_account_type
	}
}

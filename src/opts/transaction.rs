use std::{rc::Rc, str::FromStr};

use structopt::StructOpt;

use crate::error::{Error, ErrorKind};

#[derive(Debug, StructOpt)]
#[structopt(global_setting = structopt::clap::AppSettings::AllowNegativeNumbers)]
pub enum TransactionOpts {
    /// Create a new transaction
    #[structopt(
        name = "new",
        usage = "sixp txn new -a [credit] -v=[negative value] -a [debit] -v=[positive value]"
    )]
    NewTransaction(NewTransactionOpts),
}

#[derive(Debug, StructOpt)]
pub struct NewTransactionOpts {
    /// Account where money is going to or coming from (e.g. "Checking Account", "Mortgage", "Credit Card")
    #[structopt(short = "a", long = "account", required = true)]
    accounts: Vec<String>,

    /// Value being transferred; positive values are treated as debits, negative values are treated as credits
    #[structopt(short = "v", long = "value", required = true)]
    values: Vec<isize>,

    /// Set recurring period for future transactions
    #[structopt(short, long = "recurring", default_value = "onetime")]
    recurring_period: RecurringPeriod,
}

impl NewTransactionOpts {
    pub fn debits(&self) -> Vec<(Rc<String>, isize)> {
        self.filter_accounts_by(|(_, value)| *value >= 0)
    }

    pub fn credits(&self) -> Vec<(Rc<String>, isize)> {
        self.filter_accounts_by(|(_, value)| *value < 0)
    }

    fn filter_accounts_by<F>(&self, predicate: F) -> Vec<(Rc<String>, isize)>
    where
        F: FnMut(&(Rc<String>, isize)) -> bool,
    {
        self.accounts
            .iter()
            .map(|account| Rc::new(account.to_string()))
            .zip(self.values.iter().map(|value| *value))
            .filter(predicate)
            .collect()
    }
}

#[derive(Debug, StructOpt)]
pub enum RecurringPeriod {
    OneTime,
    Weekly,
    BiWeekly,
    Monthly,
    Annually,
}

impl Default for RecurringPeriod {
    fn default() -> Self {
        Self::OneTime
    }
}

impl FromStr for RecurringPeriod {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "onetime" => Ok(Self::OneTime),
            "weekly" => Ok(Self::Weekly),
            "biweekly" => Ok(Self::BiWeekly),
            "monthly" => Ok(Self::Monthly),
            "annually" => Ok(Self::Annually),
            _ => Err(Box::new(ErrorKind::RecurringPeriod(s.to_string()))),
        }
    }
}

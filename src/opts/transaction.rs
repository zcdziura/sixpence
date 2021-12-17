use std::{collections::HashSet, iter, ops::Neg, rc::Rc, str::FromStr};

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
    values: Vec<String>,

    /// Set recurring period for future transactions
    #[structopt(short, long = "recurring", default_value = "onetime")]
    recurring_period: RecurringPeriod,
}

impl NewTransactionOpts {
    pub fn recurring_period(&self) -> RecurringPeriod {
        self.recurring_period
    }

    pub fn categorize_accounts_and_values(
        &self,
    ) -> Result<(Vec<(Rc<String>, isize)>, Vec<(Rc<String>, isize)>), Error> {
        let accounts = self.get_accounts();
        let values = self.get_values(accounts.len());

        let mut combined = combine_accounts_and_values(accounts, values);

        let mut debits = extract_debits(&combined);
        combined = keep_differences(combined, &debits);

        let mut credits = extract_credits(&combined);
        let mut remaining_accounts_without_value = keep_differences(combined, &credits)
            .into_iter()
            .collect::<Vec<_>>();

        if remaining_accounts_without_value.len() > 1 {
            let accounts_without_value = remaining_accounts_without_value
                .into_iter()
                .map(|pair| Rc::try_unwrap(pair.0).unwrap())
                .collect::<Vec<_>>();

            return Err(Box::new(ErrorKind::AccountsWithoutValue(
                accounts_without_value,
            )));
        }

        let account = remaining_accounts_without_value.pop().unwrap().0.clone();
        let total_credits = get_total_values(&credits);
        let total_debits = get_total_values(&debits);

        if total_debits < total_credits {
            debits.insert((account, (total_credits - total_debits).neg()));
        } else if total_credits < total_debits {
            credits.insert((account, (total_debits - total_credits).neg()));
        }

        Ok((debits.into_iter().collect(), credits.into_iter().collect()))
    }

    fn get_accounts(&self) -> Vec<Rc<String>> {
        self.accounts
            .iter()
            .map(|account| Rc::new(account.clone()))
            .collect::<Vec<_>>()
    }

    fn get_values(&self, total_accounts: usize) -> Vec<isize> {
        self.values
            .iter()
            .map(|value| {
                value
                    .chars()
                    .filter(|c| *c == '-' || c.is_numeric())
                    .collect::<String>()
                    .parse::<isize>()
                    .unwrap()
            })
            .chain(iter::repeat(0_isize))
            .take(total_accounts)
            .collect::<Vec<_>>()
    }
}

#[derive(Clone, Copy, Debug, StructOpt)]
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

fn combine_accounts_and_values(
    accounts: Vec<Rc<String>>,
    values: Vec<isize>,
) -> HashSet<(Rc<String>, isize)> {
    accounts
        .into_iter()
        .zip(values.into_iter())
        .collect::<HashSet<_>>()
}

fn extract_debits(all_accounts: &HashSet<(Rc<String>, isize)>) -> HashSet<(Rc<String>, isize)> {
    let mut debits = HashSet::<(Rc<String>, isize)>::new();
    all_accounts.iter().for_each(|pair| {
        if pair.1 > 0 {
            debits.insert((pair.0.clone(), pair.1));
        }
    });

    debits
}

fn extract_credits(all_accounts: &HashSet<(Rc<String>, isize)>) -> HashSet<(Rc<String>, isize)> {
    let mut credits = HashSet::<(Rc<String>, isize)>::new();
    all_accounts.iter().for_each(|pair| {
        if pair.1 < 0 {
            credits.insert((pair.0.clone(), pair.1));
        }
    });

    credits
}

fn keep_differences(
    left: HashSet<(Rc<String>, isize)>,
    right: &HashSet<(Rc<String>, isize)>,
) -> HashSet<(Rc<String>, isize)> {
    left.difference(&right)
        .map(|pair| (pair.0.clone(), pair.1))
        .collect::<HashSet<_>>()
}

fn get_total_values(accounts: &HashSet<(Rc<String>, isize)>) -> isize {
    accounts
        .iter()
        .map(|(_, value)| *value)
        .reduce(|acc, cur| acc + cur)
        .unwrap_or_default()
}

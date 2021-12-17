mod save;
mod validate;

use std::{path::Path, rc::Rc};

use chrono::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    error::Error,
    opts::transaction::{self as transaction_opts, NewTransactionOpts},
};

pub use save::save_transaction;
pub use validate::validate_new_transaction_opts;

#[derive(Debug, Deserialize, Serialize)]
pub struct Transaction {
    timestamp: DateTime<Utc>,
    debits: Vec<(Rc<String>, isize)>,
    credits: Vec<(Rc<String>, isize)>,
    recurring_period: RecurringPeriod,
}

impl Transaction {
    pub fn new(
        debits: Vec<(Rc<String>, isize)>,
        credits: Vec<(Rc<String>, isize)>,
        recurring_period: transaction_opts::RecurringPeriod,
    ) -> Self {
        Self {
            timestamp: Utc::now(),
            debits,
            credits,
            recurring_period: recurring_period.into(),
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
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

impl From<crate::opts::transaction::RecurringPeriod> for RecurringPeriod {
    fn from(other: transaction_opts::RecurringPeriod) -> Self {
        match other {
            transaction_opts::RecurringPeriod::OneTime => Self::OneTime,
            transaction_opts::RecurringPeriod::Weekly => Self::Weekly,
            transaction_opts::RecurringPeriod::BiWeekly => Self::BiWeekly,
            transaction_opts::RecurringPeriod::Monthly => Self::Monthly,
            transaction_opts::RecurringPeriod::Annually => Self::Annually,
        }
    }
}

pub fn create_new_transaction(
    accounts_file: &Path,
    ledger_file: &Path,
    new_transaction_opts: &NewTransactionOpts,
) -> Result<(), Error> {
    let (debits, credits, recurring_period) =
        validate_new_transaction_opts(accounts_file, new_transaction_opts)?;

    let new_transaction = Transaction::new(debits, credits, recurring_period);
    save_transaction(ledger_file, &new_transaction)?;

    Ok(())
}

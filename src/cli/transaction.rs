use chrono::{Date, DateTime, Local, TimeZone, Utc};
use clap::Args;
use getset::{CopyGetters, Getters};

use crate::error::Error;

#[derive(Args, CopyGetters, Debug, Getters)]
pub struct TransactionOpts {
    /// Date of the transaction
    #[clap(short = 'd', long = "date", value_name = "YYYY-MM-DD", parse(try_from_str = parse_date))]
    date: Option<Date<Local>>,

    /// Mark transaction as not having cleared through the bank
    #[getset(get_copy = "pub")]
    #[clap(short = 'c', long = "cleared")]
    has_not_cleared: bool,

    /// Description of the transaction
    #[getset(get = "pub")]
    #[clap(short = 'D', long = "desc", value_name = "DESCRIPTION")]
    description: Option<String>,

    /// Transaction entries
    #[getset(get = "pub")]
    #[clap(value_name = "ACCOUNT=VALUE", parse(try_from_str = parse_entries), multiple_occurrences(true))]
    entries: Vec<(String, isize)>,
}

impl TransactionOpts {
    pub fn date(&self) -> Date<Utc> {
        self.date
            .map(|date| Utc.from_local_date(&date.naive_local()).unwrap())
            .unwrap_or(Utc::now().date())
    }
}

fn parse_date(s: &str) -> Result<Date<Local>, Error> {
    Ok(DateTime::<Local>::from(DateTime::parse_from_str(s, "%Y-%m-%d")?).date())
}

fn parse_entries(s: &str) -> Result<(String, isize), Error> {
    let pos = s.find("=").ok_or_else(|| Error::invalid_entry_format(s))?;
    let account = s[..pos]
        .parse::<String>()
        .map_err(|_| Error::invalid_entry_format(s))?;
    let value = s[pos + 1..]
        .parse::<isize>()
        .map_err(|_| Error::invalid_entry_format(s))?;

    Ok((account, value))
}

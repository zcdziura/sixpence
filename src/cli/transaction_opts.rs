use chrono::{Date, DateTime, Local, TimeZone, Utc};
use clap::Args;
use getset::{CopyGetters, Getters};
use itertools::Itertools;

use crate::error::Error;

#[derive(Args, CopyGetters, Debug, Getters)]
pub struct TransactionOpts {
    /// Date of the transaction
    #[clap(short = 'd', long = "date", value_name = "YYYY-MM-DD", parse(try_from_str = parse_date))]
    date: Option<Date<Local>>,

    /// Mark transaction as not having cleared through the bank
    #[getset(get_copy = "pub")]
    #[clap(short = 'n', long = "not-cleared")]
    has_not_cleared: bool,

    /// Description of the transaction
    #[getset(get = "pub")]
    #[clap(short = 'D', long = "desc", value_name = "DESCRIPTION")]
    description: Option<String>,

    /// Transaction entries; only one entry may leave out its value
    #[getset(get = "pub")]
    #[clap(value_name = "ACCOUNT[=VALUE]", parse(try_from_str = parse_entries), multiple_occurrences(true))]
    entries: Vec<(String, Option<isize>)>,
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

fn parse_entries(s: &str) -> Result<(String, Option<isize>), Error> {
    match s.find("=") {
        Some(_) => {
            let (account, value) = s
                .split("=")
                .map(|value| value.split(".").join(""))
                .enumerate()
                .fold((String::new(), String::new()), |tup, (idx, cur)| {
                    let (account, value) = tup;
                    if idx == 0 {
                        (cur, value)
                    } else {
                        (account, cur)
                    }
                });

            let value = value
                .parse::<isize>()
                .map_err(|_| Error::invalid_entry_format(s))?;

            Ok((account, Some(value)))
        }
        None => Ok((s.to_string(), None)),
    }
}

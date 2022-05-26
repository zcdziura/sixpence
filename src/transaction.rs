use std::{fmt::Display, ops::Neg};

use chrono::{Date, NaiveDate, TimeZone, Utc};
use getset::{CopyGetters, Getters};
use ulid::Ulid;

use crate::{cli::TransactionOpts, error::Error};

#[derive(CopyGetters, Debug, Eq, Getters, PartialEq, Ord)]
pub struct Transaction {
    #[getset(get_copy = "pub")]
    date: Date<Utc>,

    #[getset(get_copy = "pub")]
    id: Ulid,

    #[getset(get_copy = "pub")]
    has_cleared: bool,

    #[getset(get = "pub")]
    description: String,

    #[getset(get = "pub")]
    entries: Vec<Entry>,
}

impl Transaction {
    pub fn new(
        date: Date<Utc>,
        id: Ulid,
        has_cleared: bool,
        description: String,
        entries: Vec<Entry>,
    ) -> Self {
        Self {
            date,
            id,
            has_cleared,
            description,
            entries,
        }
    }
}

impl PartialOrd for Transaction {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.date().partial_cmp(&other.date())
    }
}

impl TryFrom<&TransactionOpts> for Transaction {
    type Error = Error;

    fn try_from(opts: &TransactionOpts) -> Result<Self, Self::Error> {
        match opts.entries().is_empty() {
            true => Err(Error::missing_transaction_entries()),
            false => {


                let entries = validate_and_normalize_entries(
                    opts.entries(),
                )?;

                Ok(Self::new(
                    opts.date(),
                    Ulid::new(),
                    !opts.has_not_cleared(),
                    opts.description()
                        .as_ref()
                        .map(|desc| desc.to_owned())
                        .unwrap_or_default(),
                    entries,
                ))
            }
        }
    }
}

impl TryFrom<String> for Transaction {
    type Error = Error;

    fn try_from(serialized_entry: String) -> Result<Self, Self::Error> {
        let mut date: Date<Utc> = Utc.timestamp_nanos(0).date();
        let mut id = Ulid::new();
        let mut has_cleared = true;
        let mut description = String::new();
        let mut entries = Vec::<Entry>::with_capacity(2);

        for line in serialized_entry.split("\n") {
            if line.starts_with('\t') {
                entries.push(Entry::try_from(line)?);
            } else {
                let values = line.split(" ").collect::<Vec<&str>>();
                if values.len() < 2 {
                    if values.is_empty() {
                        return Err(Error::missing_transaction_field("date"));
                    } else {
                        return Err(Error::missing_transaction_field("id"));
                    }
                }

                date = Utc
                    .from_local_date(&NaiveDate::parse_from_str(values[0], "%Y-%m-%d")?)
                    .unwrap();

                id = Ulid::from_string(values[1])?;

                for value in values.iter().skip(2) {
                    if *value == "*" {
                        has_cleared = false;
                    } else {
                        description = (*value).to_owned();
                    }
                }
            }
        }

        Ok(Self {
            id,
            date,
            has_cleared,
            description,
            entries,
        })
    }
}

impl Display for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let date = self.date.format("%Y-%m-%d");

        let has_cleared = if !self.has_cleared {
            format!(" *")
        } else {
            String::new()
        };

        let description = format!(" {}", self.description);

        let entries = self
            .entries
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>()
            .join("\n");

        write!(
            f,
            "{} {}{}{}\n{}",
            date, self.id, has_cleared, description, entries
        )
    }
}

#[derive(Clone, CopyGetters, Debug, Default, Eq, Getters, PartialEq, Ord)]
pub struct Entry {
    #[getset(get = "pub")]
    account: String,

    #[getset(get_copy = "pub")]
    value: isize,
}

impl Entry {
    pub fn new(account: &str, value: isize) -> Self {
        Self {
            account: account.to_owned(),
            value,
        }
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.account.partial_cmp(other.account())
    }
}

impl From<&(String, Option<isize>)> for Entry {
    fn from((account, value): &(String, Option<isize>)) -> Self {
        let value = match value {
            Some(value) => *value,
            None => 0,
        };

        Self {
            account: account.to_owned(),
            value,
        }
    }
}

impl TryFrom<&str> for Entry {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parts = value.trim().split(" ").collect::<Vec<&str>>();
        let value = parts
            .last()
            .unwrap()
            .parse::<isize>()
            .map_err(|_| Error::blank_entry_value())?;

        let account = parts
            .iter()
            .enumerate()
            .filter_map(|(idx, part)| {
                if idx == parts.len() - 1 {
                    None
                } else {
                    Some(*part)
                }
            })
            .collect::<Vec<&str>>()
            .join(" ");

        Ok(Self { account, value })
    }
}

impl Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\t{} {}", self.account, self.value)
    }
}

fn validate_and_normalize_entries(entries: &[(String, Option<isize>)]) -> Result<Vec<Entry>, Error> {
	let entries = {
		let number_of_entries_with_value = entries.iter().filter(|(_, value)| value.is_some()).count();
		if number_of_entries_with_value == entries.len() && entries.len() > 1 {
			entries.to_owned()
		} else {
			entries.to_owned().into_iter().chain(vec![("Equities:Starting Balances".to_owned(), None)]).collect()
		}
	};

    let values = entries.iter()
		.map(|(_, value)| *value)
		.collect::<Vec<Option<isize>>>();

	let has_multiple_blank_values = values.iter().filter(|value| value.is_none()).count() > 1;
	if has_multiple_blank_values {
		return Err(Error::unbalanced_transaction_entries());
	}


    let non_zero_values_sum = values.iter().filter_map(|value| Some(value.unwrap_or(0))).sum();
	let mut entries = match non_zero_values_sum {
        0 => entries.iter().map(|(account, value)| Entry::new(account.as_str(), value.unwrap())).collect(),
        _ => entries
            .iter()
            .map(|(account, value)| {
				let value = value.unwrap_or(0);
				Entry::new(
					account.as_str(),
					if value == 0 {
						non_zero_values_sum.neg()
					} else {
						value
					}
				)
            })
            .collect::<Vec<Entry>>(),
    };

	entries.sort();
    Ok(entries)
}

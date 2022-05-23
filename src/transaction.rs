use chrono::{DateTime, TimeZone, Utc};
use getset::{CopyGetters, Getters};

use crate::{error::Error, TIMESTAMP_CHECK_RE};

#[derive(CopyGetters, Debug, Getters)]
pub struct Transaction {
    #[getset(get_copy = "pub")]
    id: usize,

    #[getset(get_copy = "pub")]
    date: DateTime<Utc>,

    #[getset(get_copy = "pub")]
    has_cleared: bool,

    #[getset(get = "pub")]
    description: String,

    #[getset(get = "pub")]
    entries: Vec<Entry>,
}

impl TryFrom<String> for Transaction {
    type Error = Error;

    fn try_from(serialized_entry: String) -> Result<Self, Self::Error> {
        let mut id = 0_usize;
        let mut date: DateTime<Utc> = Utc.timestamp_nanos(0);
        let mut has_cleared = true;
        let mut description = String::new();
        let mut entries = Vec::<Entry>::with_capacity(2);

        for line in serialized_entry.split("\n") {
            if line.starts_with('\t') {
                entries.push(Entry::try_from(line)?);
            } else {
                for value in line.split(" ") {
                    if TIMESTAMP_CHECK_RE.is_match(value) {
                        date = DateTime::<Utc>::from(DateTime::parse_from_rfc3339(value)?);
                    } else if let Ok(value_id) = value.parse::<usize>() {
                        id = value_id
                    } else if value == "*" {
                        has_cleared = false
                    } else {
                        description = value.to_owned();
                    }
                }
            }
        }

        let _ = validate_entries(entries.as_slice())?;

        Ok(Self {
            id,
            date,
            has_cleared,
            description,
            entries,
        })
    }
}

#[derive(CopyGetters, Debug, Default, Getters)]
pub struct Entry {
    #[getset(get = "pub")]
    account: String,

    #[getset(get_copy = "pub")]
    value: isize,
}

impl TryFrom<&str> for Entry {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let values = value.trim().split(" ").collect::<Vec<&str>>();
        let account = values[0].to_owned();

        let value = if values.len() == 2 {
            values[1].parse::<isize>()?
        } else {
            return Err(Error::blank_entry_value());
        };

        Ok(Self { account, value })
    }
}

fn validate_entries(entries: &[Entry]) -> Result<(), Error> {
    let values = entries
        .iter()
        .map(|entry| entry.value())
        .collect::<Vec<isize>>();

    let has_multiple_blank_values = values.iter().filter(|&&value| value == 0).count() > 1;
    let has_blank_entry = values.iter().any(|&value| value == 0);
    let total_value = values.iter().sum::<isize>();
    if has_multiple_blank_values {
        Err(Error::unbalanced_transaction_entries())
    } else if has_blank_entry {
        Err(Error::blank_entry_value())
    } else if total_value != 0 {
        Err(Error::unbalanced_transaction_entries())
    } else {
        Ok(())
    }
}

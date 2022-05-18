use chrono::{DateTime, Utc};

use crate::error::Error;

#[derive(Debug)]
pub struct Transaction {
	id: usize,
	date: DateTime<Utc>,
	has_cleared: bool,
	entries: Vec<Entry>
}

impl Default for Transaction {
    fn default() -> Self {
        Self { id: Default::default(), date: Utc::now(), has_cleared: Default::default(), entries: Default::default() }
    }
}

impl TryFrom<String> for Transaction {
    type Error = Error;

    fn try_from(other: String) -> Result<Self, Self::Error> {
        Ok(Default::default())
    }
}

#[derive(Debug, Default)]
struct Entry {
	account: String,
	value: isize
}

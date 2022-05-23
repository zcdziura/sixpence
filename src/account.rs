use getset::{CopyGetters, Getters};

use crate::{transaction::Entry, ACCOUNTING};

#[derive(CopyGetters, Debug, Default, Eq, Getters, PartialEq, Ord)]
pub struct Account {
    #[getset(get = "pub")]
    name: String,

    #[getset(get_copy = "pub")]
    value: isize,
}

impl Account {
    pub fn format_value_as_currency(&self) -> String {
        ACCOUNTING.format_money(self.value)
    }
}

impl PartialOrd for Account {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.name().partial_cmp(other.name())
    }
}

impl From<&[&Entry]> for Account {
    fn from(entries: &[&Entry]) -> Self {
        entries.iter().fold(Account::default(), |acc, cur| {
            let name = if acc.name().is_empty() {
                cur.account().to_owned()
            } else {
                acc.name().to_owned()
            };

            let value = acc.value() + cur.value();

            Self { name, value }
        })
    }
}

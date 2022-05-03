use std::{error, fmt};

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
}

impl Error {
    pub fn account_type() -> Self {
        let kind = ErrorKind::AccountType;
        Self { kind }
    }

    pub fn accounts_without_value(accounts: Vec<String>) -> Self {
        let kind = ErrorKind::AccountsWithoutValue(accounts);
        Self { kind }
    }

    pub fn data_file(message: String, error: std::io::Error) -> Self {
        let kind = ErrorKind::DataFile(message, error);
        Self { kind }
    }

    pub fn recurring_period(period: &str) -> Self {
        let kind = ErrorKind::RecurringPeriod(period.to_string());
        Self { kind }
    }

    pub fn unbalanced_transaction(debits: isize, credits: isize) -> Self {
        let kind = ErrorKind::UnbalancedTransaction(debits, credits);
        Self { kind }
    }

    pub fn unknown_account(account: &str) -> Self {
        let kind = ErrorKind::UnknownAccount(account.to_string());
        Self { kind }
    }

    fn io(inner: std::io::Error) -> Self {
        let kind = ErrorKind::Io(inner);
        Self { kind }
    }

    fn bincode(inner: bincode::Error) -> Self {
        let kind = ErrorKind::BincodeError(inner);
        Self { kind }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self.kind {
            ErrorKind::Io(err) | ErrorKind::DataFile(_, err) => err.source(),
            ErrorKind::BincodeError(err) => err.source(),
            _ => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            ErrorKind::Io(err) => write!(f, "{}", err),
            ErrorKind::BincodeError(err) => write!(f, "{}", err),
            ErrorKind::AccountType => write!(f, "Invalid account type"),
            ErrorKind::DataFile(msg, err) => {
                write!(f, "{}: {}", msg, err)
            },
			ErrorKind::RecurringPeriod(period) => write!(f, "Invalid recurring period: '{}'. Possible values are: 'onetime', 'weekly', 'biweekly', 'monthly', 'annually'.", period),
            ErrorKind::UnknownAccount(account) => write!(f, "Unknown account: {}.", account),
            ErrorKind::UnbalancedTransaction(debits, credits) => write!(f, "Transaction has unbalanced values; debits: {}, credits: {}.", debits, credits),
            ErrorKind::AccountsWithoutValue(accounts) => {
                let accounts = accounts.join(", ");
                write!(f, "Transaction has accounts listed without any associated values: {}.", accounts)
            }
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::io(error)
    }
}

impl From<bincode::Error> for Error {
    fn from(error: bincode::Error) -> Self {
        Self::bincode(error)
    }
}

impl Into<i32> for Error {
    fn into(self) -> i32 {
        match self.kind {
            ErrorKind::AccountType => 1,
            ErrorKind::DataFile(_, _) => 2,
            ErrorKind::AccountsWithoutValue(_) => 3,
            ErrorKind::BincodeError(_) => 4,
            ErrorKind::Io(_) => 5,
            ErrorKind::RecurringPeriod(_) => 6,
            ErrorKind::UnbalancedTransaction(_, _) => 7,
            ErrorKind::UnknownAccount(_) => 8,
        }
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    AccountType,
    DataFile(String, std::io::Error),
    AccountsWithoutValue(Vec<String>),
    BincodeError(bincode::Error),
    Io(std::io::Error),
    RecurringPeriod(String),
    UnbalancedTransaction(isize, isize),
    UnknownAccount(String),
}

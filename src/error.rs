use std::{error, fmt};

pub type Error = Box<ErrorKind>;

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &**self {
            ErrorKind::Io(err)
            | ErrorKind::AccountsFile(_, err)
            | ErrorKind::LedgerFile(_, err) => err.source(),
            ErrorKind::BincodeError(err) => err.source(),
            _ => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &**self {
            ErrorKind::Io(err) => write!(f, "{}", err),
            ErrorKind::BincodeError(err) => write!(f, "{}", err),
            ErrorKind::AccountType => write!(f, "Invalid account type"),
            ErrorKind::AccountsFile(msg, err) | ErrorKind::LedgerFile(msg, err) => {
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
        Box::new(ErrorKind::Io(error))
    }
}

impl From<bincode::Error> for Error {
    fn from(error: bincode::Error) -> Self {
        Box::new(ErrorKind::BincodeError(error))
    }
}

impl Into<i32> for Error {
    fn into(self) -> i32 {
        match *self {
            ErrorKind::AccountsFile(_, _) => 1,
            _ => unimplemented!()
        }
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    AccountType,
    AccountsFile(String, std::io::Error),
    BincodeError(bincode::Error),
    Io(std::io::Error),
    LedgerFile(String, std::io::Error),
    RecurringPeriod(String),
    UnknownAccount(String),
    UnbalancedTransaction(isize, isize),
    AccountsWithoutValue(Vec<String>),
}

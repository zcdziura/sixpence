use std::{error, fmt};

pub type Error = Box<ErrorKind>;

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &**self {
            ErrorKind::Io(err) | ErrorKind::AccountsFile(_, err) => err.source(),
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
			ErrorKind::RecurringPeriod(period) => write!(f, "Invalid recurring period: '{}'. Possible values are: 'onetime', 'weekly', 'biweekly', 'monthly', 'annually'.", period)
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

#[derive(Debug)]
pub enum ErrorKind {
    Io(std::io::Error),
    BincodeError(bincode::Error),
    AccountType,
    AccountsFile(String, std::io::Error),
    LedgerFile(String, std::io::Error),
    RecurringPeriod(String),
}

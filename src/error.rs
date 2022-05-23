use std::{error, fmt, num::ParseIntError, path::PathBuf};

use chrono::ParseError;
use ErrorKind::*;

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
}

impl Error {
    pub fn ledger_file_not_found(path: PathBuf) -> Self {
        Self::new(LedgerFileNotFound(path))
    }

    pub fn bincode(inner: bincode::Error) -> Self {
        Self::new(BincodeError(inner))
    }

    pub fn blank_entry_value() -> Self {
        Self::new(BlankEntryValue)
    }

    pub fn corrupted_ledger_file() -> Self {
        Self::new(CorruptedLedgerFile)
    }

    pub fn invalid_entry_date(error: ParseError) -> Self {
        Self::new(InvalidEntryDate(error))
    }

    pub fn invalid_entry_value(error: ParseIntError) -> Self {
        Self::new(InvalidEntryValue(error))
    }

    pub fn invalid_ledger_file(path: PathBuf) -> Self {
        Self::new(InvalidLedgerFile(path))
    }

    pub fn io(inner: std::io::Error) -> Self {
        Self::new(Io(inner))
    }

    pub fn unbalanced_transaction_entries() -> Self {
        Self::new(UnbalancedTransactionEntries)
    }

    fn new(kind: ErrorKind) -> Self {
        Self { kind }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self.kind {
            BincodeError(err) => err.source(),
            Io(err) => err.source(),
            _ => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            BincodeError(err) => write!(f, "{}", err),
            BlankEntryValue => write!(f, "Entry has a blank value."),
            CorruptedLedgerFile => write!(f, "Ledger file contains corrupted data."),
            InvalidEntryDate(date) => write!(f, "Invalid entry date: {}.", date),
            InvalidEntryValue(value) => write!(f, "Invalid entry value: {}.", value),
            InvalidLedgerFile(dir) => write!(f, "Invalid ledger directory: {:?}.", dir),
            Io(err) => write!(f, "{}", err),
            LedgerFileNotFound(path) => write!(f, "Ledger file not found at: {:?}.", path),
            UnbalancedTransactionEntries => write!(f, "A transaction has unbalanced entries."),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::io(error)
    }
}

impl From<&std::io::Error> for Error {
    fn from(error: &std::io::Error) -> Self {
        Self::io(std::io::Error::from(error.kind()))
    }
}

impl From<bincode::Error> for Error {
    fn from(error: bincode::Error) -> Self {
        Self::bincode(error)
    }
}

impl From<ParseError> for Error {
    fn from(other: ParseError) -> Self {
        Self::invalid_entry_date(other)
    }
}

impl From<ParseIntError> for Error {
    fn from(other: ParseIntError) -> Self {
        Self::invalid_entry_value(other)
    }
}

impl Into<i32> for Error {
    fn into(self) -> i32 {
        match self.kind {
            BincodeError(_) => 1,
            BlankEntryValue => 2,
            CorruptedLedgerFile => 3,
            InvalidEntryDate(_) => 4,
            InvalidEntryValue(_) => 5,
            InvalidLedgerFile(_) => 6,
            Io(_) => 7,
            LedgerFileNotFound(_) => 8,
            UnbalancedTransactionEntries => 9,
        }
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    BincodeError(bincode::Error),
    BlankEntryValue,
    CorruptedLedgerFile,
    InvalidEntryDate(ParseError),
    InvalidEntryValue(ParseIntError),
    InvalidLedgerFile(PathBuf),
    Io(std::io::Error),
    LedgerFileNotFound(PathBuf),
    UnbalancedTransactionEntries,
}

use std::{error, fmt, path::Path};

use ErrorKind::*;

#[derive(Debug)]
pub struct Error<'p> {
    kind: ErrorKind<'p>,
}

impl<'p> Error<'p> {
    pub fn ledger_file_not_found(path: &'p Path) -> Self {
        Self::new(LedgerFileNotFound(path))
    }

    pub fn bincode(inner: bincode::Error) -> Self {
        Self::new(BincodeError(inner))
    }

    pub fn corrupted_ledger_file() -> Self {
        Self::new(CorruptedLedgerFile)
    }

    pub fn invalid_ledger_file(path: &'p Path) -> Self {
        Self::new(InvalidLedgerFile(path))
    }

    pub fn io(inner: std::io::Error) -> Self {
        Self::new(Io(inner))
    }

    fn new(kind: ErrorKind<'p>) -> Self {
        Self { kind }
    }
}

impl<'p> error::Error for Error<'p> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self.kind {
            BincodeError(err) => err.source(),
            Io(err) => err.source(),
            _ => None,
        }
    }
}

impl<'p> fmt::Display for Error<'p> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            BincodeError(err) => write!(f, "{}", err),
            CorruptedLedgerFile => write!(f, "Ledger file contains corrupted data."),
            InvalidLedgerFile(dir) => write!(f, "Invalid ledger directory: {:?}", dir),
            Io(err) => write!(f, "{}", err),
            LedgerFileNotFound(path) => write!(f, "Ledger file not found at: {:?}.", path),
        }
    }
}

impl<'p> From<std::io::Error> for Error<'p> {
    fn from(error: std::io::Error) -> Self {
        Self::io(error)
    }
}

impl<'p> From<&std::io::Error> for Error<'p> {
    fn from(error: &std::io::Error) -> Self {
        Self::io(std::io::Error::from(error.kind()))
    }
}

impl<'p> From<bincode::Error> for Error<'p> {
    fn from(error: bincode::Error) -> Self {
        Self::bincode(error)
    }
}

impl<'p> Into<i32> for Error<'p> {
    fn into(self) -> i32 {
        match self.kind {
            BincodeError(_) => 1,
            CorruptedLedgerFile => 2,
            InvalidLedgerFile(_) => 3,
            Io(_) => 4,
            LedgerFileNotFound(_) => 5,
        }
    }
}

#[derive(Debug)]
pub enum ErrorKind<'p> {
    BincodeError(bincode::Error),
    CorruptedLedgerFile,
    InvalidLedgerFile(&'p Path),
    Io(std::io::Error),
    LedgerFileNotFound(&'p Path),
}

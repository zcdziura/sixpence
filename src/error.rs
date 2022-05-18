use std::{error, fmt, path::PathBuf};

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

    pub fn corrupted_ledger_file() -> Self {
        Self::new(CorruptedLedgerFile)
    }

    pub fn invalid_ledger_file(path: PathBuf) -> Self {
        Self::new(InvalidLedgerFile(path))
    }

    pub fn io(inner: std::io::Error) -> Self {
        Self::new(Io(inner))
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
            CorruptedLedgerFile => write!(f, "Ledger file contains corrupted data."),
            InvalidLedgerFile(dir) => write!(f, "Invalid ledger directory: {:?}", dir),
            Io(err) => write!(f, "{}", err),
            LedgerFileNotFound(path) => write!(f, "Ledger file not found at: {:?}.", path),
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

impl Into<i32> for Error {
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
pub enum ErrorKind {
    BincodeError(bincode::Error),
    CorruptedLedgerFile,
    InvalidLedgerFile(PathBuf),
    Io(std::io::Error),
    LedgerFileNotFound(PathBuf),
}

use std::{
    fs::File,
    io::{self, Read},
    path::PathBuf,
};

use crate::error::Error;

pub struct Ledger {}

impl Ledger {
    // pub fn parse_ledger_file(file_path: PathBuf) -> Result<Self, Error> {
    //     let mut _is_empty_line = false;
    //     let mut _error: Option<io::Error> = None;
    //     let mut buffer = Vec::<u8>::with_capacity(64);

    //     for byte in file_path.bytes() {
    //         match byte {
    //             Ok(byte) => {
    //                 if byte == ('\n' as u8)
    //                     && (buffer[buffer.len() - 1] == ('\n' as u8)
    //                         || &buffer[buffer.len() - 2..] == "\r\n".as_bytes())
    //                 {
    //                     match String::from_utf8(buffer.clone()) {
    //                         Ok(transaction) => {
    //                             let parts = transaction
    //                                 .split(|ch| ch == '\n' || ch == '\r')
    //                                 .filter(|part| !part.is_empty())
    //                                 .collect::<Vec<&str>>();

    //                             println!("{:?}", parts.as_slice());
    //                             // TODO: Map each "part" to a piece of the transaction, either description or entry
    //                         }
    //                         Err(_) => return Err(Error::corrupted_ledger_file()),
    //                     }
    //                 } else {
    //                     buffer.push(byte);
    //                 }
    //             }
    //             Err(error) => {
    //                 return Err(Error::from(error));
    //             }
    //         }
    //     }

    //     Ok(Ledger {})
    // }
}

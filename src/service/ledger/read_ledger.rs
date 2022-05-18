use std::{path::{Path, PathBuf}, fs::{File}, io::Read};

use crate::{error::Error};

pub fn read_ledger<'p>(ledger_file_path: &'p Path) -> Result<(), Error> {
	if !ledger_file_path.exists() {
		return Err(Error::ledger_file_not_found(PathBuf::from(ledger_file_path)));
	}

	let mut buffer = Vec::<u8>::with_capacity(64);
	let mut is_end_of_line = false;
	let ledger_file = File::open(ledger_file_path)?;
	for byte_result in ledger_file.bytes() {
		match byte_result {
			Ok(byte) => {
				if byte != ('\n' as u8) {
					is_end_of_line = false;
					buffer.push(byte);
				} else {
					if !is_end_of_line {
						is_end_of_line = true;
						buffer.push(byte);
					} else {
						match String::from_utf8(buffer.clone()) {
							Ok(transaction) => {
								buffer.clear();
								
							},
							Err(_) => return Err(Error::corrupted_ledger_file())
						}
					}
				}
			},
			Err(error) => return Err(Error::from(error))
		}
	}

	Ok(())
}

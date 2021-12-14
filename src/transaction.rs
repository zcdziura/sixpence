mod validate;

use chrono::prelude::*;
use serde::{Deserialize, Serialize};

pub use validate::validate_new_transaction_opts;

#[derive(Debug, Deserialize, Serialize)]
pub struct Transaction {
    timestamp: DateTime<Utc>,
    debits: Vec<(String, usize)>,
    credits: Vec<(String, usize)>,
}

// pub fn create_new_transaction(
// 	path: PathBuf,
// 	debits: Vec<(String, usize)>,
// 	credits: Vec<(String, usize)>,
// ) {

// }

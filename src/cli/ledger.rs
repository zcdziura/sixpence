use std::path::PathBuf;

use clap::{Args, Subcommand};

#[derive(Debug, Subcommand)]

pub enum LedgerCommands {
    /// Generate a new, empty ledger
    New,
}

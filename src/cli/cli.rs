use clap::{Parser, Subcommand};

use super::{args::GlobalArgs, ledger::LedgerCommands};

#[derive(Debug, Parser)]
#[clap(about, author, version)]
pub struct Cli {
    #[clap(flatten)]
    global_args: GlobalArgs,

    #[clap(subcommand)]
    commands: Commands,
}

impl Cli {
    pub fn global_args(&self) -> &GlobalArgs {
        &self.global_args
    }

    pub fn commands(&self) -> &Commands {
        &self.commands
    }
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Commands to process the ledger
    #[clap(subcommand)]
    Ledger(LedgerCommands),
}

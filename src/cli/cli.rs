use clap::{Parser, Subcommand};

use super::{args::GlobalArgs};

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
	/// Create a new, empty ledger
	New,
}

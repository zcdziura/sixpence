use clap::Parser;

use super::args::GlobalArgs;

#[derive(Debug, Parser)]
#[clap(about, author, version)]
pub struct Cli {
    #[clap(flatten)]
    global_args: GlobalArgs,
}

impl Cli {
    pub fn global_args(&self) -> &GlobalArgs {
        &self.global_args
    }
}

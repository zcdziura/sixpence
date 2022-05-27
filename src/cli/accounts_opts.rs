use clap::Args;
use getset::CopyGetters;

#[derive(Args, CopyGetters, Debug)]
pub struct AccountsOpts {
    /// Display all accounts
    #[getset(get_copy = "pub")]
    #[clap(short = 'a', long = "all")]
    display_all: bool,
}

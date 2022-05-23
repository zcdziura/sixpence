use clap::Args;

#[derive(Args, Debug)]
pub struct Transaction {
    account_names: Vec<String>,
}

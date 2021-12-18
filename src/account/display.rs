use std::iter;

use pad::PadStr;

use super::Account;

pub fn display_accounts(accounts: &[Account]) {
    let (name_col_width, type_col_width) = get_column_widths(accounts);
    print_headers(name_col_width, type_col_width);
    print_accounts(accounts, name_col_width, type_col_width);
}

fn get_column_widths(accounts: &[Account]) -> (usize, usize) {
    accounts
        .iter()
        .fold(("name".len(), "type".len()), |acc, cur| {
            let (acc_name_len, acc_type_len) = acc;
            let cur_name_len = cur.name().len();
            let cur_type_len = cur.account_type().len();
            (
                if cur_name_len > acc_name_len {
                    cur_name_len
                } else {
                    acc_name_len
                },
                if cur_type_len > acc_type_len {
                    cur_type_len
                } else {
                    acc_type_len
                },
            )
        })
}

fn print_headers(name_col_width: usize, type_col_width: usize) {
    let header = format!(
        "{}  {}",
        "Name".pad_to_width(name_col_width),
        "Type".pad_to_width(type_col_width)
    );
    let separator = format!(
        "{}  {}",
        iter::repeat('-').take(name_col_width).collect::<String>(),
        iter::repeat('-').take(type_col_width).collect::<String>()
    );

    println!("{}\n{}", header, separator);
}

fn print_accounts(accounts: &[Account], name_col_width: usize, type_col_width: usize) {
    accounts.iter().for_each(|account| {
        println!(
            "{}  {}",
            account.name().pad_to_width(name_col_width),
            account
                .account_type()
                .to_string()
                .pad_to_width(type_col_width)
        );
    })
}

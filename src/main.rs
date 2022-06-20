mod bank;

use std::env::args;

const PATH: &str = "prod_data";

use crate::bank::Bank;
fn main() {
    let mut bank_instance = Bank::new(String::from(PATH));

    match std::env::args().nth(1) {
        Some(i) => match i.as_str() {
            "create-account" => bank_instance.create_account(
                &args().nth(2).unwrap(),
                args().nth(3).unwrap().parse().unwrap(),
            ),
            "transfer" => todo!(),
            "balance" => bank_instance.view_balance(&args().nth(2).unwrap()),
            &_ => todo!(),
        },
        None => (),
    };
}

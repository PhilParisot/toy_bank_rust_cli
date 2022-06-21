mod bank;

use std::env::{args};

const PATH: &str = "prod_data";

use crate::bank::Bank;
fn main() {
    let mut bank_instance = Bank::new(String::from(PATH));

    match args().len() {
        i if i > 5 => {
            println!("Too many arguments");
            return;
        }
        i if i < 3 => {
            println!("Too few arguments");
            return;
        }
        _ => (),
    }

    match args().nth(1) {
        Some(i) => match i.as_str() {
            "create-account" => bank_instance.create_account(
                &args().nth(2).unwrap(),
                args().nth(3).unwrap().parse().unwrap(),
            ),
            "transfer" => {
                bank_instance
                    .transfer(
                        &args().nth(2).unwrap(),
                        &args().nth(3).unwrap(),
                        args().nth(4).unwrap().parse().unwrap(),
                    )
                    .unwrap();
            }

            "balance" => bank_instance.view_balance(&args().nth(2).unwrap()),
            &_ => {
                println!("No valid entry");
            }
        },
        None => (),
    };
}

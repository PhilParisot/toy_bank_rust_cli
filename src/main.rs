mod bank;

use std::{env::args, fs};

const PATH: &str = "data/prod_data";

use crate::bank::Bank;
fn main() {
    let mut bank_instance = Bank::new(String::from(PATH));

    if args().len() < 2 {
        invalid_entry()
    }

    match args().nth(1) {
        Some(i) => match i.as_str() {
            "-h" | "--help" => {
                println!(
                    "{}",
                    fs::read_to_string("src/help.txt").expect("Could not read help.txt")
                )
            }
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
            &_ => invalid_entry(),
        },
        None => (),
    };
}

fn invalid_entry() {
    println!("Invalid entry\n");
    println!(
        "{}",
        fs::read_to_string("src/help.txt").expect("Could not read help.txt")
    )
}

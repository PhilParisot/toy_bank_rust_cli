mod bank;
use crate::bank::Bank;
fn main() {
    let mut bank_instance = Bank::new(String::from("data"));

    match std::env::args().nth(1) {
        Some(i) => match i.as_str() {
            "create-account" => bank_instance.create_account(
                std::env::args().nth(2).unwrap(),
                std::env::args().nth(3).unwrap().parse().unwrap()
            ),
            "transfer" => todo!(),
            "balance" => todo!(),
            &_ => todo!(),
        },
        None => (),
    };
}

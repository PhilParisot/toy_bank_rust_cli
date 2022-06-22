use assert_cmd::prelude::CommandCargoExt;
use assert_cmd::prelude::*;
use std::fs;
use std::process::Command;

const BINARY: &str = "toy_bank_rust_cli";
const CREATE_ACCOUNT: &str = "create-account";
const PATH: &str = "data/prod_data";

#[test]
fn create_same_account() {
    match fs::remove_file(PATH) {
        Ok(_) => println!("prod_data removed successfully"),
        Err(_) => println!("No prod_data to remove"),
    }

    let mut cmd = Command::cargo_bin(BINARY).unwrap();
    cmd.arg(CREATE_ACCOUNT).arg("Rizza").arg("1000");
    cmd.assert().success();
    let mut cmd2 = Command::cargo_bin(BINARY).unwrap();
    cmd2.arg(CREATE_ACCOUNT).arg("Rizza").arg("5");
    cmd2.assert().failure();
}

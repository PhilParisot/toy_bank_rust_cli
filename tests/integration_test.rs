use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

use assert_cmd::prelude::CommandCargoExt;

const BINARY: &str = "toy_bank_rust_cli";
const CREATE_ACCOUNT: &str = "create-account";
const TRANSFER: &str = "transfer";
const BALANCE: &str = "balance";

#[test]
fn view_balance() {}

#[test]
fn create_same_account() {
    let mut cmd = Command::cargo_bin(BINARY).unwrap();
    cmd.arg(CREATE_ACCOUNT).arg("Rizza").arg("1000");
    cmd.assert().success();
    let mut cmd2 = Command::cargo_bin(BINARY).unwrap();
    cmd2.arg(CREATE_ACCOUNT).arg("Rizza").arg("1000");
    cmd2.assert().failure();
}

fn view_balance_from_old_data() {
    todo!();
}

fn transfer() {
    todo!();
}

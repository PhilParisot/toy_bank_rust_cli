# Toy Bank Rust CLI

`print_test_script.sh` is a small `bash` script you can run out the box with if you have `cargo` installed in your path to print normal output.

Running `cargo test` will run all unit and integration tests.

Data is stored and extracted to and from `json` files.

```
USAGE:
    cargo run -q -- [OPTIONS] [args]

ARGS:
    <args>...

OPTIONS:
    create-account <name-of-account> <starting-balance>     Creates accounts with balance and stores data locally
    transfer <from-account> <to-account> <amount>           Transfers funds from one account to another and stores data locally
    balance <account>                                       Prints available funds in an account
```
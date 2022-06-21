#! /bin/bash
set -x

cargo run -- create-account Phil 5
cargo run -- create-account Rizza 1000
cargo run -- balance Phil
cargo run -- balance Rizza
cargo run -- transfer Phil Rizza 5
cargo run -- balance Phil
cargo run -- balance Rizza

sudo rm -f prod_data
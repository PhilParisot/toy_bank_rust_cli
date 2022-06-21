#! /bin/bash
set -x

cargo run -q -- create-account Phil 5
cargo run -q -- create-account Rizza 1000
cargo run -q -- balance Phil
cargo run -q -- balance Rizza
cargo run -q -- transfer Phil Rizza 5
cargo run -q -- balance Phil
cargo run -q -- balance Rizza

sudo rm -f prod_data
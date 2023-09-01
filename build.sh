#!/bin/bash

# Build the project
echo "Building the project..."
cd libpal
cargo build --release
cd ..

cd set1
cargo build --release
cargo run --bin exercise1
cargo run --bin exercise2
cargo run --bin exercise3
cargo run --bin exercise4
cargo run --bin exercise5

cargo run --bin exercise7
cargo run --bin exercise8

echo "Done!"


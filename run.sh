#!/bin/sh

# Build
cargo build --profile=release

# Generate inputs and Compile
./benchmarks/gen-input.sh
./target/release/thesis compile

# Run benchmarks
./target/release/thesis run -c5 -r11

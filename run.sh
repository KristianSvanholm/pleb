#!/bin/sh

# Build
cargo build --profile=release

# Generate inputs and Compile
cd benchmarks && sh gen-input.sh && cs ..
./target/release/thesis compile

# Run benchmarks
./target/release/thesis run -c5 -r11

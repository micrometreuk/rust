#!/bin/bash

# Compile the project (optional, if you want to force a recompile)
cargo build

# Run the compiled binary with additional arguments
./target/debug/my_project --arg1 value1 --arg2 value2

# Perform other tasks, like running tests, linting, or deploying
cargo test
cargo fmt
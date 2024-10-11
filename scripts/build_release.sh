#!/bin/bash

# Update the dependencies
cargo update

# Run the project
cargo lcheck --release --bin rem-controller
cargo build --release --bin rem-controller
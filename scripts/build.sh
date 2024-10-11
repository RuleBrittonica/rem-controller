#!/bin/bash

# Update the dependencies
cargo update

# Run the project
cargo lcheck --bin rem-controller
cargo build --bin rem-controller
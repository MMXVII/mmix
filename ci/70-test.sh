#!/bin/bash

# Exit script on first error
set -o errexit -o nounset

# Explain what we do
echo ">>> Running the tests..."

# Build the project
export RUSTFLAGS="--deny warnings"
cargo test

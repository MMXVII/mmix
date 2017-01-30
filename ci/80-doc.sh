#!/bin/bash

# Exit script on first error
set -o errexit -o nounset

# Explain what we do
echo ">>> Checking the documentation..."

# Build the project
export RUSTFLAGS="--deny warnings"
cargo doc

#!/bin/bash

# Exit script on first error
set -o errexit -o nounset

# Compute the current directory
DIR="`dirname \"$0\"`"

# Run each test
for TEST in $(find $DIR/*-* -type f); do
    $TEST
done

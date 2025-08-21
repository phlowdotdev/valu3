#!/bin/bash

# Clean expanded files before running tests
echo "Cleaning expanded test files..."
rm -f valu3_derive/tests/*.expanded.rs

# Run tests
echo "Running tests..."
cargo test --all "$@"

# Clean expanded files after tests
echo "Cleaning up..."
rm -f valu3_derive/tests/*.expanded.rs

echo "Done!"

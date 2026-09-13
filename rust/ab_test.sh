#!/bin/bash
set -e

echo "=== A/B Test for p853 ==="
echo

# Test version A (master)
echo "Building version A (master)..."
git checkout master -- rust/solutions/src/bin/p853.rs
cargo build --release --bin p853 2>&1 | grep -E "(Compiling|Finished)" | tail -1

echo "Running version A - 3 iterations..."
for i in 1 2 3; do
    result=$( { time ./target/release/p853; } 2>&1 )
    answer=$(echo "$result" | grep -E "^[0-9]+$")
    timing=$(echo "$result" | grep real | awk '{print $2}')
    echo "  Run $i: $timing (answer: $answer)"
done
echo

# Test version B (optimized)
echo "Building version B (optimized)..."
git checkout cursor/optimize-p853-4d39 -- rust/solutions/src/bin/p853.rs
cargo build --release --bin p853 2>&1 | grep -E "(Compiling|Finished)" | tail -1

echo "Running version B - 3 iterations..."
for i in 1 2 3; do
    result=$( { time ./target/release/p853; } 2>&1 )
    answer=$(echo "$result" | grep -E "^[0-9]+$")
    timing=$(echo "$result" | grep real | awk '{print $2}')
    echo "  Run $i: $timing (answer: $answer)"
done

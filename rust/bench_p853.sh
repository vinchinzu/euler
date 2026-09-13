#!/bin/bash
set -e

cd /workspace/rust

# Compile if needed
cargo build --release --bin p853 &> /dev/null

# Run many times and compute statistics
sum=0
count=100

for i in $(seq 1 $count); do
    elapsed=$( { time ./target/release/p853 > /dev/null; } 2>&1 | grep real | awk '{print $2}' | sed 's/0m//; s/s//')
    sum=$(echo "$sum + $elapsed" | bc)
done

avg=$(echo "scale=6; $sum / $count" | bc)
echo "Average time over $count runs: ${avg}s"

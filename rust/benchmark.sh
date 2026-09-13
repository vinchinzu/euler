#!/bin/bash
cd /workspace/rust

echo "Building original..."
git stash -q
cargo build --release --bin p897 2>/dev/null
echo "Timing original (20 runs):"
for i in {1..20}; do
    /usr/bin/time -f "%e" ./target/release/p897 2>&1 | grep -E "^[0-9]" | head -1
done | awk '{sum+=$1; count++} END {printf "Average: %.3f ms\n", sum/count*1000}'

echo ""
echo "Building optimized..."
git stash pop -q
cargo build --release --bin p897 2>/dev/null
echo "Timing optimized (20 runs):"
for i in {1..20}; do
    /usr/bin/time -f "%e" ./target/release/p897 2>&1 | grep -E "^[0-9]" | head -1
done | awk '{sum+=$1; count++} END {printf "Average: %.3f ms\n", sum/count*1000}'

#!/bin/bash
# Benchmark C vs Rust solutions
# Usage: ./bench.sh [problem_number]
# Or:    ./bench.sh all  (benchmark all available pairs)

EULER_ROOT="$(dirname "$0")/.."
C_DIR="$EULER_ROOT/c"
RUST_DIR="$(dirname "$0")"
RESULTS_FILE="$RUST_DIR/bench_results.csv"

# Initialize CSV header
echo "problem,c_time_ms,rust_time_ms,speedup,c_answer,rust_answer,match" > "$RESULTS_FILE"

bench_one() {
    local p=$1
    local c_src="$C_DIR/${p}.c"
    local pp=$(printf '%03d' "$p")
    local rust_bin_name="p${pp}"

    # Check C source exists
    if [ ! -f "$c_src" ]; then
        echo "SKIP P$p: no C source"
        return
    fi

    # Compile C
    local c_bin="/tmp/euler_c_${p}"
    gcc -O2 -lm "$c_src" -o "$c_bin" 2>/dev/null
    if [ $? -ne 0 ]; then
        echo "SKIP P$p: C compile failed"
        return
    fi

    # Build Rust (should already be built in release mode)
    local rust_bin="$RUST_DIR/target/release/$rust_bin_name"
    if [ ! -f "$rust_bin" ]; then
        echo "SKIP P$p: no Rust binary"
        return
    fi

    echo -n "P$p: "

    # Time C (average of 1 run for long, 3 for short)
    local c_start=$(date +%s%N)
    local c_answer=$(timeout 300 "$c_bin" 2>/dev/null)
    local c_end=$(date +%s%N)
    local c_ms=$(( (c_end - c_start) / 1000000 ))

    # Time Rust
    local r_start=$(date +%s%N)
    local r_answer=$(timeout 300 "$rust_bin" 2>/dev/null)
    local r_end=$(date +%s%N)
    local r_ms=$(( (r_end - r_start) / 1000000 ))

    # Calculate speedup (C/Rust ratio; >1 means Rust is faster)
    local speedup="N/A"
    if [ "$r_ms" -gt 0 ] && [ "$c_ms" -gt 0 ]; then
        speedup=$(awk "BEGIN {printf \"%.2f\", $c_ms / $r_ms}")
    elif [ "$r_ms" -eq 0 ] && [ "$c_ms" -eq 0 ]; then
        speedup="1.00"
    fi

    # Check answer match
    local match="YES"
    if [ "$c_answer" != "$r_answer" ]; then
        match="NO"
    fi

    printf "C=%5dms  Rust=%5dms  Speedup=%5sx  Match=%s\n" "$c_ms" "$r_ms" "$speedup" "$match"

    # Append to CSV
    echo "$p,$c_ms,$r_ms,$speedup,$c_answer,$r_answer,$match" >> "$RESULTS_FILE"
}

if [ "$1" = "all" ]; then
    # Find all Rust solution binaries (exclude .d debug files)
    cargo build --release 2>/dev/null
    for rs in "$RUST_DIR"/solutions/src/bin/p*.rs; do
        if [ -f "$rs" ]; then
            fname=$(basename "$rs" .rs)   # e.g. "p131"
            p=$(echo "$fname" | sed 's/^p0*//')  # strip leading zeros
            bench_one "$p"
        fi
    done
elif [ -n "$1" ]; then
    bench_one "$1"
else
    echo "Usage: $0 <problem_number|all>"
fi

# Print summary
echo ""
echo "=== SUMMARY ==="
echo "Results saved to: $RESULTS_FILE"
awk -F',' 'NR>1 {
    total++;
    if($7=="YES") correct++;
    c_total+=$2; r_total+=$3;
    if($4+0 > 1.0) faster++;
    if($4+0 > 0 && $4+0 < 1.0) slower++;
} END {
    printf "Total: %d  Correct: %d  Rust faster: %d  Rust slower: %d\n", total, correct, faster, slower;
    printf "Total C time: %dms  Total Rust time: %dms  Overall speedup: %.2fx\n", c_total, r_total, c_total/r_total;
}' "$RESULTS_FILE"

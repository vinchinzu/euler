#!/usr/bin/env bash
# A/B benchmark: compare baseline (HEAD) vs candidate (working tree) for a single problem
# Usage: ./ab_bench.sh <problem_number> [warmup_runs] [timed_runs]
# Output: JSON with baseline_median_ms, candidate_median_ms, speedup, correct
set -euo pipefail

P="$1"
WARMUP="${2:-2}"
RUNS="${3:-7}"
REPO_ROOT="$(cd "$(dirname "$0")" && pwd)"
RUST_DIR="$REPO_ROOT/rust"
SRC="$RUST_DIR/solutions/src/bin/p${P}.rs"
EXPECTED=$(grep "Problem ${P}:" "$REPO_ROOT/data/answers.txt" | tr -d '\r' | sed "s/Problem ${P}: //")

if [ -z "$EXPECTED" ]; then
    echo "{\"problem\":$P,\"error\":\"no expected answer found\"}"
    exit 1
fi

# Build and benchmark baseline (HEAD version)
BASELINE_SRC=$(git show "HEAD:rust/solutions/src/bin/p${P}.rs" 2>/dev/null) || {
    echo "{\"problem\":$P,\"error\":\"no HEAD version found\"}"
    exit 1
}

# Save candidate, restore baseline, build, benchmark, then restore candidate
cp "$SRC" "/tmp/p${P}_candidate.rs"

echo "$BASELINE_SRC" > "$SRC"
cd "$RUST_DIR"
if ! cargo build --release --bin "p${P}" 2>/dev/null; then
    # Restore candidate
    cp "/tmp/p${P}_candidate.rs" "$SRC"
    echo "{\"problem\":$P,\"error\":\"baseline build failed\"}"
    exit 1
fi
BASELINE_BIN="$RUST_DIR/target/release/p${P}"

# Check baseline correctness
BASELINE_OUT=$(timeout 120 "$BASELINE_BIN" 2>/dev/null) || BASELINE_OUT="TIMEOUT"
if [ "$BASELINE_OUT" != "$EXPECTED" ]; then
    cp "/tmp/p${P}_candidate.rs" "$SRC"
    echo "{\"problem\":$P,\"error\":\"baseline incorrect: got '$BASELINE_OUT' expected '$EXPECTED'\"}"
    exit 1
fi

# Warmup baseline
for ((i=0; i<WARMUP; i++)); do
    timeout 120 "$BASELINE_BIN" >/dev/null 2>/dev/null || true
done

# Timed baseline runs
BASELINE_TIMES=()
for ((i=0; i<RUNS; i++)); do
    T=$( { TIMEFORMAT='%R'; time timeout 120 "$BASELINE_BIN" >/dev/null 2>/dev/null ; } 2>&1 )
    BASELINE_TIMES+=("$T")
done

# Restore candidate and build
cp "/tmp/p${P}_candidate.rs" "$SRC"
cd "$RUST_DIR"
if ! cargo build --release --bin "p${P}" 2>/dev/null; then
    echo "{\"problem\":$P,\"error\":\"candidate build failed\"}"
    exit 1
fi
CANDIDATE_BIN="$RUST_DIR/target/release/p${P}"

# Check candidate correctness
CANDIDATE_OUT=$(timeout 120 "$CANDIDATE_BIN" 2>/dev/null) || CANDIDATE_OUT="TIMEOUT"
CORRECT="true"
if [ "$CANDIDATE_OUT" != "$EXPECTED" ]; then
    CORRECT="false"
fi

# Warmup candidate
for ((i=0; i<WARMUP; i++)); do
    timeout 120 "$CANDIDATE_BIN" >/dev/null 2>/dev/null || true
done

# Timed candidate runs
CANDIDATE_TIMES=()
for ((i=0; i<RUNS; i++)); do
    T=$( { TIMEFORMAT='%R'; time timeout 120 "$CANDIDATE_BIN" >/dev/null 2>/dev/null ; } 2>&1 )
    CANDIDATE_TIMES+=("$T")
done

# Compute medians using sort
BASELINE_MEDIAN=$(printf '%s\n' "${BASELINE_TIMES[@]}" | sort -n | awk "NR==$(( (RUNS+1)/2 ))")
CANDIDATE_MEDIAN=$(printf '%s\n' "${CANDIDATE_TIMES[@]}" | sort -n | awk "NR==$(( (RUNS+1)/2 ))")

# Convert to ms (multiply by 1000)
BASELINE_MS=$(echo "$BASELINE_MEDIAN * 1000" | bc)
CANDIDATE_MS=$(echo "$CANDIDATE_MEDIAN * 1000" | bc)

# Compute speedup ratio
SPEEDUP=$(echo "scale=3; $BASELINE_MS / $CANDIDATE_MS" | bc 2>/dev/null || echo "0")

# All times
B_ALL=$(printf '%s\n' "${BASELINE_TIMES[@]}" | tr '\n' ',' | sed 's/,$//')
C_ALL=$(printf '%s\n' "${CANDIDATE_TIMES[@]}" | tr '\n' ',' | sed 's/,$//')

echo "{\"problem\":$P,\"baseline_median_ms\":$BASELINE_MS,\"candidate_median_ms\":$CANDIDATE_MS,\"speedup\":$SPEEDUP,\"correct\":$CORRECT,\"baseline_times\":[$B_ALL],\"candidate_times\":[$C_ALL]}"

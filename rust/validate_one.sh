#!/bin/bash
# Validate a single Rust Euler solution with timeout protection.
# Usage: ./validate_one.sh <problem_number> [timeout_seconds]
#
# Examples:
#   ./validate_one.sh 903         # 120s default timeout
#   ./validate_one.sh 574 60      # custom 60s timeout

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

if [ $# -lt 1 ]; then
    echo "Usage: $0 <problem_number> [timeout_seconds]"
    exit 1
fi

PROB=$1
TIMEOUT_SECS=${2:-120}
BIN="$SCRIPT_DIR/target/release/p${PROB}"
ANSWERS="$PROJECT_ROOT/data/answers.txt"

# Source cargo env if needed
if ! command -v cargo &>/dev/null; then
    source "$HOME/.cargo/env" 2>/dev/null || true
fi

# Build
echo "Building p${PROB}..."
cd "$SCRIPT_DIR"
if ! cargo build --release --bin "p${PROB}" 2>&1; then
    echo "FAIL: build error"
    exit 1
fi

if [ ! -x "$BIN" ]; then
    echo "FAIL: binary not found at $BIN"
    exit 1
fi

# Look up expected answer
EXPECTED=$(grep "Problem ${PROB}:" "$ANSWERS" 2>/dev/null | sed 's/.*: //' | tr -d '\r')
if [ -z "$EXPECTED" ]; then
    echo "WARNING: no expected answer found for problem ${PROB}"
fi

# Run with timeout
echo "Running p${PROB} (timeout=${TIMEOUT_SECS}s)..."
START=$(date +%s%N)
ACTUAL=$(timeout "$TIMEOUT_SECS" "$BIN" 2>/dev/null) || {
    EXIT_CODE=$?
    ELAPSED_MS=$(( ($(date +%s%N) - START) / 1000000 ))
    if [ $EXIT_CODE -eq 124 ]; then
        echo "TIMEOUT after ${ELAPSED_MS}ms (limit=${TIMEOUT_SECS}s)"
        exit 2
    else
        echo "ERROR: exit code $EXIT_CODE after ${ELAPSED_MS}ms"
        exit 1
    fi
}
ELAPSED_MS=$(( ($(date +%s%N) - START) / 1000000 ))

# Compare
if [ -z "$EXPECTED" ]; then
    echo "OK? output=${ACTUAL} time=${ELAPSED_MS}ms (no expected answer to compare)"
elif [ "$ACTUAL" = "$EXPECTED" ]; then
    echo "OK: ${ACTUAL} in ${ELAPSED_MS}ms"
else
    echo "WRONG: got=${ACTUAL} expected=${EXPECTED} time=${ELAPSED_MS}ms"
    exit 1
fi

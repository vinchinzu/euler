#!/bin/bash
# Incremental C solution validator
# Only validates solutions that haven't been validated yet or have changed since last validation
# Results stored in c/validated.json (one JSON object per line, jq-friendly)
# Usage: ./c/validate.sh [--all] [--fix] [NUMS...]
#   --all    Force re-validate everything
#   --fix    Only validate previously failed/missing
#   NUMS...  Validate specific problem numbers only

cd "$(dirname "$0")/.." || exit 1

VALIDATED="c/validated.json"
ANSWERS="data/answers.txt"
touch "$VALIDATED"

ALL=0
FIX=0
SPECIFIC=()

for arg in "$@"; do
    case "$arg" in
        --all) ALL=1 ;;
        --fix) FIX=1 ;;
        *) SPECIFIC+=("$arg") ;;
    esac
done

correct=0
wrong=0
timeout_count=0
compile_fail=0
skipped=0
wrong_list=""
timeout_list=""
compile_list=""
total_time_ms=0

for f in c/*.c; do
    num=$(basename "$f" .c)
    [[ "$num" =~ ^[0-9]+$ ]] || continue
    # Strip leading zeros for all comparisons
    n=$((10#$num))

    # If specific problems requested, only do those
    if [ ${#SPECIFIC[@]} -gt 0 ]; then
        found=0
        for s in "${SPECIFIC[@]}"; do
            [ "$((10#$s))" = "$n" ] && found=1 && break
        done
        [ $found -eq 0 ] && continue
    fi

    # Get expected answer
    expected=$(grep "^Problem ${n}:" "$ANSWERS" | sed 's/Problem [0-9]*: //')
    if [ -z "$expected" ]; then
        continue
    fi
    expected_clean=$(echo "$expected" | tr -d '[:space:]')

    # Compute file hash
    current_hash=$(md5sum "$f" | cut -d' ' -f1)

    # Check if already validated with same hash
    if [ $ALL -eq 0 ] && [ ${#SPECIFIC[@]} -eq 0 ]; then
        prev=$(grep "\"problem\":${n}," "$VALIDATED" 2>/dev/null | tail -1)
        if [ -n "$prev" ]; then
            prev_hash=$(echo "$prev" | sed 's/.*"hash":"\([^"]*\)".*/\1/')
            prev_status=$(echo "$prev" | sed 's/.*"status":"\([^"]*\)".*/\1/')
            if [ "$current_hash" = "$prev_hash" ]; then
                if [ "$prev_status" = "OK" ]; then
                    skipped=$((skipped+1))
                    correct=$((correct+1))
                    prev_ms=$(echo "$prev" | sed 's/.*"time_ms":\([0-9]*\).*/\1/')
                    total_time_ms=$((total_time_ms + prev_ms))
                    continue
                elif [ $FIX -eq 0 ]; then
                    continue
                fi
            fi
        fi
    fi

    # Compile
    extra_flags=""
    [ "$n" = "558" ] && extra_flags="-lgmp"
    gcc -O2 -o "/tmp/ctest_$n" "$f" -lm $extra_flags 2>/dev/null
    if [ $? -ne 0 ]; then
        compile_fail=$((compile_fail+1))
        compile_list="$compile_list $n"
        sed -i "/\"problem\":${n},/d" "$VALIDATED"
        echo "{\"problem\":${n},\"hash\":\"${current_hash}\",\"status\":\"COMPILE_FAIL\",\"answer\":\"\",\"time_ms\":0}" >> "$VALIDATED"
        continue
    fi

    # Run with 30s timeout, measure time
    start_ns=$(date +%s%N)
    actual=$(timeout 30 /tmp/ctest_$n 2>/dev/null | tr -d '[:space:]')
    exit_code=$?
    end_ns=$(date +%s%N)
    elapsed_ms=$(( (end_ns - start_ns) / 1000000 ))
    rm -f "/tmp/ctest_$n"

    if [ $exit_code -eq 124 ]; then
        timeout_count=$((timeout_count+1))
        timeout_list="$timeout_list $n"
        sed -i "/\"problem\":${n},/d" "$VALIDATED"
        echo "{\"problem\":${n},\"hash\":\"${current_hash}\",\"status\":\"TIMEOUT\",\"answer\":\"\",\"time_ms\":30000}" >> "$VALIDATED"
        continue
    fi

    total_time_ms=$((total_time_ms + elapsed_ms))

    if [ "$actual" = "$expected_clean" ]; then
        correct=$((correct+1))
        sed -i "/\"problem\":${n},/d" "$VALIDATED"
        echo "{\"problem\":${n},\"hash\":\"${current_hash}\",\"status\":\"OK\",\"answer\":\"${actual}\",\"time_ms\":${elapsed_ms}}" >> "$VALIDATED"
    else
        wrong=$((wrong+1))
        wrong_list="$wrong_list ${n}(got:${actual:-EMPTY},exp:$expected_clean)"
        sed -i "/\"problem\":${n},/d" "$VALIDATED"
        echo "{\"problem\":${n},\"hash\":\"${current_hash}\",\"status\":\"WRONG\",\"answer\":\"${actual}\",\"expected\":\"${expected_clean}\",\"time_ms\":${elapsed_ms}}" >> "$VALIDATED"
    fi
done

# Sort validated.json by problem number
awk -F'"problem":' '{print $2"|"$0}' "$VALIDATED" | sort -t'|' -k1 -n | cut -d'|' -f2 > "$VALIDATED.tmp" && mv "$VALIDATED.tmp" "$VALIDATED"

total_secs=$((total_time_ms / 1000))
total_ms=$((total_time_ms % 1000))

echo "=== VALIDATION RESULTS ==="
echo "Correct: $correct (${skipped} cached)"
echo "Wrong: $wrong"
echo "Timeout: $timeout_count"
echo "Compile fail: $compile_fail"
echo "Total C runtime: ${total_secs}.${total_ms}s"
echo ""
[ -n "$wrong_list" ] && echo "Wrong:$wrong_list"
[ -n "$timeout_list" ] && echo "Timeout:$timeout_list"
[ -n "$compile_list" ] && echo "Compile fail:$compile_list"

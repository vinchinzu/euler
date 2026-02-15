#!/usr/bin/env python3
"""Benchmark all Python Euler solutions with parallel execution."""

import subprocess
import time
import re
import os
import sys
from pathlib import Path
from concurrent.futures import ProcessPoolExecutor, as_completed

BASE = Path("/home/v/01_projects/euler_project/euler")

def load_answers(path):
    """Load answer key from data/answers.txt."""
    answers = {}
    with open(path) as f:
        for line in f:
            m = re.match(r"Problem\s+(\d+):\s+(.+)", line.strip())
            if m:
                num = int(m.group(1))
                ans = m.group(2).strip()
                answers[num] = ans
    return answers

def run_solution(problem_num, py_path, expected_answer):
    """Run a single solution and return (problem, answer, time, status, stderr_snippet)."""
    cmd = f"ulimit -v 2097152; timeout 300 python {py_path}"
    t0 = time.monotonic()
    try:
        result = subprocess.run(
            cmd, shell=True, capture_output=True, text=True,
            timeout=310, cwd=str(BASE)
        )
        elapsed = time.monotonic() - t0
        stdout = result.stdout.strip()
        stderr = result.stderr.strip()

        if result.returncode == 124:
            # timeout(1) returns 124 on timeout
            return (problem_num, "", elapsed, "timeout", stderr[-200:] if stderr else "")

        if result.returncode != 0:
            return (problem_num, stdout, elapsed, "error", stderr[-200:] if stderr else f"exit={result.returncode}")

        # Extract last line as the answer (some solutions print progress on earlier lines)
        lines = [l for l in stdout.split("\n") if l.strip()]
        got = lines[-1].strip() if lines else ""

        # Try to normalize: strip labels like "Solution: " or "Answer: "
        for prefix in ["Solution:", "Answer:", "Result:"]:
            if got.lower().startswith(prefix.lower()):
                got = got[len(prefix):].strip()

        if expected_answer is None:
            return (problem_num, got, elapsed, "no_key", "")

        if got == expected_answer:
            return (problem_num, got, elapsed, "correct", "")
        else:
            return (problem_num, got, elapsed, "wrong", f"expected={expected_answer}")

    except subprocess.TimeoutExpired:
        elapsed = time.monotonic() - t0
        return (problem_num, "", elapsed, "timeout", "subprocess timeout")
    except Exception as e:
        elapsed = time.monotonic() - t0
        return (problem_num, "", elapsed, "error", str(e)[:200])

def main():
    answers = load_answers(BASE / "data" / "answers.txt")
    print(f"Loaded {len(answers)} answers from answer key")

    # Find all python/NNN.py files
    py_files = sorted(BASE.glob("python/[0-9]*.py"))
    problems = []
    for p in py_files:
        m = re.match(r"(\d+)\.py$", p.name)
        if m:
            num = int(m.group(1))
            problems.append((num, str(p)))

    problems.sort(key=lambda x: x[0])
    total = len(problems)
    print(f"Found {total} solution files to benchmark")
    print(f"Running with max_workers=4, timeout=300s each")
    print(f"Estimated worst case: {total * 300 / 4 / 60:.0f} minutes")
    print("=" * 60)

    results = []
    completed = 0
    t_start = time.monotonic()

    with ProcessPoolExecutor(max_workers=4) as executor:
        futures = {}
        for num, path in problems:
            expected = answers.get(num)
            fut = executor.submit(run_solution, num, path, expected)
            futures[fut] = num

        for fut in as_completed(futures):
            r = fut.result()
            results.append(r)
            completed += 1

            if completed % 50 == 0 or completed == total:
                elapsed_total = time.monotonic() - t_start
                statuses = {}
                for rr in results:
                    statuses[rr[3]] = statuses.get(rr[3], 0) + 1
                print(f"[{completed}/{total}] elapsed={elapsed_total:.0f}s  statuses={statuses}")

    # Sort by problem number
    results.sort(key=lambda x: x[0])

    # Write TSV
    tsv_path = BASE / "benchmarks.tsv"
    with open(tsv_path, "w") as f:
        f.write("problem\tanswer\ttime_s\tstatus\n")
        for prob, ans, t, status, _extra in results:
            f.write(f"{prob}\t{ans}\t{t:.2f}\t{status}\n")

    print(f"\nResults written to {tsv_path}")

    # Summary
    total_time = time.monotonic() - t_start
    status_counts = {}
    times_by_status = {}
    for prob, ans, t, status, _extra in results:
        status_counts[status] = status_counts.get(status, 0) + 1
        times_by_status.setdefault(status, []).append(t)

    print(f"\n{'=' * 60}")
    print(f"BENCHMARK SUMMARY")
    print(f"{'=' * 60}")
    print(f"Total solutions:   {total}")
    print(f"Wall clock time:   {total_time:.1f}s ({total_time/60:.1f} min)")
    print()
    for status in ["correct", "wrong", "timeout", "error", "no_key"]:
        count = status_counts.get(status, 0)
        times = times_by_status.get(status, [])
        avg_t = sum(times) / len(times) if times else 0
        print(f"  {status:10s}: {count:4d}  (avg {avg_t:.1f}s)")

    print()

    # Slowest correct solutions
    correct = [(p, a, t, s, e) for p, a, t, s, e in results if s == "correct"]
    correct.sort(key=lambda x: -x[2])
    print("Top 20 slowest CORRECT solutions:")
    for p, a, t, s, e in correct[:20]:
        print(f"  Problem {p:4d}: {t:7.2f}s")

    # Wrong answers
    wrong = [(p, a, t, s, e) for p, a, t, s, e in results if s == "wrong"]
    if wrong:
        print(f"\nWRONG answers ({len(wrong)}):")
        for p, a, t, s, e in wrong:
            print(f"  Problem {p:4d}: got='{a}' {e}")

    # Timeouts
    timeouts = [(p, a, t, s, e) for p, a, t, s, e in results if s == "timeout"]
    if timeouts:
        print(f"\nTIMEOUTS ({len(timeouts)}):")
        for p, a, t, s, e in timeouts:
            print(f"  Problem {p:4d}")

    # Errors
    errors = [(p, a, t, s, e) for p, a, t, s, e in results if s == "error"]
    if errors:
        print(f"\nERRORS ({len(errors)}):")
        for p, a, t, s, e in errors:
            print(f"  Problem {p:4d}: {e[:80]}")

if __name__ == "__main__":
    main()

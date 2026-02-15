#!/usr/bin/env python3
"""
Benchmark and validate all Python solutions.

Usage:
    python bench.py                 # Full run, writes benchmarks.tsv
    python bench.py --quick 772 857 # Benchmark specific problems only
    python bench.py --top 10        # Show top N slowest from benchmarks.tsv
    python bench.py --total         # Show total time from benchmarks.tsv
    python bench.py --status        # Summary counts by status
    python bench.py --diff OLD NEW  # Compare two TSV files, show changes
"""
import argparse
import glob
import os
import re
import subprocess
import sys
import time
from concurrent.futures import ProcessPoolExecutor, as_completed


def load_answers(path="data/answers.txt"):
    """Load answer key from data/answers.txt."""
    answers = {}
    with open(path) as f:
        for line in f:
            m = re.match(r"Problem\s+(\d+):\s*(.+)", line.strip())
            if m:
                answers[int(m.group(1))] = m.group(2).strip()
    return answers


def run_one(prob_num, timeout=300):
    """Run a single solution and return (prob, answer, time_s, status)."""
    py_file = f"python/{prob_num}.py"
    if not os.path.exists(py_file):
        return (prob_num, "", 0.0, "missing")

    cmd = f"ulimit -v 2097152; timeout {timeout} python {py_file}"
    t0 = time.monotonic()
    try:
        r = subprocess.run(
            cmd, shell=True, capture_output=True, text=True,
            timeout=timeout + 10
        )
        elapsed = time.monotonic() - t0
        stdout = r.stdout.strip().split("\n")[-1] if r.stdout.strip() else ""

        if r.returncode == 124:  # timeout(1) exit code
            return (prob_num, stdout, elapsed, "timeout")
        elif r.returncode != 0:
            return (prob_num, stdout, elapsed, "error")
        else:
            return (prob_num, stdout, elapsed, "ok")
    except subprocess.TimeoutExpired:
        elapsed = time.monotonic() - t0
        return (prob_num, "", elapsed, "timeout")
    except Exception as e:
        elapsed = time.monotonic() - t0
        return (prob_num, "", elapsed, "error")


def check_correctness(prob_num, got, answers):
    """Return 'correct', 'wrong', or 'no_key'."""
    if prob_num not in answers:
        return "no_key"
    return "correct" if got == answers[prob_num] else "wrong"


def write_tsv(results, path="benchmarks.tsv"):
    """Write results to TSV file."""
    with open(path, "w") as f:
        f.write("problem\tanswer\ttime_s\tstatus\n")
        for prob, ans, t, status in sorted(results):
            f.write(f"{prob}\t{ans}\t{t:.3f}\t{status}\n")


def read_tsv(path="benchmarks.tsv"):
    """Read TSV file, return list of (prob, answer, time_s, status)."""
    results = []
    with open(path) as f:
        next(f)  # skip header
        for line in f:
            parts = line.strip().split("\t")
            if len(parts) == 4:
                results.append((int(parts[0]), parts[1], float(parts[2]), parts[3]))
    return results


def fmt_time(s):
    """Format seconds to human-readable string."""
    if s >= 60:
        return f"{s/60:.1f}m"
    elif s >= 1:
        return f"{s:.1f}s"
    else:
        return f"{s*1000:.0f}ms"


def print_top(results, n=10):
    """Print top N slowest correct solutions."""
    correct = [(p, a, t, s) for p, a, t, s in results if s == "correct"]
    correct.sort(key=lambda x: -x[2])
    print(f"Top {n} slowest (of {len(correct)} correct):\n")
    print(f"  {'#':<4} {'Problem':<10} {'Time':>10}  Answer")
    print(f"  {'─'*4} {'─'*10} {'─'*10}  {'─'*20}")
    for i, (p, a, t, s) in enumerate(correct[:n], 1):
        print(f"  {i:<4} P{p:<9} {fmt_time(t):>10}  {a[:30]}")


def print_total(results):
    """Print total time and counts."""
    correct = [t for _, _, t, s in results if s == "correct"]
    total = sum(correct)
    print(f"Total problems:  {len(results)}")
    print(f"Correct:         {len(correct)}")
    print(f"Total runtime:   {fmt_time(total)} ({total:.1f}s)")
    print(f"Average runtime: {fmt_time(total/len(correct) if correct else 0)}")
    print(f"Median runtime:  {fmt_time(sorted(correct)[len(correct)//2] if correct else 0)}")


def print_status(results):
    """Print status summary."""
    counts = {}
    for _, _, _, s in results:
        counts[s] = counts.get(s, 0) + 1
    for s in ["correct", "wrong", "timeout", "error", "missing", "no_key"]:
        if s in counts:
            print(f"  {s:<10} {counts[s]:>4}")


def print_diff(old_path, new_path):
    """Compare two TSV files and show what changed."""
    old = {p: (a, t, s) for p, a, t, s in read_tsv(old_path)}
    new = {p: (a, t, s) for p, a, t, s in read_tsv(new_path)}

    changes = []
    for p in sorted(set(old) | set(new)):
        o = old.get(p)
        n = new.get(p)
        if o is None:
            changes.append((p, "NEW", "", n[0], 0, n[1], "", n[2]))
        elif n is None:
            changes.append((p, "REMOVED", o[0], "", o[1], 0, o[2], ""))
        elif o[2] != n[2] or o[0] != n[0]:  # status or answer changed
            speedup = o[1] / n[1] if n[1] > 0 else float("inf")
            changes.append((p, "CHANGED", o[0], n[0], o[1], n[1], o[2], n[2], speedup))

    if not changes:
        print("No changes.")
        return

    print(f"{'Prob':<6} {'Type':<9} {'Old Status':<12} {'New Status':<12} {'Old Time':>10} {'New Time':>10} {'Speedup':>8}")
    print("─" * 75)
    for c in changes:
        if len(c) == 9:
            p, typ, oa, na, ot, nt, os_, ns, sp = c
            sp_str = f"{sp:.1f}x" if sp < 1000 else "inf"
            print(f"P{p:<5} {typ:<9} {os_:<12} {ns:<12} {fmt_time(ot):>10} {fmt_time(nt):>10} {sp_str:>8}")
        elif c[1] == "NEW":
            p, typ, _, na, _, nt, _, ns = c
            print(f"P{p:<5} {typ:<9} {'':12} {ns:<12} {'':>10} {fmt_time(nt):>10}")
        else:
            p, typ, oa, _, ot, _, os_, _ = c
            print(f"P{p:<5} {typ:<9} {os_:<12} {'':12} {fmt_time(ot):>10}")


def main():
    parser = argparse.ArgumentParser(description="Benchmark Project Euler solutions")
    parser.add_argument("--quick", nargs="+", type=int, help="Benchmark specific problems only")
    parser.add_argument("--top", type=int, metavar="N", help="Show top N slowest from benchmarks.tsv")
    parser.add_argument("--total", action="store_true", help="Show total time summary")
    parser.add_argument("--status", action="store_true", help="Show status counts")
    parser.add_argument("--diff", nargs=2, metavar=("OLD", "NEW"), help="Compare two TSV files")
    parser.add_argument("--workers", type=int, default=4, help="Parallel workers (default: 4)")
    parser.add_argument("--timeout", type=int, default=300, help="Per-problem timeout in seconds")
    parser.add_argument("-o", "--output", default="benchmarks.tsv", help="Output TSV file")
    parser.add_argument("--from-tsv", default="benchmarks.tsv", help="TSV file for --top/--total/--status")
    args = parser.parse_args()

    # Read-only modes (no benchmarking)
    if args.top:
        results = read_tsv(args.from_tsv)
        print_top(results, args.top)
        return
    if args.total:
        results = read_tsv(args.from_tsv)
        print_total(results)
        return
    if args.status:
        results = read_tsv(args.from_tsv)
        print_status(results)
        return
    if args.diff:
        print_diff(args.diff[0], args.diff[1])
        return

    # Benchmark mode
    answers = load_answers()

    if args.quick:
        probs = args.quick
    else:
        # Find all python/NNN.py
        probs = sorted(
            int(re.match(r"(\d+)\.py", os.path.basename(f)).group(1))
            for f in glob.glob("python/[0-9]*.py")
            if re.match(r"\d+\.py", os.path.basename(f))
        )

    print(f"Benchmarking {len(probs)} solutions with {args.workers} workers, {args.timeout}s timeout...")

    results = []
    done = 0
    t_start = time.monotonic()

    with ProcessPoolExecutor(max_workers=args.workers) as pool:
        futures = {pool.submit(run_one, p, args.timeout): p for p in probs}
        for fut in as_completed(futures):
            prob, ans, t, status = fut.result()
            # Check correctness
            corr = check_correctness(prob, ans, answers)
            if status == "ok":
                final_status = corr  # correct/wrong/no_key
            else:
                final_status = status  # timeout/error/missing

            results.append((prob, ans, t, final_status))
            done += 1
            if done % 50 == 0 or done == len(probs):
                elapsed = time.monotonic() - t_start
                print(f"  [{done}/{len(probs)}] {elapsed:.0f}s elapsed", file=sys.stderr)

    total_elapsed = time.monotonic() - t_start
    write_tsv(results, args.output)

    print(f"\nDone in {total_elapsed:.0f}s. Results written to {args.output}")
    print()
    print_status(results)
    print()
    print_total(results)
    print()
    print_top(results, 10)


if __name__ == "__main__":
    main()

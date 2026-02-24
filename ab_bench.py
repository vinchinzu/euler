#!/usr/bin/env python3
"""A/B benchmark: compare baseline (HEAD) vs candidate (working tree) for a Rust problem.

Usage: python3 ab_bench.py <problem_number> [warmup=2] [runs=7]
Output: JSON line with benchmark results.
"""
import json, os, subprocess, sys, time, shutil, statistics

REPO = os.path.dirname(os.path.abspath(__file__))
RUST_DIR = os.path.join(REPO, "rust")
ANSWERS = os.path.join(REPO, "data", "answers.txt")

# Ensure cargo is on PATH
env = os.environ.copy()
cargo_bin = os.path.expanduser("~/.cargo/bin")
env["PATH"] = f"{cargo_bin}:/usr/local/bin:/usr/bin:/bin:/usr/sbin:/sbin:" + env.get("PATH", "")

def get_expected(p):
    with open(ANSWERS, "r") as f:
        for line in f:
            line = line.strip().replace("\r", "")
            if line.startswith(f"Problem {p}: "):
                return line.split(": ", 1)[1]
    return None

def build_problem(p):
    r = subprocess.run(
        ["cargo", "build", "--release", "--bin", f"p{p}"],
        cwd=RUST_DIR, env=env, capture_output=True, text=True, timeout=120
    )
    return r.returncode == 0, r.stderr

def run_problem(p, timeout_s=120):
    binary = os.path.join(RUST_DIR, "target", "release", f"p{p}")
    try:
        start = time.monotonic()
        r = subprocess.run(
            [binary], capture_output=True, text=True, timeout=timeout_s, env=env
        )
        elapsed = time.monotonic() - start
        return r.stdout.strip(), elapsed
    except subprocess.TimeoutExpired:
        return "TIMEOUT", timeout_s

def benchmark(p, warmup, runs):
    for _ in range(warmup):
        run_problem(p)
    times = []
    for _ in range(runs):
        _, t = run_problem(p)
        times.append(t)
    return times

def main():
    p = int(sys.argv[1])
    warmup = int(sys.argv[2]) if len(sys.argv) > 2 else 2
    runs = int(sys.argv[3]) if len(sys.argv) > 3 else 7

    expected = get_expected(p)
    if not expected:
        print(json.dumps({"problem": p, "error": "no expected answer"}))
        sys.exit(1)

    src = os.path.join(RUST_DIR, "solutions", "src", "bin", f"p{p}.rs")

    # Save candidate
    with open(src, "r") as f:
        candidate_code = f.read()

    # Get baseline from git
    try:
        baseline_code = subprocess.check_output(
            ["git", "show", f"HEAD:rust/solutions/src/bin/p{p}.rs"],
            cwd=REPO, text=True
        )
    except subprocess.CalledProcessError:
        print(json.dumps({"problem": p, "error": "no HEAD version"}))
        sys.exit(1)

    # --- Baseline ---
    with open(src, "w") as f:
        f.write(baseline_code)

    ok, err = build_problem(p)
    if not ok:
        with open(src, "w") as f:
            f.write(candidate_code)
        print(json.dumps({"problem": p, "error": f"baseline build failed: {err[-200:]}"}))
        sys.exit(1)

    out, _ = run_problem(p)
    if out != expected:
        with open(src, "w") as f:
            f.write(candidate_code)
        print(json.dumps({"problem": p, "error": f"baseline incorrect: got '{out}' expected '{expected}'"}))
        sys.exit(1)

    baseline_times = benchmark(p, warmup, runs)

    # --- Candidate ---
    with open(src, "w") as f:
        f.write(candidate_code)

    ok, err = build_problem(p)
    if not ok:
        print(json.dumps({"problem": p, "error": f"candidate build failed: {err[-200:]}"}))
        sys.exit(1)

    out, _ = run_problem(p)
    correct = (out == expected)

    candidate_times = benchmark(p, warmup, runs)

    # Medians
    b_med = statistics.median(baseline_times) * 1000  # ms
    c_med = statistics.median(candidate_times) * 1000
    speedup = b_med / c_med if c_med > 0 else 0

    result = {
        "problem": p,
        "baseline_median_ms": round(b_med, 1),
        "candidate_median_ms": round(c_med, 1),
        "speedup": round(speedup, 3),
        "correct": correct,
        "baseline_times": [round(t*1000, 1) for t in baseline_times],
        "candidate_times": [round(t*1000, 1) for t in candidate_times],
    }
    print(json.dumps(result))

if __name__ == "__main__":
    main()

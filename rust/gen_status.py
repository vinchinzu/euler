#!/usr/bin/env python3
"""
Generate a status visualization (status.png) for Rust Project Euler solutions.

Reads existing validation data from validated.json (incremental cache) and
bench_results.csv, then validates any uncached solutions by running the
compiled binaries against data/answers.txt.

Produces a grid PNG: GREEN=PASS, RED=FAIL, ORANGE=TIMEOUT, GRAY=missing.
"""

import csv
import hashlib
import json
import os
import re
import subprocess
import sys
import time
from pathlib import Path

import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
import matplotlib.patches as mpatches
import numpy as np


# --- paths ---
SCRIPT_DIR = Path(__file__).resolve().parent
PROJECT_ROOT = SCRIPT_DIR.parent
ANSWERS_FILE = PROJECT_ROOT / "data" / "answers.txt"
BENCH_CSV = SCRIPT_DIR / "bench_results.csv"
VALIDATED_JSON = SCRIPT_DIR / "validated.json"
BIN_DIR = SCRIPT_DIR / "target" / "release"
SRC_DIR = SCRIPT_DIR / "solutions" / "src" / "bin"
STATUS_PNG = SCRIPT_DIR / "status.png"

TIMEOUT_SECS = 30
MAX_PROBLEM = 972  # show all problems up to the highest existing solution


def load_answers():
    """Parse data/answers.txt -> {problem_num: answer_string}."""
    answers = {}
    with open(ANSWERS_FILE) as f:
        for line in f:
            m = re.match(r"Problem\s+(\d+):\s+(.+)", line.strip())
            if m:
                num = int(m.group(1))
                ans = m.group(2).strip()
                answers[num] = ans
    return answers


def load_bench_results():
    """Parse bench_results.csv -> {problem_num: 'PASS'} for matching entries."""
    results = {}
    if not BENCH_CSV.exists():
        return results
    with open(BENCH_CSV) as f:
        reader = csv.DictReader(f)
        for row in reader:
            try:
                num = int(row["problem"])
                match = row.get("match", "").strip()
                if match == "YES":
                    results[num] = "PASS"
            except (ValueError, KeyError):
                continue
    return results


def load_validated():
    """Load validated.json -> {problem_num: {status, hash, answer, time_ms}}."""
    cache = {}
    if not VALIDATED_JSON.exists():
        return cache
    with open(VALIDATED_JSON) as f:
        for line in f:
            line = line.strip()
            if not line:
                continue
            try:
                obj = json.loads(line)
                cache[obj["problem"]] = obj
            except (json.JSONDecodeError, KeyError):
                continue
    return cache


def save_validated(cache):
    """Write validated.json sorted by problem number."""
    items = sorted(cache.values(), key=lambda x: x["problem"])
    with open(VALIDATED_JSON, "w") as f:
        for item in items:
            f.write(json.dumps(item) + "\n")


def file_md5(path):
    """Compute MD5 of a file."""
    h = hashlib.md5()
    with open(path, "rb") as f:
        for chunk in iter(lambda: f.read(8192), b""):
            h.update(chunk)
    return h.hexdigest()


def find_existing_solutions():
    """Return set of problem numbers that have source files."""
    solutions = set()
    for p in SRC_DIR.glob("p*.rs"):
        m = re.match(r"p(\d+)\.rs", p.name)
        if m:
            solutions.add(int(m.group(1)))
    return solutions


def run_solution(num):
    """Run a compiled Rust solution, return (status, answer, time_ms)."""
    bin_path = BIN_DIR / f"p{num:03d}"
    if not bin_path.exists():
        return ("MISSING_BIN", "", 0)

    start = time.monotonic()
    try:
        result = subprocess.run(
            [str(bin_path)],
            capture_output=True,
            text=True,
            timeout=TIMEOUT_SECS,
            cwd=str(PROJECT_ROOT),
        )
        elapsed_ms = int((time.monotonic() - start) * 1000)
        output = result.stdout.strip()
        # Take last non-empty line as the answer (some solutions print progress)
        lines = [l.strip() for l in output.split("\n") if l.strip()]
        answer = lines[-1] if lines else ""
        return ("RAN", answer, elapsed_ms)
    except subprocess.TimeoutExpired:
        elapsed_ms = int((time.monotonic() - start) * 1000)
        return ("TIMEOUT", "", elapsed_ms)
    except Exception as e:
        elapsed_ms = int((time.monotonic() - start) * 1000)
        return ("ERROR", str(e), elapsed_ms)


def validate_all(answers, bench, existing_solutions):
    """Validate all existing solutions. Returns {num: status} where status is PASS/FAIL/TIMEOUT/MISSING."""
    cache = load_validated()
    statuses = {}
    validated_count = 0
    total = len(existing_solutions)

    for num in sorted(existing_solutions):
        src_path = SRC_DIR / f"p{num:03d}.rs"
        if not src_path.exists():
            statuses[num] = "MISSING"
            continue

        current_hash = file_md5(src_path)

        # Check cache first
        if num in cache:
            cached = cache[num]
            if cached.get("hash") == current_hash and cached.get("status") in ("OK", "WRONG", "TIMEOUT"):
                if cached["status"] == "OK":
                    statuses[num] = "PASS"
                elif cached["status"] == "TIMEOUT":
                    statuses[num] = "TIMEOUT"
                else:
                    statuses[num] = "FAIL"
                continue

        # Check bench_results.csv (these are already validated)
        if num in bench:
            statuses[num] = "PASS"
            cache[num] = {
                "problem": num,
                "hash": current_hash,
                "status": "OK",
                "answer": "",
                "time_ms": 0,
                "source": "bench_results.csv",
            }
            continue

        # Need to validate by running
        expected = answers.get(num)
        if expected is None:
            statuses[num] = "MISSING"  # no known answer
            continue

        validated_count += 1
        status_str, answer, time_ms = run_solution(num)

        if status_str == "TIMEOUT":
            statuses[num] = "TIMEOUT"
            cache[num] = {
                "problem": num,
                "hash": current_hash,
                "status": "TIMEOUT",
                "answer": "",
                "time_ms": time_ms,
            }
            print(f"  P{num:03d}: TIMEOUT ({time_ms}ms)")
        elif status_str == "MISSING_BIN":
            statuses[num] = "MISSING"
        elif status_str == "ERROR":
            statuses[num] = "FAIL"
            cache[num] = {
                "problem": num,
                "hash": current_hash,
                "status": "WRONG",
                "answer": answer,
                "expected": expected,
                "time_ms": time_ms,
            }
            print(f"  P{num:03d}: ERROR - {answer}")
        else:
            # Compare answer
            expected_clean = expected.strip()
            answer_clean = answer.strip()
            if answer_clean == expected_clean:
                statuses[num] = "PASS"
                cache[num] = {
                    "problem": num,
                    "hash": current_hash,
                    "status": "OK",
                    "answer": answer_clean,
                    "time_ms": time_ms,
                }
                if validated_count % 50 == 0:
                    print(f"  P{num:03d}: PASS ({time_ms}ms) [{validated_count} validated]")
            else:
                statuses[num] = "FAIL"
                cache[num] = {
                    "problem": num,
                    "hash": current_hash,
                    "status": "WRONG",
                    "answer": answer_clean,
                    "expected": expected_clean,
                    "time_ms": time_ms,
                }
                print(f"  P{num:03d}: FAIL (got={answer_clean}, expected={expected_clean}, {time_ms}ms)")

        # Save periodically
        if validated_count % 100 == 0:
            save_validated(cache)

    save_validated(cache)
    return statuses, cache


def generate_status_png(statuses, existing_solutions, max_problem):
    """Generate the grid visualization."""
    COLS = 25
    rows = (max_problem + COLS - 1) // COLS

    # Color map
    COLORS = {
        "PASS": (0.30, 0.75, 0.30),     # green
        "FAIL": (0.90, 0.20, 0.20),     # red
        "TIMEOUT": (1.00, 0.60, 0.15),  # orange
        "MISSING": (0.40, 0.40, 0.40),  # gray
    }

    # Build the grid
    grid = np.zeros((rows, COLS, 3))
    grid[:] = (0.20, 0.20, 0.20)  # dark background for unused cells

    for prob in range(1, max_problem + 1):
        r = (prob - 1) // COLS
        c = (prob - 1) % COLS
        if prob in existing_solutions:
            status = statuses.get(prob, "MISSING")
            grid[r, c] = COLORS.get(status, COLORS["MISSING"])
        else:
            grid[r, c] = COLORS["MISSING"]

    # Count stats
    n_pass = sum(1 for s in statuses.values() if s == "PASS")
    n_fail = sum(1 for s in statuses.values() if s == "FAIL")
    n_timeout = sum(1 for s in statuses.values() if s == "TIMEOUT")
    n_solutions = len(existing_solutions)

    # Create figure
    cell_size = 0.26
    fig_w = COLS * cell_size + 1.2
    fig_h = rows * cell_size + 1.8

    fig, ax = plt.subplots(1, 1, figsize=(fig_w, fig_h))
    fig.patch.set_facecolor("#333333")
    ax.set_facecolor("#333333")

    # Draw cells
    for r in range(rows):
        for c_idx in range(COLS):
            prob = r * COLS + c_idx + 1
            if prob > max_problem:
                continue
            color = grid[r, c_idx]
            rect = plt.Rectangle(
                (c_idx, rows - 1 - r),
                0.92, 0.92,
                facecolor=color,
                edgecolor=(0.25, 0.25, 0.25),
                linewidth=0.5,
            )
            ax.add_patch(rect)

    # Row labels (problem numbers on the left)
    for r in range(rows):
        prob_start = r * COLS + 1
        if prob_start <= max_problem:
            ax.text(
                -0.3, rows - 1 - r + 0.45,
                str(prob_start),
                color="white",
                fontsize=5.5,
                ha="right",
                va="center",
                fontfamily="monospace",
            )

    ax.set_xlim(-1.0, COLS)
    ax.set_ylim(-0.5, rows + 0.5)
    ax.set_aspect("equal")
    ax.axis("off")

    # Title
    title = f"Project Euler Rust: {n_pass} OK / {n_solutions} solutions"
    ax.set_title(title, color="white", fontsize=11, fontweight="bold", pad=10)

    # Legend
    legend_patches = [
        mpatches.Patch(color=COLORS["PASS"], label=f"OK ({n_pass})"),
        mpatches.Patch(color=COLORS["FAIL"], label=f"Wrong ({n_fail})"),
        mpatches.Patch(color=COLORS["TIMEOUT"], label=f"Timeout ({n_timeout})"),
        mpatches.Patch(color=COLORS["MISSING"], label=f"No Rust ({max_problem - n_solutions})"),
    ]
    leg = ax.legend(
        handles=legend_patches,
        loc="lower center",
        bbox_to_anchor=(0.5, -0.02),
        ncol=4,
        fontsize=7.5,
        frameon=False,
        labelcolor="white",
    )

    plt.tight_layout()
    fig.savefig(STATUS_PNG, dpi=150, bbox_inches="tight", facecolor=fig.get_facecolor())
    plt.close(fig)
    print(f"\nSaved {STATUS_PNG}")
    print(f"  PASS: {n_pass}  FAIL: {n_fail}  TIMEOUT: {n_timeout}  Missing: {max_problem - n_solutions}")


def main():
    print("Loading answers...")
    answers = load_answers()
    print(f"  {len(answers)} answers loaded")

    print("Loading bench_results.csv...")
    bench = load_bench_results()
    print(f"  {len(bench)} pre-validated from bench")

    print("Scanning Rust solutions...")
    existing = find_existing_solutions()
    print(f"  {len(existing)} solution source files found")

    max_prob = max(existing) if existing else 600
    print(f"  Max problem number: {max_prob}")

    print("Validating solutions...")
    statuses, cache = validate_all(answers, bench, existing)

    print("Generating status.png...")
    generate_status_png(statuses, existing, max_prob)


if __name__ == "__main__":
    main()

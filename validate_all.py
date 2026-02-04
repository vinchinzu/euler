#!/usr/bin/env python3
"""
Validate all Python Euler solutions against known answers.
Runs each solution with a timeout and reports results.
"""
import subprocess
import sys
import os
import re
import json
import time
from pathlib import Path

EULER_DIR = Path("/home/v/01_projects/euler")
PYTHON_DIR = EULER_DIR / "euler" / "python"
SOLUTIONS_FILE = EULER_DIR / "Solutions.txt"
EULER_SOLUTIONS_FILE = EULER_DIR / "euler" / "solutions.txt"
JAVA_DIR = EULER_DIR / "java"
RESULTS_FILE = EULER_DIR / "validation_results.json"
TIMEOUT = 60  # seconds per solution

def load_answers():
    """Load known answers from Solutions.txt and euler/solutions.txt"""
    answers = {}

    # Parse Solutions.txt (format: "N. answer")
    if SOLUTIONS_FILE.exists():
        for line in SOLUTIONS_FILE.read_text().splitlines():
            line = line.strip()
            m = re.match(r'^(\d+)\.\s+(.+)$', line)
            if m:
                num = int(m.group(1))
                ans = m.group(2).strip()
                answers[num] = ans

    # Parse euler/solutions.txt (format: "Problem N: answer")
    if EULER_SOLUTIONS_FILE.exists():
        for line in EULER_SOLUTIONS_FILE.read_text().splitlines():
            line = line.strip()
            m = re.match(r'^Problem\s+(\d+):\s+(.+)$', line)
            if m:
                num = int(m.group(1))
                ans = m.group(2).strip()
                if num not in answers:
                    answers[num] = ans

    return answers

def extract_java_answers():
    """Extract answers from Java check() calls."""
    java_answers = {}
    if not JAVA_DIR.exists():
        return java_answers
    for jf in sorted(JAVA_DIR.glob("p*.java")):
        m = re.match(r'p(\d+)\.java', jf.name)
        if not m:
            continue
        num = int(m.group(1))
        text = jf.read_text()
        # Look for check(answer) patterns
        checks = re.findall(r'check\((\d+L?)\)', text)
        if checks:
            ans = checks[-1].rstrip('L')
            java_answers[num] = ans
        # Also look for string checks
        str_checks = re.findall(r'check\("([^"]+)"\)', text)
        if str_checks:
            java_answers[num] = str_checks[-1]
    return java_answers

def run_solution(py_file, timeout=TIMEOUT):
    """Run a python solution and capture output."""
    try:
        result = subprocess.run(
            [sys.executable, str(py_file)],
            capture_output=True,
            text=True,
            timeout=timeout,
            cwd=str(py_file.parent)
        )
        stdout = result.stdout.strip()
        stderr = result.stderr.strip()
        # Extract the last line that looks like a number/answer
        lines = stdout.splitlines()
        answer = None
        if lines:
            # Try last line first
            last = lines[-1].strip()
            # Remove common prefixes
            for prefix in ["Solution:", "Answer:", "Result:", "The answer is"]:
                if last.lower().startswith(prefix.lower()):
                    last = last[len(prefix):].strip()
            answer = last
        return {
            "returncode": result.returncode,
            "stdout": stdout,
            "stderr": stderr[:500],
            "answer": answer,
            "error": None
        }
    except subprocess.TimeoutExpired:
        return {"returncode": -1, "stdout": "", "stderr": "", "answer": None, "error": "TIMEOUT"}
    except Exception as e:
        return {"returncode": -1, "stdout": "", "stderr": "", "answer": None, "error": str(e)}

def main():
    answers = load_answers()
    java_answers = extract_java_answers()

    # Merge java answers for problems not in Solutions.txt
    for num, ans in java_answers.items():
        if num not in answers:
            answers[num] = ans

    print(f"Loaded {len(answers)} known answers")
    print(f"Found {len(java_answers)} Java answers")

    # Find all python solutions
    py_files = sorted(PYTHON_DIR.glob("*.py"))
    print(f"Found {len(py_files)} Python solutions")

    # Load previous results
    prev_results = {}
    if RESULTS_FILE.exists():
        try:
            prev_results = json.loads(RESULTS_FILE.read_text())
        except:
            pass

    # Determine which to run
    start_from = int(sys.argv[1]) if len(sys.argv) > 1 else 1
    end_at = int(sys.argv[2]) if len(sys.argv) > 2 else 9999

    results = prev_results.copy()
    correct = 0
    wrong = 0
    errors = 0
    timeouts = 0
    no_answer = 0
    skipped = 0

    for py_file in py_files:
        m = re.match(r'^(\d+)\.py$', py_file.name)
        if not m:
            continue
        num = int(m.group(1))
        if num < start_from or num > end_at:
            continue

        expected = answers.get(num)

        # Skip if already validated correct in previous run
        key = str(num)
        if key in results and results[key].get("status") == "CORRECT":
            correct += 1
            skipped += 1
            continue

        print(f"Running problem {num:>4d}...", end=" ", flush=True)
        start = time.time()
        result = run_solution(py_file)
        elapsed = time.time() - start

        got = result["answer"]
        status = "UNKNOWN"

        if result["error"] == "TIMEOUT":
            status = "TIMEOUT"
            timeouts += 1
            print(f"TIMEOUT ({TIMEOUT}s)")
        elif result["returncode"] != 0:
            status = "ERROR"
            errors += 1
            print(f"ERROR: {result['stderr'][:100]}")
        elif got is None or got == "":
            status = "NO_OUTPUT"
            no_answer += 1
            print(f"NO OUTPUT")
        elif expected is not None:
            if got == expected:
                status = "CORRECT"
                correct += 1
                print(f"CORRECT ({elapsed:.1f}s) = {got}")
            else:
                status = "WRONG"
                wrong += 1
                print(f"WRONG: got '{got}' expected '{expected}' ({elapsed:.1f}s)")
        else:
            status = "NO_KNOWN_ANSWER"
            no_answer += 1
            print(f"No known answer, got: {got} ({elapsed:.1f}s)")

        results[key] = {
            "status": status,
            "got": got,
            "expected": expected,
            "time": round(elapsed, 2),
            "error": result.get("error"),
            "stderr": result.get("stderr", "")[:200]
        }

        # Save periodically
        if num % 10 == 0:
            RESULTS_FILE.write_text(json.dumps(results, indent=2))

    # Final save
    RESULTS_FILE.write_text(json.dumps(results, indent=2))

    print(f"\n{'='*60}")
    print(f"Results: {correct} correct, {wrong} wrong, {errors} errors, {timeouts} timeouts, {no_answer} no answer/unknown")
    print(f"Skipped (previously correct): {skipped}")
    print(f"Total: {correct + wrong + errors + timeouts + no_answer}")

    # Print wrong answers for fixing
    if wrong > 0:
        print(f"\nWRONG ANSWERS:")
        for key in sorted(results.keys(), key=lambda x: int(x)):
            r = results[key]
            if r["status"] == "WRONG":
                print(f"  Problem {key}: got '{r['got']}' expected '{r['expected']}'")

    if errors > 0:
        print(f"\nERRORS:")
        for key in sorted(results.keys(), key=lambda x: int(x)):
            r = results[key]
            if r["status"] == "ERROR":
                print(f"  Problem {key}: {r.get('stderr', '')[:100]}")

    if timeouts > 0:
        print(f"\nTIMEOUTS:")
        for key in sorted(results.keys(), key=lambda x: int(x)):
            r = results[key]
            if r["status"] == "TIMEOUT":
                print(f"  Problem {key}")

if __name__ == "__main__":
    main()

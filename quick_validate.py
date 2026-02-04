#!/usr/bin/env python3
"""Quick validation with short timeout to categorize problems fast."""
import subprocess, sys, os, re, time
from pathlib import Path

EULER_DIR = Path("/home/v/01_projects/euler")
PYTHON_DIR = EULER_DIR / "euler" / "python"
TIMEOUT = 30

def load_answers():
    answers = {}
    for line in (EULER_DIR / "Solutions.txt").read_text().splitlines():
        m = re.match(r'^(\d+)\.\s+(.+)$', line.strip())
        if m: answers[int(m.group(1))] = m.group(2).strip()
    for line in (EULER_DIR / "euler" / "solutions.txt").read_text().splitlines():
        m = re.match(r'^Problem\s+(\d+):\s+(.+)$', line.strip())
        if m and int(m.group(1)) not in answers:
            answers[int(m.group(1))] = m.group(2).strip()
    return answers

def run_solution(py_file):
    try:
        result = subprocess.run(
            [sys.executable, str(py_file)],
            capture_output=True, text=True, timeout=TIMEOUT,
            cwd=str(py_file.parent)
        )
        lines = result.stdout.strip().splitlines()
        answer = None
        if lines:
            last = lines[-1].strip()
            for prefix in ["Solution:", "Answer:", "Result:", "The answer is"]:
                if last.lower().startswith(prefix.lower()):
                    last = last[len(prefix):].strip()
            answer = last
        return result.returncode, answer, result.stderr[:200]
    except subprocess.TimeoutExpired:
        return -1, None, "TIMEOUT"
    except Exception as e:
        return -1, None, str(e)

answers = load_answers()
# Read problem list from stdin or args
problems = [int(x.strip()) for x in open(sys.argv[1]).readlines()]
start_idx = int(sys.argv[2]) if len(sys.argv) > 2 else 0
end_idx = int(sys.argv[3]) if len(sys.argv) > 3 else len(problems)
problems = problems[start_idx:end_idx]

for num in problems:
    py_file = PYTHON_DIR / f"{num:03d}.py"
    if not py_file.exists():
        py_file = PYTHON_DIR / f"{num}.py"
    if not py_file.exists():
        print(f"{num}|MISSING||{answers.get(num, '')}")
        continue

    t0 = time.time()
    rc, got, err = run_solution(py_file)
    elapsed = time.time() - t0
    expected = answers.get(num, "")

    if err == "TIMEOUT":
        status = "TIMEOUT"
    elif rc != 0:
        status = "ERROR"
    elif got is None or got == "":
        status = "NO_OUTPUT"
    elif got == expected:
        status = "CORRECT"
    else:
        status = "WRONG"

    print(f"{num}|{status}|{got}|{expected}|{elapsed:.1f}s|{err[:60] if status=='ERROR' else ''}")
    sys.stdout.flush()

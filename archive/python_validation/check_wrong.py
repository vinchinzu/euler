#!/usr/bin/env python3
"""Check entries that have wrong numeric answers in README."""
import subprocess, time, re, sys

answers = {}
with open("data/answers.txt") as f:
    for line in f:
        m = re.match(r"Problem\s+(\d+):\s*(.+)", line.strip())
        if m:
            answers[int(m.group(1))] = m.group(2).strip()

# Problems with wrong numeric answers in README (not verbose/None, but actual wrong numbers)
WRONG = [308, 316, 332, 372, 377, 388, 390, 391, 392, 396, 402, 420, 423]

for prob in WRONG:
    script = f"python/{prob}/{prob}.py"
    import os
    if not os.path.exists(script):
        script = f"python/{prob}.py"
    start = time.time()
    try:
        result = subprocess.run(["python", script], capture_output=True, text=True, timeout=120)
        elapsed = time.time() - start
        lines = [l.strip() for l in result.stdout.strip().split('\n') if l.strip()]
        answer = lines[-1] if lines else "None"
        expected = answers.get(prob, "?")
        correct = "PASS" if answer == expected else "FAIL"
        print(f"{prob}: {correct} | got={answer[:60]} | expected={expected} | {elapsed:.3f}s")
    except subprocess.TimeoutExpired:
        elapsed = time.time() - start
        print(f"{prob}: TIMEOUT | expected={answers.get(prob, '?')} | {elapsed:.3f}s")

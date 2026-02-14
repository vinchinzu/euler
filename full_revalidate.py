#!/usr/bin/env python3
"""Full revalidation of all Python solutions, generating updated README table."""
import subprocess
import time
import os
import re
import json
import concurrent.futures
import sys

def load_answers():
    """Load expected answers from data/answers.txt."""
    answers = {}
    with open("data/answers.txt") as f:
        for line in f:
            m = re.match(r"Problem\s+(\d+):\s*(.+)", line.strip())
            if m:
                answers[int(m.group(1))] = m.group(2).strip()
    return answers

def find_all_solutions():
    """Find all Python solution files."""
    probs = set()
    import glob
    for f in glob.glob("python/*.py"):
        m = re.match(r"python/(\d+)\.py", f)
        if m:
            probs.add(int(m.group(1)))
    for f in glob.glob("python/*/"):
        m = re.match(r"python/(\d+)/", f)
        if m:
            p = int(m.group(1))
            if os.path.exists(f"python/{p}/{p}.py"):
                probs.add(p)
    return sorted(probs)

def validate_problem(prob, expected, timeout_s=120):
    """Run a single problem and return result dict."""
    script = f"python/{prob}/{prob}.py"
    if not os.path.exists(script):
        script = f"python/{prob}.py"
    if not os.path.exists(script):
        return {"prob": prob, "answer": None, "time": 0, "correct": False, "status": "no_file"}

    start = time.time()
    try:
        result = subprocess.run(
            ["python", script],
            capture_output=True, text=True, timeout=timeout_s,
            cwd="/home/v/01_projects/euler_project/euler"
        )
        elapsed = time.time() - start
        stdout = result.stdout.strip()
        if not stdout:
            return {"prob": prob, "answer": None, "time": elapsed, "correct": False, "status": "no_output"}

        lines = [l.strip() for l in stdout.split('\n') if l.strip()]
        answer = lines[-1] if lines else ""

        correct = (answer == expected) if expected else False
        return {"prob": prob, "answer": answer, "time": elapsed, "correct": correct, "status": "ok"}
    except subprocess.TimeoutExpired:
        elapsed = time.time() - start
        return {"prob": prob, "answer": None, "time": elapsed, "correct": False, "status": "timeout"}
    except Exception as e:
        elapsed = time.time() - start
        return {"prob": prob, "answer": None, "time": elapsed, "correct": False, "status": str(e)}


def format_time(seconds):
    if seconds >= 1:
        return f"{seconds:.3f}s"
    else:
        return f"{int(seconds * 1000)}ms"


def main():
    # Determine which problems to validate
    if len(sys.argv) > 1 and sys.argv[1] == "--missing-only":
        # Only validate problems not in README
        readme_probs = set()
        with open("README.md") as f:
            in_table = False
            for line in f:
                if 'VALIDATION_RESULTS_START' in line:
                    in_table = True
                if 'VALIDATION_RESULTS_END' in line:
                    break
                if in_table:
                    m = re.match(r'\| (\d+) \|', line)
                    if m:
                        readme_probs.add(int(m.group(1)))
        all_probs = find_all_solutions()
        problems = [p for p in all_probs if p not in readme_probs]
        timeout_s = 120
    else:
        problems = find_all_solutions()
        timeout_s = 120

    answers = load_answers()
    print(f"Validating {len(problems)} problems with {timeout_s}s timeout, 20 parallel workers...", file=sys.stderr)

    results = []
    completed = 0
    with concurrent.futures.ThreadPoolExecutor(max_workers=20) as executor:
        futures = {executor.submit(validate_problem, p, answers.get(p, ""), timeout_s): p for p in problems}
        for future in concurrent.futures.as_completed(futures):
            r = future.result()
            completed += 1
            mark = "PASS" if r["correct"] else "FAIL"
            print(f"  [{completed}/{len(problems)}] {r['prob']:>3}: {mark} | {format_time(r['time']):>10} | {r['status']}", file=sys.stderr)
            results.append(r)

    results.sort(key=lambda x: x["prob"])

    # Output JSON results
    with open("validation_results_new.json", "w") as f:
        json.dump(results, f, indent=2)

    passed = [r for r in results if r["correct"]]
    failed = [r for r in results if not r["correct"]]

    print(f"\nPassed: {len(passed)}, Failed: {len(failed)}", file=sys.stderr)

    # Output table rows for passed problems
    for r in passed:
        print(f"| {r['prob']:03d} | `{r['answer']}` | {format_time(r['time'])} |")

    # Summary of failures
    print(f"\n# FAILED ({len(failed)}):", file=sys.stderr)
    for r in failed:
        exp = answers.get(r["prob"], "?")
        ans = str(r["answer"])[:50] if r["answer"] else "None"
        print(f"  {r['prob']}: got={ans}, expected={exp}, status={r['status']}", file=sys.stderr)


if __name__ == "__main__":
    main()

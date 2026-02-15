#!/usr/bin/env python3
"""Re-run timeout problems with 300s timeout. Crash-safe with incremental saves."""
import subprocess, time, os, re, json, sys

RESULTS_FILE = "timeout_passed.txt"
PROGRESS_FILE = "timeout_progress.json"

def load_answers():
    answers = {}
    with open("data/answers.txt") as f:
        for line in f:
            m = re.match(r"Problem\s+(\d+):\s*(.+)", line.strip())
            if m:
                answers[int(m.group(1))] = m.group(2).strip()
    return answers

def format_time(s):
    return f"{s:.3f}s" if s >= 1 else f"{int(s*1000)}ms"

def load_progress():
    """Load already-completed problems from previous run."""
    if os.path.exists(PROGRESS_FILE):
        with open(PROGRESS_FILE) as f:
            return json.load(f)
    return {}

def save_progress(progress):
    with open(PROGRESS_FILE, "w") as f:
        json.dump(progress, f)

def main():
    answers = load_answers()

    # Get timeout list from validation results
    with open("validation_results_new.json") as f:
        data = json.load(f)
    timeouts = sorted(r["prob"] for r in data if r["status"] == "timeout")

    # Load any previous progress
    progress = load_progress()
    already_done = set(int(k) for k in progress.keys())
    remaining = [p for p in timeouts if p not in already_done]

    print(f"Total timeouts: {len(timeouts)}, already done: {len(already_done)}, remaining: {len(remaining)}")

    for i, prob in enumerate(remaining):
        print(f"[{i+1}/{len(remaining)}] Running problem {prob}...", end=" ", flush=True)

        script = f"python/{prob}/{prob}.py"
        if not os.path.exists(script):
            script = f"python/{prob}.py"
        if not os.path.exists(script):
            progress[str(prob)] = {"answer": None, "time": 0, "correct": False, "status": "no_file"}
            save_progress(progress)
            print("NO FILE")
            continue

        start = time.time()
        try:
            # Use ulimit to cap memory at 2GB per process
            result = subprocess.run(
                ["bash", "-c", f"ulimit -v 2097152; timeout 300 python {script}"],
                capture_output=True, text=True, timeout=310,
                cwd="/home/v/01_projects/euler_project/euler"
            )
            elapsed = time.time() - start
            stdout = result.stdout.strip()
            if not stdout:
                progress[str(prob)] = {"answer": None, "time": elapsed, "correct": False, "status": "no_output"}
                save_progress(progress)
                print(f"NO OUTPUT ({format_time(elapsed)})")
                continue

            lines = [l.strip() for l in stdout.split('\n') if l.strip()]
            answer = lines[-1] if lines else ""
            expected = answers.get(prob, "")
            correct = answer == expected

            progress[str(prob)] = {"answer": answer, "time": elapsed, "correct": correct, "status": "ok"}
            save_progress(progress)

            mark = "PASS" if correct else "FAIL"
            print(f"{mark} | {answer[:40]} | {format_time(elapsed)}")

        except subprocess.TimeoutExpired:
            elapsed = time.time() - start
            progress[str(prob)] = {"answer": None, "time": elapsed, "correct": False, "status": "timeout"}
            save_progress(progress)
            print(f"TIMEOUT ({format_time(elapsed)})")
        except Exception as e:
            elapsed = time.time() - start
            progress[str(prob)] = {"answer": None, "time": elapsed, "correct": False, "status": str(e)[:50]}
            save_progress(progress)
            print(f"ERROR: {e}")

    # Summary
    passed = {k: v for k, v in progress.items() if v["correct"]}
    print(f"\nTotal passed: {len(passed)} / {len(timeouts)}")

    # Write passed results as table rows
    with open(RESULTS_FILE, "w") as f:
        for prob in sorted(passed.keys(), key=int):
            v = passed[prob]
            f.write(f"| {int(prob):03d} | `{v['answer']}` | {format_time(v['time'])} |\n")

    print(f"Wrote {len(passed)} entries to {RESULTS_FILE}")

if __name__ == "__main__":
    main()

#!/usr/bin/env python3
"""
Analyze validation results and report on problem status.

Usage:
    python analyze_status.py [--max N] [--only-missing] [--only-failing]
"""

import json
import sys
from pathlib import Path


def main():
    max_problem = None
    only_missing = False
    only_failing = False

    # Parse args
    i = 1
    while i < len(sys.argv):
        if sys.argv[i] == "--max":
            i += 1
            max_problem = int(sys.argv[i])
        elif sys.argv[i] == "--only-missing":
            only_missing = True
        elif sys.argv[i] == "--only-failing":
            only_failing = True
        elif sys.argv[i] in ["-h", "--help"]:
            print(__doc__)
            return
        i += 1

    # Load validation results
    results_file = Path("validation_results.json")
    if not results_file.exists():
        print("Error: validation_results.json not found. Run validate.py first.")
        sys.exit(1)

    with open(results_file) as f:
        data = json.load(f)
        results = data.get("results", data)

    # Load solutions to know which answers we have
    solutions_file = Path("solutions.txt")
    known_answers = set()
    if solutions_file.exists():
        with open(solutions_file) as f:
            for line in f:
                if line.startswith("Problem "):
                    try:
                        num = int(line.split(":")[0].replace("Problem ", "").strip())
                        known_answers.add(num)
                    except:
                        pass

    # Check which problems exist as Python files
    python_dir = Path("python")
    existing_problems = set()
    for p in python_dir.glob("*.py"):
        try:
            num = int(p.stem)
            existing_problems.add(num)
        except ValueError:
            pass

    # Analyze
    max_p = max_problem or 300

    # Problems by status
    passed = []
    failed = []
    errors = []
    timeouts = []
    unknown = []
    missing_solution = []  # No .py file
    missing_answer = []    # No known answer

    for p in range(1, max_p + 1):
        p_str = str(p)

        if p not in existing_problems:
            missing_solution.append(p)
            continue

        if p_str in results:
            status = results[p_str].get("status", "unknown")
            if status == "passed":
                passed.append(p)
            elif status == "failed":
                failed.append(p)
            elif status == "error":
                errors.append(p)
            elif status == "timeout":
                timeouts.append(p)
            else:
                unknown.append(p)
        else:
            # Not tested yet
            if p in known_answers:
                unknown.append(p)
            else:
                missing_answer.append(p)

    # Output
    if only_missing:
        # Just print missing solution numbers
        if missing_solution:
            print(",".join(str(p) for p in missing_solution))
        return

    if only_failing:
        # Print failed + error numbers
        problems = failed + errors
        if problems:
            print(",".join(str(p) for p in sorted(problems)))
        return

    print(f"Problem Status Report (1-{max_p})")
    print("=" * 60)
    print(f"Passed:             {len(passed)}")
    print(f"Failed:             {len(failed)}")
    print(f"Errors:             {len(errors)}")
    print(f"Timeouts:           {len(timeouts)}")
    print(f"Unknown/Not tested: {len(unknown)}")
    print(f"Missing answer key: {len(missing_answer)}")
    print(f"Missing solution:   {len(missing_solution)}")
    print()

    if failed:
        print(f"Failed ({len(failed)}): {failed}")
        # Show expected vs actual for failures
        for p in failed[:5]:
            r = results.get(str(p), {})
            print(f"  Problem {p}: expected '{r.get('expected')}', got '{r.get('actual')}'")
        if len(failed) > 5:
            print(f"  ... and {len(failed) - 5} more")
        print()

    if errors:
        print(f"Errors ({len(errors)}): {errors}")
        # Show error messages
        for p in errors[:5]:
            r = results.get(str(p), {})
            err = r.get("error", "unknown")[:100]
            print(f"  Problem {p}: {err}")
        if len(errors) > 5:
            print(f"  ... and {len(errors) - 5} more")
        print()

    if timeouts:
        print(f"Timeouts ({len(timeouts)}): {timeouts}")
        print()

    if missing_solution:
        print(f"Missing solution ({len(missing_solution)}): {missing_solution[:20]}")
        if len(missing_solution) > 20:
            print(f"  ... and {len(missing_solution) - 20} more")
        print()

    # Success rate
    total_with_answers = len([p for p in range(1, max_p + 1) if p in known_answers and p in existing_problems])
    if total_with_answers > 0:
        success_rate = len(passed) / total_with_answers * 100
        print(f"Success rate (where answers known): {success_rate:.1f}% ({len(passed)}/{total_with_answers})")


if __name__ == "__main__":
    main()

#!/usr/bin/env python3
"""
Project Euler Python Solutions Validator

Features:
- Two-pass validation: 60s timeout first pass, 120s second pass for timeouts
- Saves results after every problem (fault-tolerant, interruptible)
- Skips already validated correct answers
- Uses solutions.txt as answer source
- Clear progress output
- Sequential processing (no threading issues)
"""

import json
import os
import subprocess
import sys
import time
from pathlib import Path
from typing import Dict, Any, Optional
from datetime import datetime


def load_expected_answers(filepath: str = "solutions.txt") -> Dict[int, str]:
    """Load expected answers from solutions.txt (nayuki format: 'Problem NNN: answer')."""
    answers = {}

    if not Path(filepath).exists():
        print(f"Error: {filepath} not found!")
        return answers

    print(f"Loading answers from {filepath}...")

    with open(filepath, "r", encoding="utf-8") as f:
        for line in f:
            line = line.strip()
            if not line:
                continue

            # Skip header lines
            if line.startswith("Project Euler") or line.startswith("http") or line.startswith("Computed"):
                continue

            # Handle "Problem NNN: answer" format
            if line.startswith("Problem "):
                try:
                    # "Problem 001: 233168"
                    parts = line.split(": ", 1)
                    if len(parts) == 2:
                        problem_part = parts[0].replace("Problem ", "").strip()
                        answer_part = parts[1].strip()
                        problem_num = int(problem_part)
                        answers[problem_num] = answer_part
                except ValueError:
                    continue

    print(f"Loaded {len(answers)} answers")
    return answers


def run_python_script(script_path: Path, timeout: int = 60) -> Dict[str, Any]:
    """
    Run a Python script and return the result.

    Returns:
        Dict with keys: status, expected, actual, error, timeout, runtime
    """
    result = {
        "status": "error",
        "expected": None,
        "actual": None,
        "error": None,
        "timeout": False,
        "runtime": None
    }

    try:
        # Run the script with timeout
        start_time = time.time()
        process = subprocess.run(
            [sys.executable, str(script_path)],
            capture_output=True,
            text=True,
            timeout=timeout,
            cwd=Path.cwd()
        )
        end_time = time.time()
        runtime = end_time - start_time

        stdout = process.stdout.strip()
        stderr = process.stderr.strip()

        result["runtime"] = round(runtime, 3)

        if process.returncode == 0:
            # Get the last line of output (the answer)
            lines = stdout.split('\n')
            result["actual"] = lines[-1].strip() if lines else ""
            result["status"] = "unknown"  # Will be set when compared to expected
        else:
            result["error"] = f"Return code {process.returncode}"
            if stderr:
                # Truncate long error messages
                stderr_short = stderr[:200] + "..." if len(stderr) > 200 else stderr
                result["error"] += f": {stderr_short}"

    except subprocess.TimeoutExpired:
        result["timeout"] = True
        result["error"] = f"Timeout after {timeout}s"
        result["runtime"] = timeout
    except Exception as e:
        result["error"] = f"Exception: {str(e)[:200]}"
        result["status"] = "error"

    return result


def load_existing_results(results_file: Path) -> Dict[str, Dict[str, Any]]:
    """Load existing validation results."""
    if results_file.exists():
        try:
            with open(results_file, "r", encoding="utf-8") as f:
                data = json.load(f)
                # Handle both old format (direct dict) and new format (with metadata)
                if "results" in data:
                    return data["results"]
                else:
                    return data
        except (json.JSONDecodeError, IOError) as e:
            print(f"Warning: Could not load existing results: {e}")
    return {}


def save_results(results: Dict[str, Dict[str, Any]], output_file: Path):
    """Save validation results to JSON file with metadata."""
    data = {
        "last_updated": datetime.now().isoformat(),
        "total_problems": len(results),
        "results": results
    }

    # Write atomically by writing to temp file first
    temp_file = output_file.with_suffix('.tmp')
    try:
        with open(temp_file, "w", encoding="utf-8") as f:
            json.dump(data, f, indent=2, ensure_ascii=False)

        # Rename to final file (atomic on most systems)
        temp_file.replace(output_file)
    except Exception as e:
        print(f"Warning: Could not save results: {e}")
        if temp_file.exists():
            temp_file.unlink()


def get_python_solutions(python_dir: Path, max_problem: Optional[int] = None) -> list[tuple[Path, int]]:
    """Get all Python solution files and their problem numbers."""
    solutions = []

    # Look for .py files in python/ directory
    for script_path in sorted(python_dir.glob("*.py")):
        try:
            problem_num = int(script_path.stem)
            if max_problem is None or problem_num <= max_problem:
                solutions.append((script_path, problem_num))
        except ValueError:
            # Skip non-numeric filenames
            continue

    return sorted(solutions, key=lambda x: x[1])


def validate_solutions_pass(
    solutions: list[tuple[Path, int]],
    expected_answers: Dict[int, str],
    results: Dict[str, Dict[str, Any]],
    output_file: Path,
    timeout: int,
    pass_name: str
) -> Dict[str, Dict[str, Any]]:
    """
    Run one validation pass with specified timeout.
    """
    # Filter to problems that need testing in this pass
    need_testing = []
    for script_path, problem_num in solutions:
        problem_key = str(problem_num)

        # Check if marked for re-validation (force_revalidate flag)
        force_revalidate = False
        if problem_key in results:
            force_revalidate = results[problem_key].get("force_revalidate", False)

        # Skip if already passed (unless force_revalidate is set)
        if problem_key in results and results[problem_key].get("status") == "passed" and not force_revalidate:
            continue

        # For second pass, only retry timeouts from first pass (unless force_revalidate)
        if pass_name == "Second Pass" and problem_key in results and not force_revalidate:
            if results[problem_key].get("status") != "timeout":
                continue

        need_testing.append((script_path, problem_num))

    if not need_testing:
        print(f"\n{pass_name}: No problems to test")
        return results

    print(f"\n{'='*70}")
    print(f"{pass_name}: Testing {len(need_testing)} problems (timeout: {timeout}s)")
    print(f"{'='*70}")

    for idx, (script_path, problem_num) in enumerate(need_testing, 1):
        problem_key = str(problem_num)

        # Get expected answer
        expected = expected_answers.get(problem_num)

        # Check if this is a re-validation
        was_revalidating = results.get(problem_key, {}).get("force_revalidate", False)

        # Print progress header
        revalidate_mark = " [RE-VALIDATING]" if was_revalidating else ""
        print(f"\n[{idx}/{len(need_testing)}] Problem {problem_num}{revalidate_mark}... ", end="", flush=True)

        # Run the script
        result = run_python_script(script_path, timeout=timeout)
        result["expected"] = expected
        result["pass"] = pass_name

        # Clear force_revalidate flag and record that it was revalidated
        if was_revalidating:
            result["revalidated"] = True
            result["force_revalidate"] = False
        else:
            result["force_revalidate"] = False

        # Determine status
        if result["timeout"]:
            result["status"] = "timeout"
            print(f"TIMEOUT ({result['runtime']:.1f}s)")
        elif result["error"]:
            result["status"] = "error"
            print(f"ERROR ({result['runtime']:.3f}s)")
            print(f"    {result['error']}")
        elif expected is None:
            result["status"] = "unknown"
            print(f"UNKNOWN - no answer ({result['runtime']:.3f}s)")
            if result['actual']:
                print(f"    Output: {result['actual'][:80]}")
        elif result["actual"] == expected:
            result["status"] = "passed"
            print(f"PASSED ({result['runtime']:.3f}s)")
        else:
            result["status"] = "failed"
            print(f"FAILED ({result['runtime']:.3f}s)")
            print(f"    Expected: {expected}")
            print(f"    Got:      {result['actual'][:80] if result['actual'] else '(empty)'}")

        # Update results
        results[problem_key] = result

        # Save after every problem (fault-tolerant)
        save_results(results, output_file)

    return results


def print_summary(results: Dict[str, Dict[str, Any]], max_problem: Optional[int] = None):
    """Print a summary of the validation results."""
    if not results:
        print("\nNo results to summarize.")
        return

    # Filter results if max_problem specified
    if max_problem:
        filtered = {k: v for k, v in results.items() if int(k) <= max_problem}
    else:
        filtered = results

    total = len(filtered)
    passed = sum(1 for r in filtered.values() if r["status"] == "passed")
    failed = sum(1 for r in filtered.values() if r["status"] == "failed")
    errors = sum(1 for r in filtered.values() if r["status"] == "error")
    timeouts = sum(1 for r in filtered.values() if r["status"] == "timeout")
    unknown = sum(1 for r in filtered.values() if r["status"] == "unknown")

    # Calculate runtime statistics for passed solutions
    passed_runtimes = [r["runtime"] for r in filtered.values()
                      if r["status"] == "passed" and r["runtime"] is not None]
    total_runtime = sum(passed_runtimes) if passed_runtimes else 0
    avg_runtime = total_runtime / len(passed_runtimes) if passed_runtimes else 0
    max_runtime = max(passed_runtimes) if passed_runtimes else 0

    # Find slowest passed problems
    passed_with_runtime = [(k, r["runtime"]) for k, r in filtered.items()
                          if r["status"] == "passed" and r["runtime"] is not None]
    slowest = sorted(passed_with_runtime, key=lambda x: x[1], reverse=True)[:10]

    print("\n" + "="*70)
    print("VALIDATION SUMMARY")
    if max_problem:
        print(f"(Problems 1-{max_problem})")
    print("="*70)
    print(f"Total solutions tested: {total}")
    print(f"  Passed: {passed} ({100*passed/total:.1f}%)" if total > 0 else "  Passed: 0")
    print(f"  Failed: {failed}")
    print(f"  Errors: {errors}")
    print(f"  Timeouts: {timeouts}")
    print(f"  Unknown: {unknown}")

    if passed > 0:
        print(f"\nPerformance Stats (for {passed} passed solutions):")
        print(f"   Total runtime: {total_runtime:.2f}s")
        print(f"   Average runtime: {avg_runtime:.3f}s")
        print(f"   Max runtime: {max_runtime:.3f}s")

        if slowest:
            print(f"\n   Slowest 10 passed problems:")
            for problem_num, runtime in slowest:
                print(f"     Problem {problem_num}: {runtime:.3f}s")

    if failed > 0 or errors > 0 or timeouts > 0:
        print(f"\nFAILED/ERROR/TIMEOUT PROBLEMS:")

        failed_list = []
        error_list = []
        timeout_list = []

        for problem_num, result in sorted(filtered.items(), key=lambda x: int(x[0])):
            if result["status"] == "failed":
                failed_list.append(problem_num)
            elif result["status"] == "error":
                error_list.append(problem_num)
            elif result["status"] == "timeout":
                timeout_list.append(problem_num)

        if failed_list:
            print(f"\n   Failed ({len(failed_list)}): {', '.join(failed_list)}")
        if error_list:
            print(f"\n   Errors ({len(error_list)}): {', '.join(error_list)}")
        if timeout_list:
            print(f"\n   Timeouts ({len(timeout_list)}): {', '.join(timeout_list)}")

    print("="*70)


def main():
    """Main entry point."""
    # Parse command line arguments
    first_pass_timeout = 60
    second_pass_timeout = 120
    target_problems = None
    max_problem = None
    ci_mode = False
    revalidate_all = False

    i = 1
    while i < len(sys.argv):
        arg = sys.argv[i]
        if arg in ["--help", "-h"]:
            print("Usage: python validate.py [options]")
            print()
            print("Options:")
            print("  --timeout SECONDS    Set first pass timeout (default: 60)")
            print("  --problems 1,2,3     Test specific problems only")
            print("  --max N              Only test problems <= N")
            print("  --ci                 CI mode: exit with error if any failures")
            print("  --revalidate         Force revalidation of all problems")
            print()
            return
        elif arg == "--timeout":
            i += 1
            first_pass_timeout = int(sys.argv[i])
            second_pass_timeout = first_pass_timeout * 2
        elif arg == "--problems":
            i += 1
            target_problems = set()
            for p in sys.argv[i].split(","):
                val = p.strip()
                if val:
                    target_problems.add(int(val))
        elif arg == "--max":
            i += 1
            max_problem = int(sys.argv[i])
        elif arg == "--ci":
            ci_mode = True
        elif arg == "--revalidate":
            revalidate_all = True
        i += 1

    # Check if we're in the right directory
    python_dir = Path("python")
    if not python_dir.exists():
        print("Error: python/ directory not found.")
        sys.exit(1)

    # Load expected answers
    expected_answers = load_expected_answers("solutions.txt")
    if not expected_answers:
        sys.exit(1)

    # Get all Python solutions
    solutions = get_python_solutions(python_dir, max_problem)

    # Filter if needed
    if target_problems:
        solutions = [s for s in solutions if s[1] in target_problems]
        print(f"Filtered to {len(solutions)} target problems")

    print(f"Found {len(solutions)} Python solutions")

    # Load existing results
    output_file = Path("validation_results.json")
    results = load_existing_results(output_file)

    # Force re-validate if requested
    if revalidate_all:
        for s in solutions:
            p_key = str(s[1])
            if p_key in results:
                results[p_key]["force_revalidate"] = True
    elif target_problems:
        for s in solutions:
            p_key = str(s[1])
            if p_key in results:
                results[p_key]["force_revalidate"] = True

    already_passed = sum(1 for k, r in results.items()
                         if r.get("status") == "passed"
                         and (max_problem is None or int(k) <= max_problem))
    print(f"Already validated: {already_passed} passed problems")

    # First pass with configurable timeout
    results = validate_solutions_pass(
        solutions,
        expected_answers,
        results,
        output_file,
        timeout=first_pass_timeout,
        pass_name="First Pass"
    )

    # Second pass with 2x timeout for problems that timed out in first pass
    results = validate_solutions_pass(
        solutions,
        expected_answers,
        results,
        output_file,
        timeout=second_pass_timeout,
        pass_name="Second Pass"
    )

    # Final save
    save_results(results, output_file)
    print(f"\nResults saved to {output_file}")

    # Print summary
    print_summary(results, max_problem)

    # CI mode: exit with error if any failures
    if ci_mode:
        filtered = results if max_problem is None else {k: v for k, v in results.items() if int(k) <= max_problem}
        failed = sum(1 for r in filtered.values() if r["status"] in ["failed", "error"])
        if failed > 0:
            print(f"\nCI: {failed} failures detected, exiting with error")
            sys.exit(1)


if __name__ == "__main__":
    main()

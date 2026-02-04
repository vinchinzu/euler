#!/usr/bin/env python3
"""
Project Euler Python Solutions Validator (Improved)

Features:
- Two-pass validation: 60s timeout first pass, 120s second pass for timeouts
- Saves results after every problem (fault-tolerant, interruptible)
- Skips already validated correct answers
- Uses solutions_b.txt as answer source
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


def load_expected_answers(filepath: str = "solutions_b.txt") -> Dict[int, str]:
    """Load expected answers from solutions_b.txt or Solutions.txt."""
    answers = {}

    if not Path(filepath).exists():
        print(f"Warning: {filepath} not found, trying Solutions.txt...")
        filepath = "Solutions.txt"

    if not Path(filepath).exists():
        print(f"Error: No answer file found!")
        return answers

    print(f"Loading answers from {filepath}...")

    with open(filepath, "r", encoding="utf-8") as f:
        for line in f:
            line = line.strip()
            if not line:
                continue

            # Skip header lines
            if line.startswith("Project Euler") or line.startswith("http"):
                continue

            # Handle both "number. answer" and "number." formats
            if ". " in line:
                try:
                    problem_part, answer_part = line.split(". ", 1)
                    problem_num = int(problem_part)
                    answers[problem_num] = answer_part.strip()
                except ValueError:
                    continue
            elif line.endswith("."):
                try:
                    problem_num = int(line[:-1])
                    # No answer available for this problem
                    answers[problem_num] = None
                except ValueError:
                    continue

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
            result["actual"] = stdout
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


def get_python_solutions(python_dir: Path) -> list[tuple[Path, int]]:
    """Get all Python solution files and their problem numbers."""
    solutions = []

    # Look for .py files in python/ directory
    for script_path in sorted(python_dir.glob("*.py")):
        try:
            problem_num = int(script_path.stem)
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

    Args:
        solutions: List of (script_path, problem_num) tuples
        expected_answers: Dict of problem_num -> expected answer
        results: Existing results dict (will be updated)
        output_file: Path to save results after each problem
        timeout: Timeout in seconds
        pass_name: Name of this pass (for display)

    Returns:
        Updated results dict
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
            print(f"⏰ TIMEOUT ({result['runtime']:.1f}s)")
        elif result["error"]:
            result["status"] = "error"
            print(f"🔥 ERROR ({result['runtime']:.3f}s)")
            print(f"    {result['error']}")
        elif expected is None:
            result["status"] = "unknown"
            print(f"❓ UNKNOWN - no answer ({result['runtime']:.3f}s)")
            print(f"    Output: {result['actual'][:80]}")
        elif result["actual"] == expected:
            result["status"] = "passed"
            # Show performance improvement if this was a re-validation
            if was_revalidating and problem_key in results:
                old_runtime = results[problem_key].get("runtime", 0)
                new_runtime = result["runtime"]
                if old_runtime > 0:
                    speedup = ((old_runtime - new_runtime) / old_runtime) * 100
                    if speedup > 0:
                        print(f"✅ PASSED ({result['runtime']:.3f}s) 🚀 {speedup:.1f}% faster!")
                    elif speedup < -5:  # More than 5% slower
                        print(f"✅ PASSED ({result['runtime']:.3f}s) ⚠️  {abs(speedup):.1f}% slower")
                    else:
                        print(f"✅ PASSED ({result['runtime']:.3f}s)")
                else:
                    print(f"✅ PASSED ({result['runtime']:.3f}s)")
            else:
                print(f"✅ PASSED ({result['runtime']:.3f}s)")
        else:
            result["status"] = "failed"
            print(f"❌ FAILED ({result['runtime']:.3f}s)")
            print(f"    Expected: {expected}")
            print(f"    Got:      {result['actual'][:80]}")

        # Update results
        results[problem_key] = result

        # Save after every problem (fault-tolerant)
        save_results(results, output_file)

    return results


def print_summary(results: Dict[str, Dict[str, Any]]):
    """Print a summary of the validation results."""
    if not results:
        print("\nNo results to summarize.")
        return

    total = len(results)
    passed = sum(1 for r in results.values() if r["status"] == "passed")
    failed = sum(1 for r in results.values() if r["status"] == "failed")
    errors = sum(1 for r in results.values() if r["status"] == "error")
    timeouts = sum(1 for r in results.values() if r["status"] == "timeout")
    unknown = sum(1 for r in results.values() if r["status"] == "unknown")

    # Calculate runtime statistics for passed solutions
    passed_runtimes = [r["runtime"] for r in results.values()
                      if r["status"] == "passed" and r["runtime"] is not None]
    total_runtime = sum(passed_runtimes) if passed_runtimes else 0
    avg_runtime = total_runtime / len(passed_runtimes) if passed_runtimes else 0
    max_runtime = max(passed_runtimes) if passed_runtimes else 0

    # Find slowest passed problems
    passed_with_runtime = [(k, r["runtime"]) for k, r in results.items()
                          if r["status"] == "passed" and r["runtime"] is not None]
    slowest = sorted(passed_with_runtime, key=lambda x: x[1], reverse=True)[:10]

    print("\n" + "="*70)
    print("VALIDATION SUMMARY")
    print("="*70)
    print(f"Total solutions tested: {total}")
    print(f"✅ Passed: {passed} ({100*passed/total:.1f}%)")
    print(f"❌ Failed: {failed}")
    print(f"🔥 Errors: {errors}")
    print(f"⏰ Timeouts: {timeouts}")
    print(f"❓ Unknown: {unknown}")

    if passed > 0:
        print(f"\n⏱️  Performance Stats (for {passed} passed solutions):")
        print(f"   Total runtime: {total_runtime:.2f}s")
        print(f"   Average runtime: {avg_runtime:.3f}s")
        print(f"   Max runtime: {max_runtime:.3f}s")

        if slowest:
            print(f"\n   Slowest 10 passed problems:")
            for problem_num, runtime in slowest:
                print(f"     Problem {problem_num}: {runtime:.3f}s")

    if failed > 0 or errors > 0 or timeouts > 0:
        print(f"\n⚠️  FAILED/ERROR/TIMEOUT PROBLEMS:")

        failed_list = []
        error_list = []
        timeout_list = []

        for problem_num, result in sorted(results.items(), key=lambda x: int(x[0])):
            if result["status"] == "failed":
                failed_list.append(problem_num)
            elif result["status"] == "error":
                error_list.append(problem_num)
            elif result["status"] == "timeout":
                timeout_list.append(problem_num)

        if failed_list:
            print(f"\n   ❌ Failed ({len(failed_list)}): {', '.join(failed_list)}")
        if error_list:
            print(f"\n   🔥 Errors ({len(error_list)}): {', '.join(error_list)}")
        if timeout_list:
            print(f"\n   ⏰ Timeouts ({len(timeout_list)}): {', '.join(timeout_list)}")

    print("="*70)


def main():
    """Main entry point."""
    # Parse command line arguments
    first_pass_timeout = 60
    second_pass_timeout = 120
    target_problems = None

    if len(sys.argv) > 1:
        if sys.argv[1] in ["--help", "-h"]:
            print("Usage: python validate.py [--timeout SECONDS] [--problems 1,2,3]")
            print()
            return
        elif sys.argv[1] == "--timeout":
            if len(sys.argv) < 3:
                print("Error: --timeout requires a value")
                sys.exit(1)
            try:
                first_pass_timeout = int(sys.argv[2])
                second_pass_timeout = first_pass_timeout * 2
            except ValueError:
                print(f"Error: invalid timeout value: {sys.argv[2]}")
                sys.exit(1)
        elif sys.argv[1] == "--problems":
            if len(sys.argv) < 3:
                print("Error: --problems requires a value")
                sys.exit(1)
            try:
                target_problems = set()
                for p in sys.argv[2].split(","):
                    val = p.strip()
                    if val:
                        target_problems.add(int(val))
            except ValueError:
                print("Error: invalid problem list")
                sys.exit(1)

    # Check if we're in the right directory
    python_dir = Path("python")
    if not python_dir.exists():
        print("Error: python/ directory not found.")
        sys.exit(1)

    # Load expected answers
    expected_answers = load_expected_answers("solutions_b.txt")
    if not expected_answers:
        sys.exit(1)

    # Get all Python solutions
    solutions = get_python_solutions(python_dir)
    
    # Filter if needed
    if target_problems:
        solutions = [s for s in solutions if s[1] in target_problems]
        print(f"Filtered to {len(solutions)} target problems")
        
    print(f"Found {len(solutions)} Python solutions")

    # Load existing results
    output_file = Path("validation_results.json")
    results = load_existing_results(output_file)
    
    # If using target_problems, force re-validate them
    if target_problems:
        for s in solutions:
            p_key = str(s[1])
            if p_key in results:
                results[p_key]["force_revalidate"] = True

    already_passed = sum(1 for r in results.values() if r.get("status") == "passed")
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
    print(f"\n✅ Results saved to {output_file}")

    # Print summary
    print_summary(results)


if __name__ == "__main__":
    main()

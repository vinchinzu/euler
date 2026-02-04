#!/usr/bin/env python3
"""
Mark problems for re-validation

This script marks one or more problems with the 'force_revalidate' flag,
causing them to be re-tested on the next validation run even if they
previously passed.

Usage:
    python mark_for_revalidation.py 296           # Mark single problem
    python mark_for_revalidation.py 296 150 187   # Mark multiple problems
    python mark_for_revalidation.py --list        # List marked problems
    python mark_for_revalidation.py --clear 296   # Clear flag for problem 296
    python mark_for_revalidation.py --clear-all   # Clear all flags
"""

import json
import sys
from pathlib import Path
from datetime import datetime


def load_results(results_file: Path = Path("validation_results.json")) -> dict:
    """Load validation results."""
    if not results_file.exists():
        print(f"Error: {results_file} not found")
        print("Run validation first: python validate.py")
        return None

    with open(results_file, 'r') as f:
        data = json.load(f)

    # Handle both old and new format
    if "results" in data:
        return data
    else:
        return {"results": data, "last_updated": "unknown", "total_problems": len(data)}


def save_results(data: dict, results_file: Path = Path("validation_results.json")):
    """Save validation results."""
    # Write atomically
    temp_file = results_file.with_suffix('.tmp')
    try:
        with open(temp_file, "w", encoding="utf-8") as f:
            json.dump(data, f, indent=2, ensure_ascii=False)
        temp_file.replace(results_file)
        print(f"✅ Saved to {results_file}")
    except Exception as e:
        print(f"Error saving: {e}")
        if temp_file.exists():
            temp_file.unlink()


def mark_problems(problem_nums: list[int], data: dict, reason: str = "updated") -> int:
    """Mark problems for re-validation."""
    results = data.get("results", {})
    marked_count = 0

    for num in problem_nums:
        key = str(num)

        if key not in results:
            print(f"⚠️  Problem {num}: Not found in validation results (not yet validated)")
            continue

        if results[key].get("status") != "passed":
            print(f"⚠️  Problem {num}: Not marked (status: {results[key].get('status')})")
            print(f"    Only passed problems can be marked for re-validation")
            continue

        # Mark for re-validation
        results[key]["force_revalidate"] = True
        results[key]["revalidate_reason"] = reason
        results[key]["marked_at"] = datetime.now().isoformat()

        old_runtime = results[key].get("runtime", 0)
        print(f"✅ Problem {num}: Marked for re-validation (current: {old_runtime:.3f}s)")
        marked_count += 1

    # Update data
    data["results"] = results
    data["last_updated"] = datetime.now().isoformat()

    return marked_count


def clear_marks(problem_nums: list[int], data: dict) -> int:
    """Clear re-validation marks."""
    results = data.get("results", {})
    cleared_count = 0

    for num in problem_nums:
        key = str(num)

        if key not in results:
            print(f"⚠️  Problem {num}: Not found in validation results")
            continue

        if results[key].get("force_revalidate", False):
            results[key]["force_revalidate"] = False
            print(f"✅ Problem {num}: Cleared re-validation flag")
            cleared_count += 1
        else:
            print(f"ℹ️  Problem {num}: Not marked for re-validation")

    # Update data
    data["results"] = results
    data["last_updated"] = datetime.now().isoformat()

    return cleared_count


def clear_all_marks(data: dict) -> int:
    """Clear all re-validation marks."""
    results = data.get("results", {})
    cleared_count = 0

    for key, info in results.items():
        if info.get("force_revalidate", False):
            info["force_revalidate"] = False
            cleared_count += 1

    # Update data
    data["results"] = results
    data["last_updated"] = datetime.now().isoformat()

    return cleared_count


def list_marked(data: dict):
    """List all problems marked for re-validation."""
    results = data.get("results", {})
    marked = []

    for key, info in results.items():
        if info.get("force_revalidate", False):
            marked.append({
                "num": int(key),
                "runtime": info.get("runtime", 0),
                "reason": info.get("revalidate_reason", "unknown"),
                "marked_at": info.get("marked_at", "unknown")
            })

    if not marked:
        print("No problems marked for re-validation")
        return

    marked.sort(key=lambda x: x["num"])

    print(f"\n⚡ Problems Marked for Re-validation ({len(marked)}):")
    print("="*70)
    for item in marked:
        print(f"Problem {item['num']:03d}: {item['runtime']:.3f}s")
        print(f"  Reason: {item['reason']}")
        print(f"  Marked: {item['marked_at'][:19]}")
        print()


def main():
    """Main entry point."""
    if len(sys.argv) < 2:
        print(__doc__)
        return 1

    # Parse arguments
    if sys.argv[1] in ["--help", "-h"]:
        print(__doc__)
        return 0

    if sys.argv[1] == "--list":
        data = load_results()
        if data:
            list_marked(data)
        return 0

    if sys.argv[1] == "--clear-all":
        data = load_results()
        if not data:
            return 1

        count = clear_all_marks(data)
        if count > 0:
            save_results(data)
            print(f"\n✅ Cleared {count} re-validation marks")
        else:
            print("No marks to clear")
        return 0

    if sys.argv[1] == "--clear":
        if len(sys.argv) < 3:
            print("Error: --clear requires problem number(s)")
            print("Usage: python mark_for_revalidation.py --clear 296 150")
            return 1

        data = load_results()
        if not data:
            return 1

        try:
            problem_nums = [int(arg) for arg in sys.argv[2:]]
        except ValueError:
            print("Error: Problem numbers must be integers")
            return 1

        count = clear_marks(problem_nums, data)
        if count > 0:
            save_results(data)
            print(f"\n✅ Cleared {count} re-validation marks")
        return 0

    # Mark problems for re-validation
    try:
        problem_nums = [int(arg) for arg in sys.argv[1:]]
    except ValueError:
        print("Error: Problem numbers must be integers")
        print("Usage: python mark_for_revalidation.py 296 150 187")
        return 1

    data = load_results()
    if not data:
        return 1

    count = mark_problems(problem_nums, data)
    if count > 0:
        save_results(data)
        print(f"\n✅ Marked {count} problems for re-validation")
        print("\nRun validation to re-test:")
        print("  python validate.py")
    else:
        print("\n⚠️  No problems were marked")

    return 0


if __name__ == "__main__":
    sys.exit(main())

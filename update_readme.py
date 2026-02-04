#!/usr/bin/env python3
"""
Update README.md with Python Validation Results

Extracts passed solutions from validation_results.json and updates README.md
with a formatted table showing problem numbers, answers, and runtimes.
"""

import json
from pathlib import Path
from datetime import datetime


def load_validation_results(json_file: str = "validation_results.json") -> dict:
    """Load validation results from JSON file."""
    if not Path(json_file).exists():
        print(f"Error: {json_file} not found")
        return None

    with open(json_file, 'r') as f:
        data = json.load(f)

    # Handle both old and new format
    if "results" in data:
        return data
    else:
        return {"results": data, "last_updated": "unknown", "total_problems": len(data)}


def format_runtime(seconds: float) -> str:
    """Format runtime in a human-readable way."""
    if seconds < 0.001:
        return f"{seconds*1000000:.0f}µs"
    elif seconds < 1.0:
        return f"{seconds*1000:.0f}ms"
    else:
        return f"{seconds:.3f}s"


def generate_results_table(data: dict) -> str:
    """Generate markdown table from validation results."""
    results = data.get("results", {})
    last_updated = data.get("last_updated", "unknown")

    # Filter for passed problems only
    passed = {
        int(num): info
        for num, info in results.items()
        if info.get("status") == "passed"
    }

    if not passed:
        return "No passed solutions found.\n"

    # Sort by problem number
    sorted_problems = sorted(passed.items())

    # Build markdown table
    lines = []
    lines.append("# Project Euler - Python Solutions Results")
    lines.append("")
    lines.append(f"**Last Updated:** {last_updated}")
    lines.append(f"**Total Passed:** {len(passed)} problems")
    lines.append("")
    lines.append("| Problem | Answer | Runtime |")
    lines.append("|---------|--------|---------|")

    for prob_num, info in sorted_problems:
        answer = info.get("actual", "N/A")
        runtime = info.get("runtime", 0)
        runtime_str = format_runtime(runtime)

        # Clean answer: take first line only, strip whitespace, truncate to 80 chars
        if answer:
            answer = answer.split('\n')[0].strip()
            if len(answer) > 80:
                answer = answer[:77] + "..."

        lines.append(f"| {prob_num:03d} | `{answer}` | {runtime_str} |")

    lines.append("")

    # Add summary statistics
    runtimes = [info.get("runtime", 0) for info in passed.values()]
    total_time = sum(runtimes)
    avg_time = total_time / len(runtimes) if runtimes else 0
    max_time = max(runtimes) if runtimes else 0
    min_time = min(runtimes) if runtimes else 0

    lines.append("## Performance Summary")
    lines.append("")
    lines.append(f"- **Total Runtime:** {format_runtime(total_time)}")
    lines.append(f"- **Average Runtime:** {format_runtime(avg_time)}")
    lines.append(f"- **Fastest:** {format_runtime(min_time)}")
    lines.append(f"- **Slowest:** {format_runtime(max_time)}")
    lines.append("")

    # Find slowest problems
    slowest = sorted(sorted_problems, key=lambda x: x[1].get("runtime", 0), reverse=True)[:10]
    if slowest:
        lines.append("### Top 10 Slowest Problems")
        lines.append("")
        for prob_num, info in slowest:
            runtime = info.get("runtime", 0)
            lines.append(f"- Problem {prob_num:03d}: {format_runtime(runtime)}")
        lines.append("")

    return "\n".join(lines)


def update_readme(table_content: str, readme_file: str = "README.md"):
    """Update README.md with the results table."""
    readme_path = Path(readme_file)

    # Markers for the section to replace
    start_marker = "<!-- VALIDATION_RESULTS_START -->"
    end_marker = "<!-- VALIDATION_RESULTS_END -->"

    if readme_path.exists():
        with open(readme_path, 'r') as f:
            content = f.read()

        # Check if markers exist
        if start_marker in content and end_marker in content:
            # Replace content between markers
            before = content.split(start_marker)[0]
            after = content.split(end_marker)[1]
            new_content = f"{before}{start_marker}\n\n{table_content}\n{end_marker}{after}"
        else:
            # Append to end with markers
            new_content = f"{content}\n\n{start_marker}\n\n{table_content}\n{end_marker}\n"
    else:
        # Create new README
        new_content = f"{start_marker}\n\n{table_content}\n{end_marker}\n"

    # Write updated content
    with open(readme_path, 'w') as f:
        f.write(new_content)

    print(f"✅ Updated {readme_file}")


def main():
    """Main entry point."""
    # Load validation results
    print("Loading validation results...")
    data = load_validation_results()

    if data is None:
        return 1

    results = data.get("results", {})
    total = len(results)
    passed = sum(1 for r in results.values() if r.get("status") == "passed")

    print(f"Found {total} total results, {passed} passed")

    # Generate table
    print("Generating results table...")
    table = generate_results_table(data)

    # Print to console
    print("\n" + "="*70)
    print(table)
    print("="*70)

    # Update README
    print("\nUpdating README.md...")
    update_readme(table)

    print("\n✅ Done!")
    return 0


if __name__ == "__main__":
    exit(main())

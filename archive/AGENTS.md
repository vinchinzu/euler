## Agents Benchmark: Project Euler

This project aims to use autonomous agents to solve Project Euler problems
and evolve those agents to require less human intervention over time. The
repository provides a simple, repeatable benchmark harness and conventions
so improvements can be measured consistently.

**CURRENT FOCUS: Problems < 800 only. Work one problem at a time.**

* Keep the root directory clean; use problem-specific directories for all non-permanent files (e.g. `unsolved/123/`).
* Never delete wrong.txt for candidate solutions; as they are history of wrong decisions until validated.

### Objectives

- Focus on problems < 800 first
- Work on ONE problem at a time
- Reduce human intervention needed for solving new problems
- Improve solution quality, runtime, and reliability
- Track progress with clear, automatable metrics


### Workflow

**Granular Problem-by-Problem Approach:**

1) Ask the agent to produce a solution for a **single specific** Project Euler problem
   as a standalone Python file under its folder.
2) The agent will test the solution using their `run_python_file` tool to verify it works correctly.
3) **Manual Review Only:** Run the benchmark harness manually (`uv run python `)
   only when you want to review multiple solutions and collect aggregate metrics.
5) Inspect logs and metrics, then iterate on the agent or prompt style to
   reduce manual edits and improve outcomes.

**Key Changes:**
- Agents work on **one problem at a time** and test individually
- Full benchmark runs are **manual only** - agents never trigger them automatically
- Single-problem testing is available for granular development and debugging


Tips for prompting:
- Specify filename and location (e.g., `300.py`).
- Require the script to print only the final numeric answer on stdout.
- Require type hints and ≤ 88-char lines.
- Encourage small, focused functions and clear variable names.
- The agent will automatically test solutions using their `run_python_file` tool.


### Solution script contract

Generated solutions under `` should:
- Be standalone Python scripts (no external network calls).
- Print only the final answer to stdout when executed.
- Exit with status 0 on success.
- Include type hints and keep lines ≤ 88 chars.
- Prefer deterministic, efficient algorithms.

Minimal template agents can aim for:

```python
from __future__ import annotations

def solve() -> int:
    # ... compute the answer deterministically ...
    return 42

if __name__ == "__main__":
    print(solve())
```

### Integrity and anti-cheating policy

- Compute answers algorithmically. Never hard‑code final answers or rely on
  memorized results.
- No magic numbers. Avoid unexplained numeric literals in algorithms. Use
  descriptive named constants with clear derivations or references, or compute
  values directly from definitions.
- Preserve the original problem. Each `XXX.py` must begin
  with a top‑of‑file block comment that includes the Project Euler problem
  number, title, and the full description verbatim. This prevents
  misrepresentation and aids review. The script must still print only the final
  numeric answer on stdout.
  External materials may be consulted for algorithmic background only—never to
- Determinism and validation. Prefer deterministic algorithms. Optional internal
  sanity checks with small parameters are allowed but must not print anything.

Results:
- `debug/run_report.txt`: Human-readable summary.
- `debug/run_report.json`: Structured data for analysis.
- `debug/outputs/**`: Per-file stdout/stderr logs.

A successful solution has `return_code == 0`, no timeout, and a numeric answer
printed to its `.stdout.txt` log.

### Metrics

The current harness provides:
- Total files executed
- Number of successes and failures
- Per-file runtime (ms), timeout flag, output sizes

We use these as baseline benchmark signals:
- Problems solved (count)
- Timeouts/failures (count)
- Median/mean runtime per problem (ms)

Future extension ideas (optional):
- Count of tool calls per solve attempt
- Number of agent steps/retries
- Human intervention rate (e.g., manual edits per solution)

### Iteration loop

1) Generate or refine solutions for a chosen problem set.
2) Run the harness.

Use `pytest` for regression and edge cases on shared helpers or solution logic.

```bash
uv run --frozen pytest
```

Guidelines:
- Favor small, pure functions that are easy to test.
- Avoid mocks when possible; prefer real, deterministic inputs.
- Add regression tests when fixing bugs in shared utilities.

### Adding dependencies

Use `uv add` (never `pip`):

```bash
uv add <package>
uv add --dev <package>
```

Prefer standard library and simple numeric methods where feasible to keep
solutions portable and fast.

### Conventions

- Filenames: `300.py`, `301.py`, etc.
- Output: print only the numeric answer on stdout.
- Style: type hints, ≤ 88-char lines, small focused functions.
- Determinism: no network calls or nondeterministic behavior.

### Troubleshooting

  algorithm.
- Empty stdout: Ensure the script prints the final answer in the main guard.
- Large stderr: Investigate for accidental logging or exceptions.


## File Organization Rules

* Never create temporary files (tmp_*) or test files (test_*) directly in the project root.
* For problem-specific temporary or test files, create them in subfolders like `364/test_*` or `unsolved/842/tmp_*`.
* Keep the root directory clean; use problem-specific directories for all non-permanent files (e.g. `unsolved/123/`).
* When testing or debugging, prefer in-memory computation or short-lived scripts over persistent files.

## Additional Agent Guidelines

* Follow existing code style: type hints, ≤88 char lines, small functions.
* Never move solution files to the `solved/` directory until the answer has been manually validated by the user. This is to prevent incorrect solutions from being marked as solved.

This rule helps maintain project cleanliness and scalability.

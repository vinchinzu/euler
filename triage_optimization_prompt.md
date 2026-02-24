Task: Build a static optimization triage across remaining Euler problems without executing solutions.

Context:
- Repository root: current working directory (euler/)
- Target artifacts likely include validated.json and any validation_results*.json in this repo.
- The user wants per-problem optimization potential ranking based on existing runtime/score data in validation JSON files + code inspection.

Requirements:
1) Enumerate all candidate problem entries from validation JSON files (prefer validated.json if present, also inspect validation_results*.json if they exist).
2) For each problem, inspect implementation code and metadata only. DO NOT run binaries, tests, benchmarks, or scripts.
3) Estimate optimization opportunity for each problem:
   - current recorded runtime/score from JSON
   - bottleneck class (algorithmic complexity, data structure overhead, I/O, alloc churn, etc.)
   - estimated potential speedup factor range (e.g., 1.2-1.5x, 2-4x, 10x+)
   - confidence (low/med/high)
4) Produce ranked triage output (highest potential savings first).
5) Output BOTH:
   A) CSV file at euler/optimization_triage.csv
      columns: problem_id,current_runtime_or_score,estimated_speedup_range,estimated_time_saved_pct,confidence,bottleneck_class,notes
   B) JSON augmentation file at euler/optimization_triage.json keyed by problem id, suitable to merge into validated.json later.
6) Also provide a short summary in stdout:
   - total problems scanned
   - top 15 highest-priority optimization targets
   - any data quality gaps in validation JSON

Execution constraints:
- Non-interactive completion.
- You may create lightweight helper scripts for parsing, but do not execute solution code.
- Keep edits focused to triage artifacts only.

When finished, print exact paths of created files.
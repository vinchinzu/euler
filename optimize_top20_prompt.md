Task: Optimize the top 20 Euler Rust solutions from optimization triage using controlled A/B testing. You may use sub-agents in parallel.

Working directory: euler/
Primary codebase: rust/solutions/src/bin/pNNN.rs
Triage source: optimization_triage.csv (already generated)

Goal:
- For each of the top 20 ranked problems in optimization_triage.csv:
  1) create an optimized candidate implementation
  2) benchmark baseline vs candidate with identical conditions
  3) accept candidate only if measurably faster and still correct
  4) reject candidate (revert/skip) if not faster or unstable

Hard constraints:
- Never overwrite with slower code.
- Never break correctness.
- Keep each accepted change isolated and auditable.
- Use timeout protection on runs.

Required procedure per problem (A/B gate):
1. Identify problem id from top 20 rows in optimization_triage.csv.
2. Baseline:
   - Build release binary for pNNN
   - Run correctness check against expected answer in data/answers.txt
   - Record baseline timing with at least 5 runs (discard first warmup, compare median)
3. Candidate:
   - Create optimization patch in rust/solutions/src/bin/pNNN.rs
   - Rebuild and re-check correctness
   - Run same timing protocol (>=5 runs, same machine/settings)
4. Decision rule:
   - Accept only if candidate median is >=5% faster and correctness passes
   - Otherwise revert to baseline for that problem
5. Persist per-problem result in machine-readable report.

Parallelism/sub-agents:
- Use sub-agents/workers for multiple problems concurrently, but cap concurrency to avoid noisy benchmarks (e.g., 2-4 workers).
- Ensure benchmark runs are not heavily contended; serialize timing section if needed for fair A/B.

Artifacts to produce:
1) optimization_ab_results.csv with columns:
   problem_id,baseline_ms,candidate_ms,delta_pct,accepted,correctness_pass,notes
2) optimization_ab_results.json with full details (timings arrays, medians, rationale)
3) optimization_applied_summary.md with:
   - accepted optimizations list
   - rejected attempts list and reasons
   - net projected time saved across accepted set
4) Git commits for accepted changes only (one commit per problem or small logical group), clear messages.

Implementation notes:
- Prefer deterministic, low-risk optimizations first (algorithmic wins, data-structure improvements, reduced allocations, u128->u64 where safe, parallelization where embarrassingly parallel).
- Keep code readable and maintainable.
- If benchmark noise is high, increase run count and note confidence.

At end:
- Print concise summary to stdout (accepted/rejected counts, total net speedup).
- Print exact paths of artifacts.
- Then notify completion with:
  openclaw system event --text "Done: Top20 A/B optimization pass complete (see optimization_ab_results.csv)" --mode now

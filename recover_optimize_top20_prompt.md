Continue and recover the interrupted Top-20 optimization A/B run.

Repo root: euler/
Rust repo: euler/rust

Current state:
- Previous run was interrupted and left modified files in rust/solutions/src/bin/*.rs.
- No final A/B artifacts were produced yet.

Your job:
1) Detect all currently modified problem files in rust/solutions/src/bin/pNNN.rs.
2) For each modified problem, run strict A/B evaluation vs baseline:
   - baseline = version at HEAD
   - candidate = current working-tree modified version
   - correctness check against ../data/answers.txt
   - benchmark protocol: warmup + at least 5 timed runs, compare medians
3) Decision rule:
   - accept only if candidate median is >=5% faster and correctness passes
   - otherwise revert that file to HEAD
4) Commit accepted problems only (one commit per problem preferred).
5) Produce artifacts at euler/:
   - optimization_ab_results.csv
   - optimization_ab_results.json
   - optimization_applied_summary.md

Then continue remaining top-20 problems from optimization_triage.csv that have not yet been attempted:
- For each remaining problem: create candidate, run same A/B gate, accept/reject accordingly.
- Keep benchmark fairness (low contention; serialize timing if needed).

Safety constraints:
- Never keep slower code.
- Never keep incorrect code.
- Always use timeout for runs.
- If a problem is too noisy/inconclusive, mark rejected with reason "inconclusive benchmark".

Output requirements:
- Update the same CSV/JSON/summary incrementally after each problem so interruption won’t lose progress.
- Print progress every few problems.
- At end print accepted/rejected counts and net speedup.
- Print exact artifact paths.
- Notify completion with:
  openclaw system event --text "Done: Top20 A/B optimization recovery complete" --mode now

---
name: euler-rust-speed
description: >
  Speed-tune Project Euler Rust binaries (especially 1.5–3s solutions)
  down toward sub-1s. Use when asked to optimize Euler Rust runtimes,
  drop 2s problems under 1s, A/B a rust/solutions/src/bin/pNNN.rs change,
  or when the user runs /euler-rust-speed.
---

# Euler Rust speed

Repo-local workflow for wall-clock cuts on `rust/solutions/src/bin/pNNN.rs`.

Canonical rules: `rust/CLAUDE.md`.
Living log: `optimization_applied_summary.md`.
5900XT snapshot (pre-wave-4): `optimization_status.md`.
A/B log: `optimization_ab_results.csv` / `.json`.
Times: `validated.json` (JSONL at repo root). Answers: `data/answers.txt`.

## Target selection

Prefer problems in **1.5–3.0s** (`validated.json` `time_ms`) that can realistically 2×.

Scan source for, in order:

1. Sequential independent work with **no rayon** (outer `for` over primes / n / pairs).
2. Hot `i128`/`u128` mulmod when `MOD^2 < 2^64` (usually `MOD < 2^32`).
3. `pow_mod(base, i, m)` inside `for i in 0..N` — replace with incremental `acc = acc * base % m`.
4. Naive multiplicative order (multiply until 1) — use `φ(m)` factorization instead.

Skip first: already-`par_iter` solutions, sequential recurrences that only need a closed form you do not have, and Fenwick/heap/Lucy DP with true loop-carried deps.

## Techniques that hit sub-1s

| Pattern | When | Typical gain |
|---------|------|--------------|
| rayon over independent outer index | per-iter work ≳ few µs, balanced or work-stealing | 3–7× on 4c/8t |
| `i128` → `u64` mulmod | `0 ≤ a,b < m` and `m*m` fits `u64` | 2–6× if mulmod is the loop |
| incremental power / φ-order | loop of `pow_mod(2, i, k)` or naive `order()` | 10×+ |
| `rayon::join` of two large independent calls | e.g. `g(N1)`, `g(N2)` | only if both sides are heavy |

## Do not

- Rayon a **1e8-element** loop of a few arithmetic ops (`p410` regressed).
- Rayon **tens of millions** of near-empty outer iterations (`p518` 25M `k` regressed).
- Use `with_min_len` on `RangeInclusive<usize>` — that range is not `IndexedParallelIterator`; use `2..n+1`.
- Merge a candidate without the A/B gate.
- Change output format (stdout = answer only).

## A/B gate (required)

Accept only if **median ≥5% faster** and stdout matches `data/answers.txt`.

```bash
python3 ab_bench.py NNN 1 3    # warmup=1, runs=3 is enough for 2s problems
```

`ab_bench.py` swaps HEAD vs working-tree source, rebuilds, times, then restores the candidate. Append the JSON line to `optimization_ab_results.csv` / `.json` and a row to `optimization_applied_summary.md`.

On reject: `git checkout -- rust/solutions/src/bin/pNNN.rs`.

Do not A/B two problems at once on this machine (16c/32t; times fight).

## Sub-agents

Worktree-isolate children so they cannot clobber an in-progress wave. One problem per agent. Prompt must include: current `time_ms`, expected answer, suggested approach, A/B rule, “edit only `pNNN.rs`”, “do not merge”. Parent copies the file and re-runs A/B before keeping it.

## After a wave

Update `optimization_applied_summary.md` (accepted / rejected / remaining counts). Do not rewrite `optimization_status.md` (frozen 5900XT snapshot).

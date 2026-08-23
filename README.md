# Project Euler Solutions

Solutions to Project Euler problems. **Primary language is Rust**, with C as reference/comparison. Legacy solutions exist in Python, Ruby, C++, Fortran.

## Quick Start

```bash
# Build all Rust solutions
cd rust && cargo build --release

# Run a single solution
./rust/target/release/p308

# Validate all solutions (single entry point)
python rust/gen_status.py

# Benchmark C vs Rust for a problem
cd rust && ./bench.sh 308
```

## Repository Structure

```
rust/                          # PRIMARY - Rust solutions
  solutions/src/bin/pNNN.rs    # One binary per problem
  euler_utils/src/             # Shared library (primes, modular, binomial, crt, etc.)
  Cargo.toml                   # Workspace config
  gen_status.py                # Validation entry point
  bench.sh                     # C vs Rust benchmark
  CLAUDE.md                    # Rust performance guide

c/                             # C reference solutions
  NNN.c
  validate.sh                  # C validation (used by bench.sh)

java/                          # Java reference solutions (read-only)
data/answers.txt               # Master answer key
validated.json                 # Validation cache (JSONL, repo root)
problems/                      # Problem statements
python/                        # Legacy Python solutions
archive/                       # Archived Python validation scripts and notes
```

## Validation

### Single entry point: `rust/gen_status.py`

1. Reads expected answers from `data/answers.txt`
2. Runs compiled Rust binaries with 30s timeout
3. Compares output against expected answers
4. Caches results in `validated.json` (JSONL at repo root, keyed by source MD5)
5. Generates `rust/status.png` grid visualization

```bash
cd rust && cargo build --release
python rust/gen_status.py

# Force re-validation of one problem: delete its line from validated.json
```

Cache line:

```json
{"problem": 308, "hash": "abc123...", "status": "OK", "answer": "1539669807660924", "time_ms": 141}
```

- `hash`: MD5 of source file — a source change triggers re-validation
- `status`: `OK`, `WRONG`, or `TIMEOUT`

### C vs Rust: `rust/bench.sh`

```bash
cd rust && ./bench.sh 308        # Single problem
cd rust && ./bench.sh all        # All problems with both C and Rust sources
```

## Solution Output Format

All solutions print **only the numeric answer** on stdout. Use stderr for debug.

## Optimization

A/B gate: accept only if median is **≥5% faster** and the answer matches `data/answers.txt`.

| File | Role |
|------|------|
| `optimization_applied_summary.md` | Living log: waves, remaining counts, accepts/rejects |
| `optimization_status.md` / `.csv` | Frozen 5900XT re-time (pre-wave-4 snapshot) |
| `optimization_ab_results.csv` / `.json` | Per-problem A/B timings |
| `ab_bench.py` | A/B gate (HEAD vs working tree) |
| `rust/CLAUDE.md` | Performance rules from C→Rust ports |
| `rust/profiles/SUMMARY.md` | Feb 2026 perf classes (times are not current) |
| `.grok/skills/euler-rust-speed/SKILL.md` | Speed playbook (rayon / `u64` / A/B) |

2026-08-22 re-time on Ryzen 9 5900XT (16c/32t): 997 binaries, **1101s → 386s** (2.85×). After waves 4–6 (overlay, not a full re-validate): **~264s**. Remaining overlay: 83 ≥1s, 65 in 500ms–1s. Details in `optimization_applied_summary.md`.

## Key files

| File | Purpose |
|------|---------|
| `data/answers.txt` | Master answer key |
| `rust/gen_status.py` | Validation script |
| `validated.json` | Validation cache |
| `rust/bench.sh` | C vs Rust benchmark |
| `c/validate.sh` | C-only validation (used by `bench.sh`) |

# Project Euler Solutions

## Overview

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
  gen_status.py                # THE validation entry point
  validated.json               # Validation cache (JSONL)
  bench.sh                     # C vs Rust benchmark
  CLAUDE.md                    # Rust-specific dev guide

c/                             # C reference solutions
  NNN.c                        # Source files
  validate.sh                  # C validation script

java/                          # Java reference solutions (read-only)
  pNNN.java                    # Algorithm references

data/
  answers.txt                  # Master answer key (982 problems)

problems/                      # Problem statements
python/                        # Legacy Python solutions
archive/                       # Archived Python validation scripts
```

## Validation

### Single Entry Point: `rust/gen_status.py`

This is the **only** validation script. It:
1. Reads expected answers from `data/answers.txt`
2. Runs compiled Rust binaries with 30s timeout
3. Compares output against expected answers
4. Caches results in `rust/validated.json` (JSONL, keyed by source MD5)
5. Generates `rust/status.png` grid visualization

```bash
# Build first, then validate
cd rust && cargo build --release
python rust/gen_status.py

# Force re-validation: delete the entry from rust/validated.json
```

### Cache Format (`rust/validated.json`)

```json
{"problem": 308, "hash": "abc123...", "status": "OK", "answer": "1539669807660924", "time_ms": 141}
```

- `hash`: MD5 of source file — if source changes, re-validated automatically
- `status`: `OK`, `WRONG`, or `TIMEOUT`

### C vs Rust Benchmark: `rust/bench.sh`

```bash
cd rust && ./bench.sh 308        # Single problem
cd rust && ./bench.sh all        # All problems with both C and Rust sources
```

## Solution Output Format

All solutions output **only the numeric answer** on stdout. No headers, no verbose text. Use stderr for debug output.

## Optimization

Rust wall-clock tuning uses a static triage + A/B gate (accept only if median is ≥5% faster and the answer is correct).

| File | Purpose |
|------|---------|
| `optimization_applied_summary.md` | **Current status**: accepted speedups, rejects, and **partials** to resume |
| `optimization_ab_results.csv` / `.json` | Per-problem A/B timings and decisions |
| `optimization_triage.csv` / `.json` | Ranked optimization opportunity estimates |
| `rust/CLAUDE.md` | Performance rules learned from C→Rust ports |
| `rust/profiles/SUMMARY.md` | Profiling classes for solutions >1s |

**Partials:** wave-2 batches B/C/D were stopped mid-run. Unvalidated candidates may still live under `~/.grok/worktrees/euler-project-euler/subagent-*`. **Do not merge them without re-running the A/B gate** (≥5% faster median + correct answer). See the Partials section in `optimization_applied_summary.md` for the worktree map, WIP tables, and resume recipe.

## Key Files

| File | Purpose |
|------|---------|
| `data/answers.txt` | Master answer key (source of truth) |
| `rust/gen_status.py` | Validation script (single entry point) |
| `rust/validated.json` | Validation cache |
| `rust/bench.sh` | C vs Rust benchmark |
| `rust/CLAUDE.md` | Rust performance guide |
| `c/validate.sh` | C-only validation (used by bench.sh) |
| `optimization_applied_summary.md` | Optimization accepts / rejects / partials |

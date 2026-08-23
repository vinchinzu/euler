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
| `optimization_status.md` / `.csv` | **Current status** (2026-08-22 5900XT re-time): already-optimized vs needs-refactor |
| `optimization_applied_summary.md` | Historical A/B waves: accepted speedups, rejects, and **partials** |
| `optimization_ab_results.csv` / `.json` | Per-problem A/B timings and decisions |
| `optimization_triage.csv` / `.json` | Ranked opportunity estimates (static; pre-wave-2) |
| `rust/CLAUDE.md` | Performance rules learned from C→Rust ports |
| `rust/profiles/SUMMARY.md` | Profiling classes for solutions >1s |
| `.grok/skills/euler-rust-speed/SKILL.md` | 2s→<1s playbook (target scan, rayon/`u64` rules, A/B gate) |

**2026-08-22 re-time** on Ryzen 9 5900XT (16c/32t): 997 binaries, sum **1101s → 386s** (2.85×). See `optimization_status.md` (pre-wave-4 snapshot). After waves 4–6 (overlay, not a full re-validate): **~264s**. **Wave 6** (22 leftovers + first sub-500ms, all ≥2×). **Wave 5** squeezed the 500ms–1s band: 22 one-run+answer (all ≥2.2×). **Wave 4** merged 52 candidates (12 A/B, 40 one-run+answer), including a **p968** correctness fix. **Wave 3** dropped thirteen ~2s binaries under 1s on the old 4c/8t box. A/B gate unchanged (≥5% faster median + correct answer). Remaining: 83 ≥1s, 65 in 500ms–1s, ~314 in 50–500ms (cursory pass in progress).

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
| `optimization_status.md` / `.csv` | 5900XT re-time + already-optimized vs needs-refactor |

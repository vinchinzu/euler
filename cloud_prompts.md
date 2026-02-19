# Cloud Agent Prompts — Remaining Broken Euler Solutions

**Last updated:** 2026-02-19

11 problems originally TIMEOUT. **6 FIXED**, 5 remaining. No algorithmic overlap between the 5.

Read `CLAUDE.md` and `rust/CLAUDE.md` for conventions.

---

## Rules

- Solutions go in `rust/solutions/src/bin/pNNN.rs`
- Output ONLY the numeric answer on stdout
- Must run in <120s (release build)
- NO hardcoded answers — must compute from first principles
- Expected answers are in `data/answers.txt` (has \r\n line endings — strip \r when comparing)
- C/Python references exist but are often WRONG or slow — verify before trusting
- Build: `source ~/.cargo/env && cargo build --release --bin pNNN`
- Run: `timeout 120 ./rust/target/release/pNNN`
- Problem statements: `https://projecteuler.net/problem=NNN`

---

## Execution

```bash
# Build and test
source ~/.cargo/env
cd rust && cargo build --release --bin pNNN 2>&1
timeout 120 ./target/release/pNNN

# Check answer
expected=$(grep "Problem NNN:" ../data/answers.txt | sed 's/.*: //' | tr -d '\r')
actual=$(timeout 120 ./target/release/pNNN)
[ "$actual" = "$expected" ] && echo "OK" || echo "WRONG: got=$actual expected=$expected"

# Incremental validation (all problems, skips unchanged)
cd rust && cargo build --release 2>&1 | tail -3 && python3 gen_status.py
```

---

## Safety

- **Always** use `timeout 120` when running binaries
- Use `ulimit -v 4000000` for memory caps on OOM-risk problems (p774)
- Test with reduced inputs first, scale up gradually
- After stopping agents: `ps aux | grep target/release/p | grep -v grep` and kill stragglers
- Max 3 concurrent subagents

---

## Status

| # | Problem | Status | Risk | Refs | Category |
|---|---------|--------|------|------|----------|
| 1 | p928 | **WRONG** (was TIMEOUT) | CPU hang | C, Py | Combinatorics / generating functions |
| 2 | p968 | **WRONG** (fast) | — | Py | Constrained sums / geometric series |
| 3 | p774 | TIMEOUT | **OOM + CPU** | C, Py | Tensor-train linear algebra |
| 4 | p878 | TIMEOUT | CPU hang | Py | GF(2) polynomial orbit iteration |
| 5 | p954 | TIMEOUT | CPU hang | C, Py | Digit DP / DFS with wide branching |

Previously fixed: p574 (0.09s), p566 (0.24s), p641 (0.09s), p861 (3.8s), p927 (28s), p953 (46s).

---

## Remaining Problems

### p928 — Cribbage Scoring
**Expected:** 81108001093

**State:** Modified with incremental GF approach but gives **wrong answer** (~306M vs 81B). The incremental `extend_gf()` likely has a double-counting or backtracking bug.

**Fix approach:**
1. Revert to original brute-force GF computation at leaves (known correct, just slow)
2. Add pruning: skip branches where remaining cards can't change score
3. The search space is 5^13 ≈ 1.2B leaves — needs DP or aggressive pruning
4. **Safe test:** NRANKS=8 or MAX_COUNT=2

### p968 — Quintic Pair Sums
**Expected:** 885362394

**State:** Rust has partial analytical handling (d-e loops only). The a-b-c loops are still brute-force O(X^3). Currently gives wrong answer (294683487) because the analytical part is buggy.

**Fix approach:**
1. **Port `compute_P_closed_form` from Python** (python/968.py lines 167-270)
2. The `_compute_level` recursive function handles each variable analytically via geometric series and critical-point segmentation
3. Modular arithmetic helpers already exist in the Rust code
4. **Safe test:** `P(2,2,2,2,2,2,2,2,2,2) = 7120`, `P(1,2,3,4,5,6,7,8,9,10) ≡ 799809376 (mod 10^9+7)`

### p774 — Conjunctive Sequences
**Expected:** 459155763

**State:** Tensor-train / MPS approach with Gaussian elimination. Hadamard product multiplies TT ranks multiplicatively — ranks blow up to 100s, causing OOM.

**Fix approach:**
1. Apply `reduce_left` after EVERY operation, not just after hadamard
2. Implement `reduce_right` (bidirectional compression)
3. Consider entirely different algorithm: transfer matrix with zeta/Mobius on divisor lattice
4. **Safe test:** Reduce n from 123 to 10, b to 255 (8 bits), monitor ranks

### p878 — XOR-Equation B
**Expected:** 23707109

**State:** Main loop iterates k=1..10^6. For each k, factors over GF(2)[x], builds generators, iterates orbits. Orbit iteration is unbounded for some k values.

**Fix approach:**
1. Add hard cap on orbit iteration (100K steps)
2. Profile which k values are slowest (batch test k=1..1000 first)
3. Precompute factorizations for k=1..10^6 upfront
4. **Safe test:** Reduce m from 10^6 to 1000, N from 10^17 to 10^8

### p954 — Swap-Divisibility
**Expected:** 736463823

**State:** DFS places digits one at a time for lengths L=1..13. Branching factor ~7 gives 7^13 ≈ 96B nodes for L=13. Way too slow.

**Fix approach:**
1. **DP with compressed state:** Track (position, current_mod_7, set of relevant digit-mod-7 classes) instead of full digit sequence
2. **Analytical counting on bad branches:** When a swap is "bad", remaining digits are free — count analytically
3. Parallelize with rayon over first-digit choices (9 branches for L > 1)
4. **Safe test:** Run with L limited to 1..10

---

## Key Lessons from Previous Fixes

- **Don't trust C/Python references.** Both were wrong/slow for multiple problems.
- **Profile the slow input, not the slow function.** Use `Instant::now()` per-call timing.
- **Measure data structure sizes** — many timeouts stem from unexpected quadratic growth.
- **u128 → u64 narrowing** gives 2-4x in tight mod-arith loops when modulus fits u32.
- **Rayon parallelism** works well when per-iteration work > 1ms and units are independent.
- **Batch + sort + merge** beats one-at-a-time insertion into sorted collections.
- **Always use `timeout`** — multiple machine crashes from unbounded loops.

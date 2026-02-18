# Cloud Agent Prompts — Remaining Broken Euler Solutions

**Last updated:** 2026-02-17

6 problems remain: 5 TIMEOUT + 1 WRONG/TIMEOUT (p861 source changed but still too slow).
3 previously broken problems are now solved: p903 (OK, 178ms), p681 (OK, 14.8s), p735 (OK, 11.6s).

The repo is at the current directory. Read `CLAUDE.md` and `rust/CLAUDE.md` for conventions.

---

## Rules

- Solutions go in `rust/solutions/src/bin/pNNN.rs`
- Output ONLY the numeric answer on stdout
- Must run in <120s (release build)
- NO hardcoded answers — must compute from first principles
- Expected answers are in `data/answers.txt`
- C reference solutions in `c/NNN.c` — WARNING: often WRONG or slow too
- Python references in `python/NNN.py` — also often WRONG or too slow
- Build: `source ~/.cargo/env && cargo build --release --bin pNNN`
- Run: `timeout 120 ./rust/target/release/pNNN`
- Problem statements: visit `https://projecteuler.net/problem=NNN`

---

## Timeout-Safe Execution Strategy

### Build and test a single problem

```bash
# Always source cargo env first (non-login shells may lack it)
source ~/.cargo/env

# Build
cd rust && cargo build --release --bin pNNN 2>&1

# Run with wall-clock timeout (prevents hangs)
timeout 120 ./target/release/pNNN

# Check against expected answer
expected=$(grep "Problem NNN:" ../data/answers.txt | sed 's/.*: //')
actual=$(timeout 120 ./target/release/pNNN)
if [ "$actual" = "$expected" ]; then echo "OK"; else echo "WRONG: got=$actual expected=$expected"; fi
```

### Incremental validation (Rust only)

```bash
source ~/.cargo/env
cd rust && cargo build --release 2>&1 | tail -3
python3 gen_status.py
```

`gen_status.py` uses a 30s timeout per problem by default. It is **incremental**: problems whose source hash hasn't changed since last validation are skipped. Only modified/new solutions are re-tested.

### Single-problem revalidation (bypasses cache)

```bash
# Delete the cached entry so gen_status.py re-tests it
python3 -c "
import json
lines = open('validated.json').readlines()
with open('validated.json','w') as f:
  for l in lines:
    d = json.loads(l)
    if d['problem'] != NNN:
      f.write(l)
"
# Then rebuild and run gen_status.py
cargo build --release --bin pNNN && python3 gen_status.py
```

---

## Subagent Orchestration

### Hard limits

- **Max 3 concurrent subagents** at any time
- Each subagent should handle **one problem** (or one tightly-scoped task)
- Always use `timeout 120` when running compiled binaries in subagents
- After stopping subagents, **check for orphaned processes**:

```bash
ps aux | grep -E 'p[0-9]+_diag|python3 -c|target/release/p[0-9]' | grep -v grep
# Kill any stragglers
kill <pids>
```

### Batching strategy

Work in batches of 2-3 problems at a time:

1. **Batch 1 (best near-term wins):** p574, p641, p902
2. **Batch 2 (harder algorithmic work):** p861, p774
3. **Batch 3 (heaviest):** p566

Within each batch, launch subagents in parallel (up to 3). Wait for all to finish before starting the next batch.

### Subagent task template

Each subagent should:
1. Read the problem statement (web fetch or from `problems/` directory)
2. Read the current Rust source and any C/Python references
3. Identify the algorithmic bottleneck (profile if needed)
4. Implement a faster algorithm
5. Build and test: `source ~/.cargo/env && cd rust && cargo build --release --bin pNNN && timeout 120 ./target/release/pNNN`
6. Verify answer matches `data/answers.txt`
7. Report result back

### Checkpointing

After each batch completes:
1. Run `cd rust && python3 gen_status.py` to update validation cache
2. Update this file's status section below
3. Commit progress if answers are correct

---

## Current Status

| Problem | Status | Expected Answer | Time | Notes |
|---------|--------|-----------------|------|-------|
| p903 | **SOLVED** | 128553191 | 178ms | Fixed closed-form derivation |
| p681 | **SOLVED** | 2611227421428 | 14.8s | Divisor loop optimization |
| p735 | **SOLVED** | 174848216767932 | 11.6s | Summatory transform |
| p574 | TIMEOUT | 5780447552057000454 | >120s | CRT subset enumeration — needs meet-in-middle or pruning |
| p641 | TIMEOUT | 793525366 | >30s | Lucy DP — needs exponent-class mod 6 pruning |
| p902 | TIMEOUT | 343557869 | >30s | Permutation cycle order — needs cycle decomposition math |
| p861 | TIMEOUT | 672623540591 | >120s | Bi-unitary DFS — source recently changed, still too slow |
| p774 | TIMEOUT | 459155763 | >30s | Tensor-train rank blow-up — needs aggressive compression |
| p566 | TIMEOUT | 329569369413585 | >30s | Cake cutting — needs event-driven + periodicity detection |

---

## Problem Details (remaining 6)

### Problem 574 — TIMEOUT (>120s)

**Expected:** 5780447552057000454
**Rust:** `rust/solutions/src/bin/p574.rs` (185 lines)
**C ref:** `c/574.c` (131 lines)

Prime verification via CRT. Gray code subset iteration.

**Assessment:** Current per-prime subset enumeration is still too expensive. Best candidate for near-term win via tighter CRT residue generation reuse and pruning (or meet-in-the-middle on subset contributions).

---

### Problem 641 — TIMEOUT (>30s)

**Expected:** 793525366
**Rust:** `rust/solutions/src/bin/p641.rs` (168 lines)
**C ref:** `c/641.c` (213 lines)

Lucy DP for counting integers with divisor count divisible by 6.

**Assessment:** Recursive search repeatedly does expensive root/power-bound logic. Add memoized state compression and tighter exponent-class pruning; likely requires re-deriving counting recursion around divisor-exponent residues mod 6.

---

### Problem 902 — TIMEOUT (>30s)

**Expected:** 343557869
**Rust:** `rust/solutions/src/bin/p902.rs` (122 lines)
**C ref:** `c/902.c` (197 lines)

Permutation rank sum. Cycle order may be huge — need mathematical shortcut.

**Assessment:** Current code iterates permutation powers until identity (`current == identity`), which is the timeout driver. Replace with cycle-decomposition/order math + rank-sum formula over cycle structure, not explicit orbit traversal.

---

### Problem 861 — TIMEOUT (>120s, C also slow)

**Expected:** 672623540591
**Rust:** `rust/solutions/src/bin/p861.rs` (source recently modified, still too slow)
**C ref:** `c/861.c` (303 lines) — C also >2min

Bi-unitary divisors, Lucy DP + backtracking DFS.

**Assessment:** Source was recently rewritten but still times out (>120s). The backtracking DFS explores too many states. Needs canonical signature caching + cheaper used-prime bookkeeping + deeper final-level counting shortcuts. Consider completely different approach: multiplicative function decomposition or generating function methods.

---

### Problem 774 — TIMEOUT (>30s)

**Expected:** 459155763
**Rust:** `rust/solutions/src/bin/p774.rs` (362 lines)
**C ref:** `c/774.c` (516 lines)

Conjunctive sequences, tensor-train/MPS compression.

**Assessment:** Core risk is rank blow-up in repeated TT add/hadamard/reduce. Add stricter compression strategy and operation ordering to cap ranks aggressively after each transform.

---

### Problem 566 — TIMEOUT (>30s)

**Expected:** 329569369413585
**Rust:** `rust/solutions/src/bin/p566.rs` (399 lines)
**C ref:** `c/566.c` (290 lines)

Cake cutting with algebraic numbers. Complex interval merging.

**Assessment:** Heavy interval/position simulation with repeated O(n^2) style merging and sorting in hot paths. Needs event-driven state transitions + stronger periodicity detection, not more threading. Heaviest problem to fix — save for last.

---

## Solved Problems (for reference)

### Problem 903 — SOLVED (178ms)
Combinatorial sum with factorials and harmonic numbers mod 10^9+7. Fixed by correcting the closed-form derivation.

### Problem 681 — SOLVED (14.8s)
Maximal area quadrilateral via factorization. Fixed by multiplicative aggregation.

### Problem 735 — SOLVED (11.6s)
Divisors of 2n^2 with Mobius function. Fixed with summatory transform approach.

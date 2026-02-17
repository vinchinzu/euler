# Cloud Agent Prompts — Remaining Broken Euler Solutions

13 problems remain: 3 WRONG + 10 TIMEOUT. Work on them sequentially.
The repo is at the current directory. Read `CLAUDE.md` and `rust/CLAUDE.md` for conventions.

**Rules:**
- Solutions go in `rust/solutions/src/bin/pNNN.rs`
- Output ONLY the numeric answer on stdout
- Must run in <120s (release build) — timeout bumped from 30s
- NO hardcoded answers — must compute from first principles
- Expected answers are in `data/answers.txt`
- C reference solutions in `c/NNN.c` — WARNING: often WRONG or slow too
- Python references in `python/NNN.py` — also often WRONG or too slow
- Build: `cd rust && cargo build --release --bin pNNN`
- Run: `./rust/target/release/pNNN`
- Problem statements: visit `https://projecteuler.net/problem=NNN`

**IMPORTANT:** For the WRONG problems, ALL existing references give wrong answers. You need to read the actual problem statement, understand the mathematics, and write a correct solution from scratch.

**Agent notes from first pass:**
- P846/P861/P864: C solutions also slow (C P846: 2.5min, C P864: 1.5min, C P861: >2min). Rust is 1.5-2x slower than C. Need algorithmic improvement, not just optimization.
- P566/P574/P641: Agent was profiling individual slow cases.

---

## Problem 900 — WRONG (all refs wrong)

**Expected:** 646900900
**Current output:** 296202364 (in <2ms)
**Rust:** `rust/solutions/src/bin/p900.rs` (30 lines)
**C ref:** `c/900.c` — ALSO WRONG (same answer)
**Python:** `python/900.py` — ALSO WRONG (same answer)

All three use wrong formula: `((4^N + 2) / 3 - 2^N) mod P` with P=900497239, N=10000. Read projecteuler.net/problem=900 and derive the correct formula.

---

## Problem 903 — WRONG (all refs wrong)

**Expected:** 128553191
**Current output:** 231548875 (~400ms)
**Rust:** `rust/solutions/src/bin/p903.rs` (73 lines)
**C ref:** `c/903.c` — ALSO WRONG (same answer)
**Python:** `python/903.py` — ALSO WRONG, says "known incorrect guess"

Combinatorial sum with factorials and harmonic numbers mod 10^9+7. Read projecteuler.net/problem=903.

---

## Problem 910 — WRONG (all refs wrong)

**Expected:** 547480666
**Current output:** 432948826 (in <2ms)
**Rust:** `rust/solutions/src/bin/p910.rs` (45 lines)
**C ref:** `c/910.c` — ALSO WRONG (same answer)
**Python:** `python/910.py` — ALSO WRONG (same answer)

Combinatory logic / Church numerals. All refs assume power tower of 2 but get wrong answer. Read projecteuler.net/problem=910.

---

## Problem 902 — TIMEOUT (30s)

**Expected:** look up in `data/answers.txt`
**Rust:** `rust/solutions/src/bin/p902.rs` (122 lines)
**C ref:** `c/902.c` (197 lines)

Permutation rank sum. Cycle order may be huge — need mathematical shortcut.

---

## Problem 566 — TIMEOUT (120s)

**Expected:** look up in `data/answers.txt`
**Rust:** `rust/solutions/src/bin/p566.rs` (399 lines)
**C ref:** `c/566.c` (290 lines)

Cake cutting with algebraic numbers. Complex interval merging.

---

## Problem 574 — TIMEOUT (120s)

**Expected:** look up in `data/answers.txt`
**Rust:** `rust/solutions/src/bin/p574.rs` (185 lines)
**C ref:** `c/574.c` (131 lines)

Prime verification via CRT. Gray code subset iteration.

---

## Problem 641 — TIMEOUT (30s)

**Expected:** look up in `data/answers.txt`
**Rust:** `rust/solutions/src/bin/p641.rs` (168 lines)
**C ref:** `c/641.c` (213 lines)

Lucy DP for counting integers with divisor count divisible by 6.

---

## Problem 681 — TIMEOUT (30s)

**Expected:** look up in `data/answers.txt`
**Rust:** `rust/solutions/src/bin/p681.rs` (108 lines)
**C ref:** `c/681.c` (113 lines)

Maximal area quadrilateral via factorization. Triple-nested divisor loop too slow.

---

## Problem 735 — TIMEOUT (30s)

**Expected:** look up in `data/answers.txt`
**Rust:** `rust/solutions/src/bin/p735.rs` (162 lines)
**C ref:** `c/735.c` (143 lines)

Divisors of 2n^2 with Mobius function. Already uses rayon.

---

## Problem 774 — TIMEOUT (30s)

**Expected:** look up in `data/answers.txt`
**Rust:** `rust/solutions/src/bin/p774.rs` (362 lines)
**C ref:** `c/774.c` (516 lines)

Conjunctive sequences, tensor-train/MPS compression.

---

## Problem 846 — TIMEOUT (C also slow: 2.5min)

**Expected:** look up in `data/answers.txt`
**Rust:** `rust/solutions/src/bin/p846.rs` (280 lines)
**C ref:** `c/846.c` (413 lines) — C takes 2.5min

Graph potency. C is also slow, needs algorithmic improvement.

---

## Problem 861 — TIMEOUT (C also slow: >2min)

**Expected:** look up in `data/answers.txt`
**Rust:** `rust/solutions/src/bin/p861.rs` (225 lines)
**C ref:** `c/861.c` (303 lines) — C also >2min

Bi-unitary divisors, Lucy DP + backtracking DFS.



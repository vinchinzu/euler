# Cloud Agent Prompts — Remaining Broken Euler Solutions

**Last updated:** 2026-02-19

11 problems originally TIMEOUT. **9 FIXED**, 2 remaining. No algorithmic overlap between the 2.

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
- Test with reduced inputs first, scale up gradually
- After stopping agents: `ps aux | grep target/release/p | grep -v grep` and kill stragglers
- Max 3 concurrent subagents

---

## Status

| # | Problem | Status | Risk | Refs | Category |
|---|---------|--------|------|------|----------|
| 1 | p968 | **WRONG** (fast) | — | Py | Constrained sums / geometric series |


---

## Remaining Problems


### p968 — Quintic Pair Sums
**Expected:** 885362394

**State:** Rust has partial analytical handling (d-e loops only). The a-b-c loops are still brute-force O(X^3). Currently gives wrong answer (294683487) because the analytical part is buggy.

**Fix approach:**
1. **Port `compute_P_closed_form` from Python** (python/968.py lines 167-270)
2. The `_compute_level` recursive function handles each variable analytically via geometric series and critical-point segmentation
3. Modular arithmetic helpers already exist in the Rust code
4. **Safe test:** `P(2,2,2,2,2,2,2,2,2,2) = 7120`, `P(1,2,3,4,5,6,7,8,9,10) ≡ 799809376 (mod 10^9+7)`

---

## Key Lessons from Previous Fixes

- **Don't trust C/Python references.** Both were wrong/slow for multiple problems.
- **Profile the slow input, not the slow function.** Use `Instant::now()` per-call timing.
- **Measure data structure sizes** — many timeouts stem from unexpected quadratic growth.
- **u128 → u64 narrowing** gives 2-4x in tight mod-arith loops when modulus fits u32.
- **Rayon parallelism** works well when per-iteration work > 1ms and units are independent.
- **Batch + sort + merge** beats one-at-a-time insertion into sorted collections.
- **Always use `timeout`** — multiple machine crashes from unbounded loops.

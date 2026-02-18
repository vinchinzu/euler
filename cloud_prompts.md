# Cloud Agent Prompts — Remaining Broken Euler Solutions

**Last updated:** 2026-02-18

17 problems remain: 11 TIMEOUT + 6 WRONG.

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
source ~/.cargo/env
cd rust && cargo build --release --bin pNNN 2>&1
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

`gen_status.py` uses a 30s timeout per problem by default. It is **incremental**: problems whose source hash hasn't changed since last validation are skipped. Only modified/new solutions are re-tested. The validation cache is at `validated.json` (project root).

### Single-problem revalidation (bypasses cache)

```bash
# Delete the cached entry so gen_status.py re-tests it
python3 -c "
import json
lines = open('../validated.json').readlines()
with open('../validated.json','w') as f:
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

### Subagent task template

Each subagent should:
1. Read the problem statement (web fetch or from `problems/` directory)
2. Read the current Rust source and any C/Python references
3. Identify the algorithmic bottleneck (profile if needed)
4. Implement a faster/correct algorithm
5. Build and test: `source ~/.cargo/env && cd rust && cargo build --release --bin pNNN && timeout 120 ./target/release/pNNN`
6. Verify answer matches `data/answers.txt`
7. Report result back

---

## TIMEOUT Problems (11)

These compile and run but exceed the time limit. Need algorithmic improvements.

| # | Problem | Time | Refs | Notes |
|---|---------|------|------|-------|
| 1 | p566 | 30s | C, Py | Cake cutting — needs event-driven + periodicity detection |
| 2 | p574 | 120s | C, Py | CRT subset enumeration — needs meet-in-middle or pruning |
| 3 | p641 | 30s | C, Py | Lucy DP — needs exponent-class mod 6 pruning |
| 4 | p774 | 30s | C, Py | Tensor-train rank blow-up — needs aggressive compression |
| 5 | p861 | 30s | C, Py | Bi-unitary DFS — needs signature caching or different approach |
| 6 | p878 | 30s | Py | |
| 7 | p927 | 30s | C, Py | |
| 8 | p928 | 30s | C, Py | |
| 9 | p953 | 30s | C, Py | |
| 10 | p954 | 30s | C, Py | |
| 11 | p968 | 30s | Py | |

---

## WRONG Problems (6)

These produce output but it doesn't match. Need debugging.

| # | Problem | Got | Expected | Time | Refs |
|---|---------|-----|----------|------|------|
| 1 | p870 | 118246.0000000000 | 229.9129353234 | 5.1s | Py |
| 2 | p933 | *(empty after 19s)* | 5707485980743099 | 19s | Py |
| 3 | p934 | 292137809722227094 | 292137809490441370 | 655ms | C, Py |
| 4 | p937 | 866599922 | 792169346 | 319ms | C, Py |
| 5 | p939 | 1149840452 | 246776732 | 54ms | C, Py |
| 6 | p947 | 235556258 | 213731313 | 6s | Py |

---

## Previously Solved This Session (25 problems)

- p780 (2.2s), p798 (1.9s), p889 (~1ms), p890 (2.9s), p894 (~4ms)
- p895 (0.19s), p896 (0.04s), p902 (<10s, fixed sigma bug), p904 (0.29s), p908 (1.1s)
- p911 (63ms), p920 (55ms), p922 (0.11s), p923 (130ms), p924 (0.47s)
- p925 (OK), p935 (29ms), p941 (8.3s), p943 (OK), p949 (2.1s)
- p950 (14ms), p958 (~14s), p961 (0.5s), p964 (8ms), p970 (~2ms)

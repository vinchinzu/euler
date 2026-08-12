# Optimization Applied Summary

Updated: 2026-08-11 (wave 2 partial wrap-up)

A/B gate: accept only if median is **≥5% faster** and answer matches `data/answers.txt`.
Artifacts: `optimization_ab_results.csv`, `optimization_ab_results.json`, `optimization_triage.csv`.

## Accepted (20 total)

| Problem | Baseline ms | Candidate ms | Speedup | Saved ms | Notes |
|--------:|------------:|-------------:|--------:|---------:|-------|
| p910 | 25310.4 | 7158.6 | 3.54× | 18152 | speedup=3.536x |
| p937 | 14406.0 | 4309.6 | 3.34× | 10096 | speedup=3.343x |
| p941 | 14353.0 | 5205.0 | 2.76× | 9148 | speedup=2.758x |
| p534 | 10197.7 | 1850.0 | 5.51× | 8348 | rayon over independent k=0..13 (DP + DFS) |
| p445 | 11426.8 | 4741.7 | 2.41× | 6685 | cache cur_pow + delta p^e; egcd mod_inv |
| p946 | 10516.8 | 4311.6 | 2.44× | 6205 | i128→i32; specialize coeff=1; floor_div fast path |
| p558 | 6937.8 | 1175.1 | 5.90× | 5763 | speedup=5.904x |
| p657 | 5074.4 | 411.4 | 12.33× | 4663 | rayon parallel pow[t]^(N+1) over 10M; i128→u64 |
| p785 | 5569.6 | 942.6 | 5.91× | 4627 | speedup=5.908x |
| p774 | 15887.1 | 11351.1 | 1.40× | 4536 | i32 cores+mat; euclid modinv; ptr GE; mat reuse |
| p932 | 4637.5 | 135.1 | 34.33× | 4502 | speedup=34.337x |
| p557 | 5290.9 | 889.7 | 5.95× | 4401 | speedup=5.947x |
| p421 | 5947.7 | 2053.9 | 2.90× | 3894 | speedup=2.896x |
| p971 | 5525.7 | 1980.5 | 2.79× | 3545 | speedup=2.79x |
| p650 | 4715.4 | 1174.0 | 4.02× | 3541 | i128→u64 in power() / D(n) product |
| p507 | 6142.3 | 2707.8 | 2.27× | 3434 | speedup=2.268x |
| p873 | 5428.0 | 2472.1 | 2.20× | 2956 | linear-sieve inv table; i128→u64 mul |
| p540 | 7063.2 | 4974.3 | 1.42× | 2089 | speedup=1.420x |
| p962 | 8877.1 | 7186.4 | 1.24× | 1691 | speedup=1.235x |
| p693 | 6877.7 | 5281.6 | 1.30× | 1596 | speedup=1.302x |

**Net median wall-clock saved (sum of per-problem medians): ~109.9s**

### Wave 1 (merged earlier)

p421, p507, p540, p557, p558, p693, p785, p910, p932, p937, p941, p962, p971  
(~73s of the total above)

### Wave 2 (merged this session)

| Problem | Speedup | Technique |
|--------:|--------:|-----------|
| p657 | 12.3× | rayon over 10M modpows + u64 |
| p534 | 5.5× | rayon over independent k |
| p650 | 4.0× | i128→u64 hot path |
| p946 | 2.4× | i128→i32 + floor_div |
| p445 | 2.4× | power cache + egcd inv |
| p873 | 2.2× | inverse table + u64 mul |
| p774 | 1.4× | i32 mat/cores, buffer reuse |

Wave 2 net: **~36.9s**. Smoke-checked correct on merge.

Wave 2 rejected: **p606** (already pure u64; rayon attempts no gain).

---

## Partials (not merged — resume here)

Wave 2 spawned four worktree-isolated subagents. **Batch A finished** and was applied
(plus p774/p946 from D). Batches **B**, **C**, and residual **D** were stopped mid-run.
Candidate sources still live in worktrees but **were not A/B-gated**. **Do not merge
without re-running the A/B gate** (≥5% faster median + correct answer vs `data/answers.txt`).

### Worktree map

Base: `~/.grok/worktrees/euler-project-euler/`

| Batch | Subagent id | Status | Problems |
|-------|-------------|--------|----------|
| A | `subagent-019ff34b-e55d-7e53-8074-bee5dc72cf88` | **Done** — merged | 445, 534, 650, 657, 873 (+ rejected 606) |
| B | `subagent-019ff34b-e55e-7d00-9311-8150cfbb4921` | **Partial** — unvalidated WIP | 448, 513, 543, 559, 715 |
| C | `subagent-019ff34b-e55e-7d00-9311-816f01d2535a` | **Partial** — unvalidated WIP | 415, 427, 536 (391, 681 untouched) |
| D | `subagent-019ff34b-e55e-7d00-9311-817bfa5b6d1a` | **Partial** — 774/946 merged from here; rest WIP | 735, 958 (+846 no diff) |

Also: `/tmp/euler_opt/cands/` has snapshot copies of p735, p774, p846, p946, p958
from mid-batch D (may be stale vs worktree).

Batch results JSONL (present only for finished A/B runs):
- `/tmp/euler_opt/batch_a_results.jsonl` — complete
- `/tmp/euler_opt/batch_d_results.jsonl` — partial (774, 946 only)
- `batch_b_results.jsonl` / `batch_c_results.jsonl` — **missing** (agents stopped early)

### Batch B — candidate WIP (needs A/B)

No `batch_b_results.jsonl` (agent stopped before A/B). All five have source diffs:

| Problem | Worktree diff | Triage estimate | Intended approach (from triage) |
|--------:|:-------------:|-----------------|----------------------------------|
| p448 | yes (~77+/41−) | 1.5–2× high | FxHashMap; drop dyn Fn; u64 mod arith |
| p513 | yes (~436+/141−) | 4–10× med | Port C Mobius / O(√N) pair approach from `c/513.c` |
| p543 | yes (~105+/38−) | 1.5–2× med | Sort queries; single linear prime scan |
| p559 | yes (~51+/34−) | 2–4× high | DP / convolution or rayon over k |
| p715 | yes (~91+/45−) | 1.5–2× high | `usize→u32` for `ff`; parallel `big[i]` fill |

### Batch C — candidate WIP (needs A/B)

No `batch_c_results.jsonl`. Three of five have source diffs; two untouched:

| Problem | Worktree diff | Triage estimate | Intended approach |
|--------:|:-------------:|-----------------|-------------------|
| p391 | **no** (still baseline) | 2–4× high | Thread/cache tuning vs C static arrays |
| p415 | yes (~125+/53−) | 1.5–2× med | Fuse Lucy passes; incremental `pow(2,g)` |
| p427 | yes (~134+/43−) | 1.5–2× med | Parallel `fk` precompute; sequential delta |
| p536 | yes (~130+/17−) | 2–4× med | Rayon root-level prime iteration |
| p681 | **no** (still baseline) | 1.5–2× med | Rebalance rayon for highly-composite K |

### Batch D — residual WIP (needs A/B)

Merged already (A/B passed): **p774**, **p946**. Remaining unvalidated:

| Problem | Worktree diff | Triage estimate | Intended approach |
|--------:|:-------------:|-----------------|-------------------|
| p735 | yes (~17+/27−) | 1.5–2× med | CHUNK tune; fuse inner loops; isqrt |
| p846 | **no** (snapshot only in `/tmp/euler_opt/cands/`) | 1.5–2× low | Already parallel DFS; work decomposition |
| p958 | yes (~34+/8−) | 2–4× low | Top-level rayon; memoize BFS/DFS |

### Resume recipe (per partial problem)

```bash
# 1. Copy candidate from worktree (example: batch B p513)
WT=~/.grok/worktrees/euler-project-euler/subagent-019ff34b-e55e-7d00-9311-8150cfbb4921
P=513
cp "$WT/rust/solutions/src/bin/p${P}.rs" rust/solutions/src/bin/p${P}.rs

# 2. Build + A/B (keep HEAD baseline available via git)
cd rust && cargo build --release --bin p${P}
# baseline: git stash or checkout HEAD file, rebuild, time ≥5 runs (median)
# candidate: restore candidate, rebuild, same protocol
# accept only if median ≥5% faster AND stdout matches data/answers.txt

# 3. If reject:
git checkout -- rust/solutions/src/bin/p${P}.rs

# 4. Append result to optimization_ab_results.csv / .json
```

Helper (if still present): `/tmp/euler_opt/ab_bench.py NNN EXPECTED --runs 5 --timeout 120`

### Cleanup (optional)

When partials are either merged or abandoned:

```bash
# remove worktrees after extracting any wanted candidates
rm -rf ~/.grok/worktrees/euler-project-euler/subagent-019ff34b-*
rm -rf /tmp/euler_opt
```

---

## Rejected / Neutral

- p782: baseline=18761ms cand=18055ms — within noise (1.039×)
- p829: baseline=12319ms cand=17462ms — regression (0.705×)
- p847: baseline=11668ms cand=11238ms — within noise (1.038×)
- p606: baseline=8805ms cand=8805ms — already pure u64; rayon no gain

---

## Related files

| File | Role |
|------|------|
| `optimization_triage.csv` / `.json` | Ranked opportunity estimates (static; pre-wave-2 times) |
| `optimization_ab_results.csv` / `.json` | Every A/B decision with timings |
| `optimize_top20_prompt.md` | Wave-1 agent prompt template |
| `triage_optimization_prompt.md` | How triage was generated |
| `rust/CLAUDE.md` | Performance rules (HashMap→Vec, u128→u64, rayon, etc.) |
| `rust/profiles/SUMMARY.md` | perf classification for solutions >1s |

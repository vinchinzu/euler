# Optimization Applied Summary

Updated: 2026-08-22 (5900XT re-time; Wave 6 500ms leftovers + first sub-500ms)

A/B gate: accept only if median is **≥5% faster** and answer matches `data/answers.txt`.
Artifacts: `optimization_ab_results.csv`, `optimization_ab_results.json`, `optimization_triage.csv`.
Playbook: `.grok/skills/euler-rust-speed/SKILL.md`.
**Current queue:** `optimization_status.md` (re-timed on Ryzen 9 5900XT, 16c/32t).

## 2026-08-22 re-time (this machine)

Clean fat-LTO rebuild + sequential wall-clock of all 997 `pNNN` binaries. `RAYON_NUM_THREADS=32`. Answers vs `data/answers.txt`.

| | Old cache (2026-07-28) | 5900XT |
|---|---:|---:|
| Sum of OK times | 1101.1s | 386.1s (2.85×) |
| Median | 73 ms | 41 ms |
| ≥1s | 258 | 118 |
| ≥1.5s | 214 | 73 |
| ≥6s | 49 | 3 |
| ≥10s | 6 | 0 |

**p968 is fixed** this wave (digit-DP over 3^10 carries; prints `885362394` in 69 ms). The old closed-form was algorithmically wrong (`P(2,…,2)` ≠ 7120); Python `compute_P_closed_form` has the same bug.

`optimization_status.md` is the **pre-wave-4** 5900XT snapshot. Wave 4 merged 52 speed/correctness candidates (12 full A/B, 40 one-run + answer).

---

## Wave 4 — 5900XT (this session)

Worktree-isolated subagents, one problem each. Parent copied candidates, smoke-checked stdout vs `data/answers.txt`, and A/B-gated modest gains (`python3 ab_bench.py NNN 1 3`). Huge wins (typically 3–20× with matching answers) recorded as one-run parent verify vs the 5900XT status baseline.

**A/B accepted (12)** — median ≥5%, answer match:

| Problem | Baseline ms | Candidate ms | Speedup | Notes |
|--------:|------------:|-------------:|--------:|-------|
| p611 | 1460 | 543 | 2.69× | Lucy i_max split + rayon DFS frames |
| p815 | 945 | 593 | 1.59× | flat `f64` cache, not `Vec<Vec>` |
| p465 | 1499 | 960 | 1.56× | u64 mulmod + join Lucy prefixes |
| p977 | 872 | 576 | 1.52× | u64 mulmod (drop u128) |
| p797 | 2866 | 2046 | 1.40× | incremental 2^i + egcd inv |
| p637 | 1834 | 1364 | 1.34× | `rayon::join` compute_f(10)/(3) |
| p972 | 1782 | 1367 | 1.30× | packed circle keys + diameter Vec |
| p655 | 2332 | 1841 | 1.27× | split-index palindrome DP |
| p738 | 813 | 659 | 1.24× | FxHashMap memo |
| p654 | 1319 | 1129 | 1.17× | u64 NTT mulmod |
| p468 | 5144 | 4579 | 1.12× | deferred small-B sum |
| p433 | 1874 | 1698 | 1.10× | iterative extgcd + rayon g |

**One-run + answer (40)** — parent smoke vs 5900XT baseline; all stdout match `data/answers.txt`:

| P | was ms | now ms | × | What changed |
|--:|-------:|-------:|--:|---|
| 968 | WRONG | 69 | — | digit DP (was closed-form overcount) |
| 314 | 3159 | 13 | 243× | DAG DP + Dinkelbach |
| 867 | 3071 | 35 | 88× | conflict-bitmask tiling DP |
| 154 | 3558 | 79 | 45× | rayon a-stripes + AVX2 |
| 799 | 1240 | 87 | 14× | AP sieve + rayon candidates |
| 538 | 1610 | 61 | 26× | coord-compressed Brahmagupta |
| 947 | 1584 | 62 | 26× | Vec period cache |
| 416 | 1327 | 114 | 12× | join M1/M2 mat_pow |
| 238 | 3936 | 123 | 32× | arithmetic digits + bitset |
| 414 | 2619 | 132 | 20× | closed-form Kaprekar |
| 592 | 4141 | 182 | 23× | rayon 27 f_vals |
| 701 | 1635 | 187 | 8.7× | packed u64 row DP |
| 708 | 1831 | 191 | 9.6× | memo floor quotients |
| 451 | 994 | 196 | 5.1× | drop i128 CRT |
| 216 | 1321 | 204 | 6.5× | u64 Tonelli + segmented sieve |
| 864 | 1801 | 229 | 7.9× | join Part A/B |
| 925 | 3292 | 265 | 12× | rayon digit DFS |
| 536 | 2570 | 266 | 9.7× | rayon CRT leaves |
| 362 | 1134 | 367 | 3.1× | drop HashMap qmap |
| 427 | 1958 | 367 | 5.3× | rayon fk |
| 437 | 2050 | 426 | 4.8× | u64 fib_pair + rayon primes |
| 829 | 7407 | 596 | 12× | rayon n=2..31 (not rejected prune) |
| 483 | 2518 | 658 | 3.6× | packed partition cache |
| 378 | 5367 | 726 | 7.4× | i32 Fenwick; drop SPF |
| 769 | 3120 | 713 | 4.4× | rayon g + isqrt |
| 559 | 5518 | 876 | 6.3× | rayon k + NTT inverse |
| 681 | 2681 | 927 | 2.9× | flatten heavy K |
| 411 | 1482 | 950 | 1.6× | u64 + rayon k LIS |
| 614 | 9153 | 975 | 9.4× | dest-chunk; no temp merge |
| 461 | 4092 | 1016 | 4.0× | par_sort_unstable |
| 715 | 3747 | 1108 | 3.4× | Vec\<u32\> SPF |
| 379 | 3215 | 1138 | 2.8× | drop nested rayon |
| 578 | 2491 | 1180 | 2.1× | FxHashMap + rayon DFS |
| 543 | 2348 | 1248 | 1.9× | one prefix π |
| 415 | 3215 | 1450 | 2.2× | incremental 2^g; fused Lucy |
| 994 | 2951 | 2181 | 1.4× | drop tiny Du Jiao par |
| 890 | 1868 | 578 | 3.2× | parity convolution |
| 447 | 1154 | 516 | 2.2× | u64 hyperbola sigma |
| 691 | 1140 | 495 | 2.3× | SA-IS |
| 639 | 1363 | 395 | 3.5× | u64 mulmod + rayon k |

Reverted (wrong or no gain): **p501** (i64 `p³` overflow under rayon), **p585** (one-run slower), **p291** (~3%, below 5% gate).

≥6s on the re-time snapshot: p614, p829, p846. After wave 4, p614 and p829 are sub-1s; **p846** still ~7.4s (already seed-level rayon).

---

## Wave 5 — 500ms–1s band (this session)

Worktree-isolated subagents, one problem each. Parent copied candidates and one-run smoke-checked stdout vs `data/answers.txt` (`RAYON_NUM_THREADS=32`, fat LTO). All 22 were ≥2.2× (most 7–100×), so they were **not** A/B-gated — same rule as wave-4 one-run wins. Sub-50 ms binaries were still left alone.

**One-run + answer (22)** — parent smoke vs 5900XT `validated.json` baseline; all stdout match `data/answers.txt`:

| P | was ms | now ms | × | What changed |
|--:|-------:|-------:|--:|---|
| 322 | 783 | 1 | 805× | residue count `(val + j·5^nd) & K == 0` |
| 542 | 846 | 6 | 137× | pass `s_low`/`s_high`; `rayon::join` D&C |
| 921 | 704 | 9 | 76× | incremental `5^{F_i}`; u64 mulmod; rayon i-chunks |
| 818 | 776 | 13 | 59× | u128 SET masks; rayon first extra-set index |
| 279 | 783 | 18 | 43× | rayon 4 coprime families; binary gcd |
| 195 | 893 | 21 | 42× | integer inradius; striped rayon `n` |
| 630 | 708 | 21 | 33× | packed (slope, intercept); `par_sort_unstable` |
| 494 | 809 | 25 | 32× | rayon specials; stack helper; `bool` vec |
| 908 | 867 | 41 | 21× | thread-local `b[]` + rayon moduli |
| 781 | 934 | 44 | 21× | NTT FPS inverse of `(1+s)` |
| 553 | 640 | 42 | 15× | 3-mod NTT poly mul/inv/exp |
| 659 | 678 | 46 | 15× | u64; `sqrt(-1)` shortcut; 2^64 inverse div |
| 869 | 871 | 51 | 17× | bit-packed segmented sieve; LSB histograms |
| 373 | 722 | 51 | 14× | closed-form `N(r)` from 1-mod-4 exponents |
| 397 | 771 | 63 | 12× | rayon over independent `k` |
| 548 | 574 | 76 | 7.5× | Pollard factor; FxHashMap `g` |
| 522 | 798 | 112 | 7.2× | rayon over 12M independent `mod_pow` |
| 214 | 658 | 138 | 4.8× | linear-sieve φ; even-only chain |
| 484 | 780 | 172 | 4.5× | rayon DFS; i64 walk; floor-block large p |
| 245 | 800 | 241 | 3.3× | u64 Tonelli; local SPRP; rayon two-prime |
| 446 | 710 | 295 | 2.4× | u64; specialized `sqrt(-1)`; inverse div |
| 636 | 592 | 267 | 2.2× | u64 mulmod; skip profiles with `dp[1]=0` |

No reverts this wave.

Still in 500ms–1s after wave 5 (untouched leftovers, skip-first Lucy/Fenwick, or already-opt): p766, p709, p642, p502, p823, p354, p625, p643, p311, p408, p733, p644, p699, p763, p780, … plus wave-4 landings still in-band (p465, p411, p614, p681, …).

Wave 5 net (sum of parent one-run saves): **~14.9s**.

---

## Wave 6 — leftovers + first sub-500ms (this session)

Worktree-isolated subagents, one problem each. Parent copied candidates and one-run smoke-checked stdout vs `data/answers.txt` (`RAYON_NUM_THREADS=32`, fat LTO). All 22 were ≥2× (most 5–100×). Includes the first cursory sub-500ms pass (50ms→24ms is in-scope). p501/p585/p291 were wave-4/prior reverts; this wave used different algorithms (i128 `p³`, Dirichlet grouping, segmented Tonelli).

**One-run + answer (22)** — parent smoke vs 5900XT `validated.json` baseline; all stdout match `data/answers.txt`:

| P | was ms | now ms | × | What changed |
|--:|-------:|-------:|--:|---|
| 961 | 352 | 1 | 352× | closed form `W(10^{2m})=(100^m−(−8)^m)/6` |
| 699 | 548 | 5 | 110× | 2-3-5 kernels; rayon seeds; Pollard factor |
| 526 | 620 | 16 | 39× | residue families + SPRP; drop √N sieve |
| 311 | 641 | 21 | 31× | Dirichlet last-prime; segmented bit sieve |
| 842 | 311 | 12 | 26× | packed even-n groups; rayon `n=2k` |
| 644 | 596 | 28 | 21× | lattice `(a,b√2)` buckets; drop heap/HashMap |
| 266 | 515 | 32 | 16× | doubling MITM; par two-pointer |
| 763 | 614 | 39 | 16× | circular k-buffers + AVX2 stencil |
| 780 | 632 | 62 | 10× | Vec D-cache; rayon v-stripes |
| 823 | 775 | 82 | 9.5× | linked factor pool; jump to `m=10^16` |
| 585 | 2248 | 245 | 9.2× | Dirichlet `fp` + rayon n (prior revert was slower) |
| 354 | 737 | 87 | 8.5× | exact `N=Lmax²/3`; rayon `(p,q)` |
| 766 | 910 | 108 | 8.4× | 30-bit masks; lock-free packed visited |
| 709 | 862 | 108 | 8.0× | NTT inverse of `cos` (Euler zigzag EGF) |
| 292 | 565 | 86 | 6.6× | packed OA map; 180° symmetry |
| 733 | 647 | 98 | 6.6× | interleaved u32 Fenwick; no k=4 trees |
| 789 | 331 | 53 | 6.2× | bound-256 once; Vec products not HashMap |
| 502 | 773 | 139 | 5.6× | u64 mulmod; join independent (w,h) |
| 353 | 321 | 64 | 5.0× | rayon radii; dense CSR; SPF two-squares |
| 291 | 947 | 236 | 4.0× | rayon q≡1 mod 4; u64 Tonelli (prior ~3%) |
| 501 | 1540 | 604 | 2.5× | rayon p; i128 `p³` (prior overflow revert) |
| 408 | 574 | 284 | 2.0× | u32 fact tables; join triples |

No reverts this wave.

Wave 6 net (sum of parent one-run saves): **~13.6s**.

Estimated remaining after waves 4–6 (overlay on 5900XT re-time, not a full re-validate): **~264s** total. Still **83 ≥1s**, **65 in 500ms–1s**, **116 in 200–500ms**, **198 in 50–200ms**. Named 500ms leftovers still untouched: p642, p625, p643 (Lucy/totient sequential). Next: remaining ≥1s playbook + cursory 50–500ms.

---

## Accepted (133 total; 37 prior + 52 wave 4 + 22 wave 5 + 22 wave 6)

### Wave 3 — 2s band (this session)

Session timings: `validated.json` baseline vs 2–3 release runs of the candidate (same box, 4c/8t). All answers match `data/answers.txt`.

| Problem | Baseline ms | Candidate ms | Speedup | Saved ms | Sub-1s | Notes |
|--------:|------------:|-------------:|--------:|---------:|:------:|-------|
| p455 | 3029 | 468 | 6.47× | 2561 | yes | rayon over n=2..1e6; u64 pow_mod (K=1e9) |
| p531 | 2445 | 368 | 6.64× | 2077 | yes | rayon over n; iterative ext_gcd; drop i128 |
| p404 | 2694 | 470 | 5.73× | 2224 | yes | rayon over both nn cases |
| p628 | 2192 | 415 | 5.28× | 1777 | yes | i128→u64 factorial pass (M² < 2^64) |
| p784 | 2807 | 567 | 4.95× | 2240 | yes | rayon over p; Vec SPF (was `static mut`) |
| p589 | 2464 | 617 | 4.00× | 1847 | yes | rayon over independent (m,n) linear systems |
| p741 | 2803 | 716 | 3.92× | 2087 | yes | u64 mulmod + `rayon::join` of g(N1)/g(N2) |
| p486 | 2313 | 114 | 20.29× | 2199 | yes | φ-order of 2 mod K; incremental 2^k not pow_mod |
| p743 | 2305 | 1619 | 1.42× | 686 | no | u64 mulmod + unchecked inv; recurrence still serial |
| p747 | 1553 | 1119 | 1.39× | 434 | no | rayon over a + u64 ncr |
| p757 | 2639 | 2075 | 1.27× | 564 | no | parallel generate + `par_sort_unstable` |
| p675 | 1767 | 1620 | 1.09× | 147 | no | i128→u64 in S(i!) update |
| p752 | 2744 | 237 | 11.58× | 2507 | yes | i64 mat_mul; SPF factor p²−1; rayon primes |
| p263 | 2307 | 90 | 25.63× | 2217 | yes | local SPRP 2/7/61; wheel; rayon chunks |
| p688 | 2595 | 433 | 6.00× | 2162 | yes | rayon over k-chunks; u64 mulmod (141M divs) |
| p586 | 2458 | 316 | 7.78× | 2142 | yes | k=1/k=2 leaf counts; odd sieve; `rayon::join` |
| p544 | 1975 | 728 | 2.71× | 1247 | yes | no HashMap clone; i32 stack DP; FxHashMap |

Wave 3 net (sum of medians): **~29.2s**. Thirteen problems crossed under 1s.

Wave 3 rejected (reverted):

- **p518** — rayon over 25M `k` (most empty): 2873ms → 3811ms
- **p410** — rayon over 1e8 cheap `j` after a memory-bound sieve: 2747ms → 3132ms

Still in the ~2s band: p221 (already rayon).

Wave-3 follow-up agents — **copied into the working tree after parent verify**:

| Problem | Subagent | Result |
|--------:|----------|--------|
| p752 | `019ff720-9466-7a40-929a-f16e02545fec` | 2744→237 ms, answer OK |
| p263 | `019ff720-9466-7a40-929a-f170bf1ae781` | 2307→90 ms, answer OK |

## Accepted waves 1–2 (20 total)

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

**Waves 1–2 net (sum of per-problem medians): ~109.9s.** Combined with wave 3: **~139.1s**.

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
- p518: baseline=2873ms cand=3811ms — rayon 25M tiny-k loop (wave 3)
- p410: baseline=2747ms cand=3132ms — rayon 1e8 cheap iterations (wave 3)

---

## Related files

| File | Role |
|------|------|
| `optimization_status.md` / `.csv` | 2026-08-22 5900XT re-time + already-optimized vs needs-refactor |
| `optimization_triage.csv` / `.json` | Ranked opportunity estimates (static; pre-wave-2 times) |
| `optimization_ab_results.csv` / `.json` | Every A/B decision with timings |
| `optimize_top20_prompt.md` | Wave-1 agent prompt template |
| `triage_optimization_prompt.md` | How triage was generated |
| `rust/CLAUDE.md` | Performance rules (HashMap→Vec, u128→u64, rayon, etc.) |
| `rust/profiles/SUMMARY.md` | perf classification for solutions >1s |
| `.grok/skills/euler-rust-speed/SKILL.md` | 2s→<1s playbook |

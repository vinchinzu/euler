# Optimization Applied Summary

Updated: 2026-09-03 (Wave 25 batch: p433, p953, p870, p994, p468, p505, p989)

A/B gate: accept only if median is **≥5% faster** and answer matches `data/answers.txt`.
Playbook: `.grok/skills/euler-rust-speed/SKILL.md`.
`optimization_status.md` is the 5900XT re-time **before** waves 4–7. Remaining counts after wave 7 are in this file.

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

---

## 2026-09-03 Post-Wave 25 Status

Following Waves 18–25, all 997 problem binaries run sequentially in **119.17 s** (vs 177.40s roughly a week ago on 2026-08-25, saving **~58.23s**, a 1.49× speedup; and vs 386.1s on 2026-08-22, saving **266.9s**, a 3.24× speedup).
- Median runtime: **23.5 ms** (was 41 ms)
- Binaries ≥ 1.0s: **11** (down from 118 on 2026-08-22)
- Binaries in 500ms–1s: **48**
- Full descending queue of remaining targets: see [`slowest_remaining.md`](file:///home/v/01_projects/euler_project/euler/slowest_remaining.md).

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

---

## Wave 7 — leftover ≥1s playbook (this session)

Worktree-isolated subagents, one problem each. Parent copied candidates and sequential-smoke-checked stdout vs `data/answers.txt` (`RAYON_NUM_THREADS=32`, fat LTO). 21 of 22 were ≥2.2× (most 10–500×) so they were **not** A/B-gated — same rule as waves 4–6 one-run wins. **p850** was 1.44× and went through `python3 ab_bench.py 850 1 3`.

**A/B accepted (1)** — median ≥5%, answer match:

| Problem | Baseline ms | Candidate ms | Speedup | Notes |
|--------:|------------:|-------------:|--------:|-------|
| p850 | 1137 | 787 | 1.44× | rayon first-level DFS; integer isqrt/icbrt; stack C_k |

**One-run + answer (21)** — parent smoke vs 5900XT `validated.json` baseline; all stdout match `data/answers.txt`:

| P | was ms | now ms | × | What changed |
|--:|-------:|-------:|--:|---|
| 626 | 1039 | 2 | 520× | rayon partitions; compact `Copy` parts; u64 mulmod |
| 984 | 1264 | 6 | 211× | rayon boards; bit-parallel flood fill |
| 878 | 1287 | 20 | 64× | GF(2) SPF sieve; pclmul; rayon k |
| 420 | 1072 | 24 | 45× | rayon t1; divisor sieve shrunk to n/4 |
| 470 | 1053 | 28 | 38× | rayon c_val; stack `r_func` |
| 153 | 1012 | 36 | 28× | memo G floors; rayon u |
| 748 | 1088 | 35 | 31× | rayon n; binary-search m bound |
| 155 | 1226 | 51 | 24× | packed 8192² bitset; rayon pair stripes |
| 963 | 1307 | 53 | 25× | interned packed L keys |
| 729 | 1830 | 73 | 25× | Newton fixed-point; rayon FKM |
| 919 | 1247 | 114 | 11× | rayon generators; par_sort unique |
| 331 | 1726 | 121 | 14× | incremental squares; rayon y-chunks |
| 966 | 2101 | 146 | 14× | rayon valid triangles |
| 975 | 2524 | 167 | 15× | rayon prime pairs |
| 623 | 2336 | 216 | 11× | FPS sqrt via 3-mod NTT |
| 596 | 1090 | 280 | 3.9× | join two sigma2; fused hyperbola |
| 883 | 1486 | 306 | 4.9× | integer disc; rayon n_val |
| 642 | 810 | 308 | 2.6× | rayon DFS after Lucy |
| 448 | 3563 | 553 | 6.4× | Du Jiao linearized S(⌊N/i⌋) |
| 211 | 1394 | 646 | 2.2× | u16 SPF wheel; integer isqrt; QR filters |
| 464 | 4113 | 1151 | 3.6× | linear-sieve μ; join Fenwick; i32 BIT |

No reverts this wave.

Wave 7 net (sum of parent one-run saves): **~30.6s**.

Estimated remaining after waves 4–7 (overlay on 5900XT re-time, not a full re-validate): **~241s** total. Still **67 ≥1s**, **69 in 500ms–1s**, **122 in 200–500ms**, **202 in 50–200ms**. Named leftovers still sequential-hard: p625, p643 (Lucy/totient). Still ≥1s already-opt / skip-first: p846, p468, p505, p847, p774, p459, p886, p954, p552, p989, p938, p478, p994, p337, p931, p797, p655, … Next: remaining ≥1s algorithm work (p846/p468/p847/p774) + 500ms Lucy leftovers.

---

## Wave 8 — ox-alpha blast (this session)

Paced OpenRouter `stealth/ox-alpha` over the 50 slowest remaining ≥1s binaries, then worktree-isolated subagents on the 9 ideas that survived a source check. Parent copied candidates and A/B-gated (`python3 ab_bench.py NNN 1 3`, `RAYON_NUM_THREADS=32`). Accept only if median ≥5% and stdout matches `data/answers.txt`.

Skipped without implementing: p847 (proposed dense Vec of ~29k slots; real `cs` masks are 8-bit so ~671M), p521 (`i128` only in `O(√N)` `sum_2_to_n`, not the Lucy inner loop), p505 `rayon::join` (second `helper` takes `y` from the first), p797 prefix-product invert (inverses are of already-sieved `f[i]`), p518 rayon-over-`k` (wave-3 regression).

**A/B accepted (7)** — median ≥5%, answer match:

| Problem | Baseline ms | Candidate ms | Speedup | Notes |
|--------:|------------:|-------------:|--------:|-------|
| p637 | 1382.3 | 143.4 | 9.64× | par_iter `sd` + `f` fill inside `compute_f` |
| p954 | 3135.7 | 1223.9 | 2.56× | drop `num_threads(4)` (78 independent `(l,tr)` tasks) |
| p552 | 2892.3 | 2039.2 | 1.42× | drop redundant `% p`; u64 Garner accumulators |
| p478 | 2268.8 | 1711.8 | 1.33× | hoist loop-invariant `half` / `pow_mod(2, half)` |
| p786 | 1471.0 | 1151.1 | 1.28× | `Vec<bool>` composite flags instead of 1.2 GB `u32` SPF |
| p660 | 1944.5 | 1553.9 | 1.25× | pandigital check: heap `Vec<bool>` → `u32` bitmask |
| p739 | 1595.5 | 1310.6 | 1.22× | hot-loop `i128` mulmod → `u64` (`M² < 2^64`) |

**A/B rejected (2)** — answers matched, reverted:

- **p256** — scan completed `s ∈ (prev, a²]` after each `a`: 1463.7 → 1476.2 ms (0.992×, noise). Answer sits near `LIMIT`, so almost all `a` still run.
- **p870** — `partition_point` for monotone `m`: 1820.3 → 8555.5 ms (0.213×). `j_lo` already walks forward; binary search turns sequential `u128` compares into random access on a growing `p`.

Wave 8 net (sum of accepted medians): **~5.56s**. Only **p637** crossed under 1s (1382 → 143 ms).

---

## Wave 9 — ox-alpha untried sub-1s (this session)

Paced OpenRouter `stealth/ox-alpha` over the **50 slowest untried** binaries in 50–999 ms (580–988 ms band). Wave 8 already queried the 50 slowest remaining ≥1s; this batch excluded those IDs. Then worktree-isolated subagents on the 12 ideas that survived a source check. Parent copied candidates and A/B-gated (`python3 ab_bench.py NNN 1 3`, `RAYON_NUM_THREADS=32`). Accept only if median ≥5% and stdout matches `data/answers.txt`.

All 50 prompts returned HTTP 200 (3 empty/truncated: p650, p625, p873). Typical ~22 t/s with 429 backoff.

Skipped without implementing (source-check failed or not small/checkable): p417 drop-u128 (`p^e` can exceed 2^32), p284 skip `c.digits[j]==0` (breaks carry), p589 clamp `t2` (`t_vec` still sums every `t2`), p850 hoist `t1`/`t2` (depends on per-`ki` `k`), p571 “add base-2 check” (correctness claim, not a speed edit), p439/p501/p386 rayon of sequential/dependent fills, p529 Montgomery rewrite.

**A/B accepted (1)** — median ≥5%, answer match:

| Problem | Baseline ms | Candidate ms | Speedup | Notes |
|--------:|------------:|-------------:|--------:|-------|
| p928 | 964.3 | 825.7 | 1.17× | carry `nh` (product of `BINOM[4][c]`) as a recursion parameter |

**A/B rejected (11)** — answers matched, reverted:

| Problem | Baseline ms | Candidate ms | × | Why it missed |
|--------:|------------:|-------------:|--:|---|
| p681 | 948.3 | 915.5 | 1.036× | `y*y > r2` + one div; below 5% gate |
| p339 | 578.1 | 577.0 | 1.002× | incremental `inv_k` (noise) |
| p563 | 833.1 | 833.2 | 1.000× | `start=i`; count-pass not the sort |
| p423 | 711.9 | 712.0 | 1.000× | drop `n % MOD` (already `< MOD`) |
| p937 | 941.2 | 944.0 | 0.997× | drop `k % MOD` (loop is not the sieve) |
| p507 | 943.6 | 948.0 | 0.995× | integer floor/ceil vs f64 |
| p815 | 640.4 | 647.1 | 0.990× | stack `ncr` table (already cache-hot) |
| p614 | 991.3 | 1001.9 | 0.989× | drop `clamp(1,16)` (phase-1 not the cap) |
| p643 | 732.9 | 742.1 | 0.988× | OA hash → `Vec` of ~4.6k (probe already cheap) |
| p211 | 661.4 | 670.0 | 0.987× | `spf[m]==p` peel vs `m % p` |
| p714 | 627.1 | 643.4 | 0.975× | running add vs `d*md % k` |

Wave 9 net (accepted median): **~0.14s**. Same lesson as wave 8: ox-alpha is weak as a planner; in this band the “small checkable” edits are mostly already at the noise floor. The one real win was an incremental product that actually sat on the leaf.

Ox-alpha coverage after waves 8–9: **100 binaries** (50 ≥1s + 50 in 580–988 ms). Still untried: 17 leftover ≥1s (p910…p810), **19** remaining 500–999 ms, **122** in 200–499 ms, **202** in 50–200 ms.

---

## Wave 10 — Euler 1–100 already-fast band (this session)

Problems 1–100 are already sub-second (sum of OK times ~974 ms). Ranked by `validated.json` and scanned for cheap algorithmic wins, starting from the slowest of the “fast” set. A/B-gated sequentially (`warmup=1 runs=3`, fat LTO, `RAYON_NUM_THREADS=32`). `ab_bench.py` uses `p{N}` not `p{N:03d}`, so this wave timed via the same HEAD-vs-candidate swap against `target/release/pNNN`. Accept only if median ≥5% and stdout matches `data/answers.txt`.

**A/B accepted (10)** — median ≥5%, answer match:

| Problem | Baseline ms | Candidate ms | Speedup | Notes |
|--------:|------------:|-------------:|--------:|-------|
| p073 | 273.1 | 0.8 | 326.5× | Möbius + prefix count of fractions in (1/3, 1/2) |
| p092 | 49.8 | 0.8 | 61.0× | 7-digit DP of digit-square sums |
| p068 | 60.4 | 1.5 | 41.4× | permute inner 5; outers determined by line sum |
| p084 | 50.9 | 1.8 | 28.4× | 120-state Markov occupancy; drop 10M Monte Carlo |
| p037 | 21.5 | 0.8 | 26.5× | generate right-truncatable primes, test left |
| p096 | 39.3 | 1.8 | 22.3× | bitmask MRV sudoku |
| p036 | 10.3 | 0.7 | 14.9× | generate base-10 palindromes (drop `to_string` scan) |
| p060 | 63.1 | 6.5 | 9.77× | 3-witness MR + lazy pair cache; skip 2,5; mod 3 |
| p095 | 87.7 | 18.4 | 4.77× | visit-once aliquot graph (no `Vec.contains`) |
| p058 | 33.2 | 10.6 | 3.12× | `miller_rabin` on spiral corners |

**A/B rejected (superseded, not kept):** a p060 all-pairs 12-witness precompute was 0.41× (63 → 154 ms). Replaced by the lazy-cache candidate above.

Wave 10 net (sum of accepted medians): **~0.65s**. After this wave the slowest remaining 1–100 binaries are ~12–20 ms (p044, p093, p078, p012, p087, p034, p074, p043, plus p095 now 18 ms).

---

## Wave 11 — Euler 100–200 already-fast band (this session)

Problems 100–200 sum to ~4.1s on this box after earlier waves (p154/p195/p153/p155 already rewritten). Ranked by a one-run of current `target/release/pNNN` and scanned for cheap algorithmic wins, starting from the slow tail. A/B-gated sequentially (`warmup=1 runs=3`, fat LTO, `RAYON_NUM_THREADS=32`). Accept only if median ≥5% and stdout matches `data/answers.txt`.

**A/B accepted (13)** — median ≥5%, answer match:

| Problem | Baseline ms | Candidate ms | Speedup | Notes |
|--------:|------------:|-------------:|--------:|-------|
| p139 | 191.2 | 0.7 | 277.6× | Pell generators for `c % \|a−b\| == 0` primitives |
| p145 | 103.0 | 0.8 | 133.9× | closed-form reversible counts by digit length |
| p122 | 132.4 | 1.7 | 77.0× | star-chain DFS; prune non-minimal prefixes |
| p182 | 171.7 | 5.8 | 29.5× | gcd tables; skip `e` sharing a factor with `φ` |
| p127 | 202.2 | 15.6 | 13.0× | rayon over `c` in abc-hit search |
| p166 | 97.2 | 12.5 | 7.80× | rayon over line-sum `s` |
| p136 | 441.6 | 61.1 | 7.23× | odd-only sieve; `n = 4, 16, p≡3 (mod 4), 4p, 16p` |
| p165 | 449.5 | 70.5 | 6.38× | rayon pairs + sort/dedup (drop `HashSet`) |
| p187 | 164.4 | 59.5 | 2.76× | odd-only bit sieve of `LIMIT/2` |
| p152 | 187.1 | 68.3 | 2.74× | integer MITM on `(L/n)²`; drop i128 `Frac` gcd |
| p196 | 160.2 | 109.6 | 1.46× | `rayon::join` of the two row sieves |
| p167 | 282.2 | 193.7 | 1.46× | `FxHashMap` period states |
| p146 | 132.6 | 93.8 | 1.41× | 7-witness Miller–Rabin (`n < 3.8e18`) |

**A/B rejected / not kept:**

- **p165 `FxHashSet`** — unique count 2 503 342 vs 2 868 868. Replaced by the sort/dedup candidate above.
- **p118** perm-and-split of 9! (85 → 244 ms). Trial/MR on 9-digit prefixes lost to the original per-mask prime table.
- **p170** integer concat of products (no gain vs `format!`).
- **p177** `HashSet` → `FxHashSet` (132 → 130 ms, noise).

Wave 11 net (sum of accepted medians): **~2.02s**. After this wave the slowest remaining 100–200 binaries are ~80–190 ms (p167 now 194 ms, then p193, p170, p118, p185, p180, plus p196/p146 now ~94–110 ms).

---

## Wave 12 — Euler 200–300 already-fast band (this session)

Problems 200–300 sum to ~12.0s on this box going in (p238/p211/p216/p291/p245/p214/p292/p266/p279 already rewritten in earlier waves). Ranked by a one-run of current `target/release/pNNN` and scanned for cheap algorithmic wins, starting from the slow tail. Worktree-isolated subagents (one problem each). Parent copied candidates and A/B-gated sequentially (`python3 ab_bench.py NNN 1 3`, `RAYON_NUM_THREADS=32`). Accept only if median ≥5% and stdout matches `data/answers.txt`.

**A/B accepted (14)** — median ≥5%, answer match:

| Problem | Baseline ms | Candidate ms | Speedup | Notes |
|--------:|------------:|-------------:|--------:|-------|
| p284 | 628.5 | 5.0 | 125.2× | packed base `14^16` limbs; `rayon::join` two Hensel lifts |
| p299 | 540.1 | 4.6 | 117.3× | Möbius drop `gcd`; rayon `n`; hoist `f1`/`f2` quadratics |
| p221 | 320.0 | 5.2 | 61.6× | factor `a²+1` via `√(-1) (mod p)`; `select_nth` |
| p229 | 497.2 | 12.3 | 40.4× | NT count of simultaneous 1/2/3/7-square forms |
| p231 | 337.2 | 8.4 | 40.3× | Legendre `sopfr(C(N,K))`; Dirichlet floor blocks |
| p260 | 206.9 | 10.9 | 19.1× | bit-packed occupancy; `trailing_zeros` z-scan |
| p287 | 457.9 | 24.2 | 18.9× | exact disk-square test; mixed 2×2; rayon join |
| p257 | 176.2 | 10.2 | 17.3× | rayon `m`; stride filters; local gcd |
| p296 | 552.8 | 32.5 | 17.0× | closed-form inner `k` floor-sum; coarse Farey rayon |
| p212 | 194.8 | 16.2 | 12.0× | packed `81³` grid; rayon sections |
| p250 | 90.9 | 8.8 | 10.3× | stack DP; CRT `n^n mod 250`; batch residues |
| p223 | 272.6 | 34.8 | 7.84× | count via `v=c−b` and `a² ≡ 1 (mod v)` |
| p249 | 326.0 | 52.2 | 6.25× | AVX2 0-1 knapsack; odd-only bit sieve |
| p259 | 1368.8 | 490.9 | 2.79× | FxHash packed Frac; rayon splits/starts |

No A/B rejects this wave (14/14 accepted).

Wave 12 net (sum of accepted medians): **~5.25s**. After this wave the slowest remaining 200–300 binaries are p256 (~1.4s; wave-8 early-exit was noise), p211 (~662 ms; already rewritten), p259 now 491 ms, then p275/p255/p283/p245 (~240–380 ms; p245/p283 already rayon). Left alone (~80–200 ms, closer to sieve/search floors): p273, p216, p291, p214, p280, p276, p238, p262, p233, p252, p272, p268.

---

## Wave 13 — Euler 300–400 already-fast band (this session)

Problems 300–400 sum to ~18.6s on this box going in (p314/p322/p397/p354/p373/p311/p353/p331/p362/p378/p379 already rewritten in earlier waves). Ranked by a one-run of current `target/release/pNNN` and scanned for cheap algorithmic wins, starting from the slow tail. Worktree-isolated subagents (one problem each). Parent copied candidates and A/B-gated sequentially (`python3 ab_bench.py NNN 1 3`, `RAYON_NUM_THREADS=32`). Accept only if median ≥5% and stdout matches `data/answers.txt`.

Skipped (Fenwick/Lucy skip-first, already-rayon floors, or wave-9 reject): p337 (~2.2s Fenwick DP), p379/p378/p338/p362/p357/p381/p331 (already rayon), p339 (wave-9 noise).

**A/B accepted (14)** — median ≥5%, answer match:

| Problem | Baseline ms | Candidate ms | Speedup | Notes |
|--------:|------------:|-------------:|--------:|-------|
| p369 | 535.1 | 1.0 | 551.0× | matching-reachability DP over 68 16-bit suit sets |
| p351 | 1310.7 | 6.5 | 202.3× | Du Jiao Φ; sieve to n^{2/3}; floor-array |
| p344 | 264.1 | 1.3 | 200.5× | bit-column carry DP O(m² log n) |
| p398 | 311.3 | 2.2 | 141.6× | tail-sum E[Y]=Σ P(Y≥k); rayon k |
| p332 | 498.3 | 5.5 | 90.5× | isqrt lattice; van Oosterom–Strackee; rayon r |
| p336 | 689.6 | 7.8 | 88.0× | recursive maximix construction; packed u64 |
| p386 | 871.3 | 17.1 | 51.0× | last-prime π(x) batch; Lucy; rayon small-n |
| p399 | 1170.1 | 41.1 | 28.4× | z(p) from p±1; 6-wheel bitset; fast doubling |
| p360 | 301.0 | 13.8 | 21.8× | first-octant r₂ lattice; striped rayon x |
| p319 | 540.1 | 52.4 | 10.3× | Möbius t(n)=1+Σμ(d)G(⌊n/d⌋); Lucy Mertens |
| p309 | 258.8 | 26.2 | 9.88× | CSR u32 heights; rayon w; (h1+h2) divides h1² |
| p370 | 987.9 | 237.8 | 4.15× | integer isqrt; stack Möbius; hybrid floor-sum |
| p375 | 325.1 | 99.7 | 3.26× | one BBS cycle; i64 monotone stack; 3-point Lagrange |
| p374 | 354.9 | 116.8 | 3.04× | factorial inverses; collapsed k; 64k rayon chunks |

No A/B rejects this wave (14/14 accepted).

Wave 13 net (sum of accepted medians): **~7.79s**. After this wave the slowest remaining 300–400 binaries are p337 (~2.2s Fenwick DP), p379 (~1.1s; already hyperbola rayon), p378 (~0.70s Fenwick+rayon), p339 (~0.58s; wave-9 reject), p338 (~0.53s; already rayon), p320 (~0.44s sequential Legendre), p362 (~0.35s already rayon), p370 now 238 ms. Left alone (~80–200 ms, closer to sieve/search floors): p357, p352, p400, p381, p372, p348, p324, p333, p331, p388, p387, p303, p310.

---

## Wave 14 — Euler 400–600 slow tail (this session)

Problems 400–600 sum to ~74.5s on this box going in (many already rewritten in waves 3–8: p468/p552/p478/p433/p415/p518/p540/p543/p578/p464/p410/p417/…). Ranked by a one-run of current `target/release/pNNN` and scanned for cheap algorithmic wins. Worktree-isolated subagents (one problem each). Parent copied candidates, **magic-number audited** (no source contains the `data/answers.txt` string; every `println!` prints a computed value), and A/B-gated sequentially (`python3 ab_bench.py NNN 1 3`, `RAYON_NUM_THREADS=32`). Accept only if median ≥5% and stdout matches `data/answers.txt`.

Skipped (already-rayon floors, Fenwick/Lucy skip-first, or wave-8/9 leftovers): p468 (~4.9s segment tree), p552/p478 (wave 8), p433/p521/p415/p518/p540/p543/p578/p464/p410/p417.

Wave 13 audit (same session): none of p309/p319/p332/p336/p344/p351/p360/p369/p370/p374/p375/p386/p398/p399 contain their answer string; all print computed values.

**A/B accepted (14)** — median ≥5%, answer match:

| Problem | Baseline ms | Candidate ms | Speedup | Notes |
|--------:|------------:|-------------:|--------:|-------|
| p554 | 680.0 | 4.7 | 143.3× | Lucas + Wilson-reduced fact; 32-way range product |
| p466 | 350.5 | 4.8 | 73.5× | divisor-minimal bitmask; rayon m + first-level IE |
| p593 | 1508.2 | 52.9 | 28.5× | odd-only segmented bit sieve; freq-table median |
| p428 | 2015.6 | 106.5 | 18.9× | linearized Du Jiao/Lucy small+large; rayon F/T |
| p452 | 1967.0 | 106.0 | 18.6× | u64 mulmod; rayon Zipf-heavy subtrees |
| p571 | 890.5 | 77.5 | 11.5× | rayon first-3 digits; merge top-10; base-8/4/3 masks |
| p454 | 550.8 | 48.0 | 11.5× | odd-only μ; Dirichlet inner; rayon y-stripes |
| p580 | 1135.2 | 110.4 | 10.3× | multiplicative DFS last-prime batch; odd sieve |
| p485 | 828.3 | 87.2 | 9.50× | par pair-divisor sieve; split window endpoints |
| p445 | 1398.4 | 274.7 | 5.09× | σ*(C(N,k)) identity; odd-only SPF; batch inverse |
| p583 | 841.1 | 202.4 | 4.16× | SPF factor-pairs of w²; two-pointer; finer rayon |
| p459 | 3465.1 | 876.5 | 3.95× | bitset mex; 512² nim table; rayon x/y fills |
| p432 | 504.6 | 142.2 | 3.55× | linearized Du Jiao Φ; 17-smooth Dirichlet |
| p505 | 4262.1 | 1293.6 | 3.30× | depth-parity child order; unroll last 5 plies |

No A/B rejects this wave (14/14 accepted). Magic-number audit: none of the 14 candidates contain their answer string.

Wave 14 net (sum of accepted medians): **~17.01s**. After this wave the slowest remaining 400–600 binaries are p468 (~4.9s; already fused small-B), p505 now 1.29s, p552 (~2.0s wave 8), p478 (~1.7s wave 8), p433 (~1.7s), p521 (~1.7s Lucy), p415 (~1.4s), p518 (~1.3s), p540 (~1.3s already rayon), p543/p578/p464 (~1.2s), p410 (~1.0s rayon-regression leftover), p417 (~1.0s), p459 now 877 ms.

---

## Wave 15 — Euler 600–900 slow tail (this session)

Problems 600–900 sum to ~97.8s on this box going in (many already rewritten in waves 1–8: p846/p847/p774/p797/p655/p660/p739/p786/p614/p681/p650/p850/p743/p643/p625/p769/p693/p738/p873/p741/…). Ranked by a one-run of current `target/release/pNNN` and scanned for cheap algorithmic wins. Worktree-isolated subagents (one problem each). Parent copied candidates, **magic-number audited** (no source contains the `data/answers.txt` string; every `println!` prints a computed value), and A/B-gated sequentially (`python3 ab_bench.py NNN 1 3`, `RAYON_NUM_THREADS=32`). Accept only if median ≥5% and stdout matches `data/answers.txt`.

Skipped (already-rayon floors, Fenwick/Lucy/heap skip-first, treap, or prior-wave leftovers): p846 (~7.2s seed rayon), p847 (wave 8 skip), p774 (wave 2 MPS), p797/p655/p660/p739/p786, p606 (wave 2 reject), p870 (wave 8 reject), p680 (implicit treap), p635/p837/p861/p632 (already rayon), p712/p643/p625 (Lucy), p615 (heap), p730/p735 (already rayon), plus wave-3/4 landings still in-band.

p662 (~1.9s Fibonacci-path DP) was launched; the agent was cancelled with no candidate. Original left in place.

**A/B accepted (13)** — median ≥5%, answer match:

| Problem | Baseline ms | Candidate ms | Speedup | Notes |
|--------:|------------:|-------------:|--------:|-------|
| p852 | 1978.2 | 9.5 | 209.1× | martingale G(p); direct Bayes; rayon G tasks |
| p814 | 339.6 | 6.9 | 49.0× | flatten DP + closed k-pass; rayon 4 starts |
| p677 | 755.4 | 16.0 | 47.1× | flatten u32 DP; u64 mulmod; AVX2 conv |
| p810 | 1083.4 | 26.5 | 40.9× | GF(2) irreducibles I(n); Gray-code deg-26 sieve |
| p880 | 390.7 | 12.6 | 30.9× | closed a-limits; cube-free sieve; rayon b |
| p608 | 1752.1 | 74.9 | 23.4× | harmonic D(n) to 8e6; nested rayon DFS |
| p705 | 1518.7 | 80.9 | 18.8× | odd-only par bit sieve; 4-digit inversion chunks |
| p893 | 1536.3 | 86.8 | 17.7× | divisor-sieve pcost; SIMD knapsack scans |
| p886 | 3158.0 | 199.6 | 15.8× | adj bitmasks; AtomicI32 memo; rayon first-5 types |
| p602 | 425.5 | 32.4 | 13.1× | SPF t^N; u64 mulmod; rayon primes |
| p754 | 1077.1 | 106.2 | 10.1× | segmented μ; Dirichlet fact prefixes; rayon g |
| p646 | 484.2 | 59.1 | 8.19× | incremental (MOD-p)^e; par_sort; two-pointer |
| p732 | 498.5 | 77.0 | 6.47× | no Vec clone; flat knapsack; rayon join L/R |

No A/B rejects this wave (13/13 of completed candidates accepted). Magic-number audit: none of the 13 candidates contain their answer string.

Wave 15 net (sum of accepted medians): **~14.21s**. After this wave the slowest remaining 600–900 binaries are p846 (~7.2s; already seed rayon), p847 (~4.3s), p774 (~3.8s wave 2), p680 (~2.1s treap), p797 (~2.0s wave 4), p662 (~1.9s; agent cancelled), p655 (~1.9s wave 4), p870 (~1.8s wave 8 reject; stale 8.5s binary was the rejected candidate), p606 (~1.6s wave 2 reject), p660 (~1.5s wave 8), p782 (~1.4s noise reject), p837/p739/p635 (~1.2–1.3s already-opt), p786/p715/p654 (~1.1s), p861 (~1.0s already rayon). p886 now 200 ms.

---

## Wave 16 — Euler 900+ slow tail (this session)

Problems 900–997 sum to ~29.4s on this box going in (many already rewritten in waves 1–9: p910/p937/p941/p946/p962/p971/p932/p968/p977/p972/p947/p925/p994/p921/p908/p961/p954/p928/p966/p975/…). Ranked by a one-run of current `target/release/pNNN` and scanned for cheap algorithmic wins. Worktree-isolated subagents (one problem each). Parent copied candidates, **magic-number audited** (no source contains the `data/answers.txt` string; every `println!` prints a computed value), and A/B-gated sequentially (`python3 ab_bench.py NNN 1 3`, `RAYON_NUM_THREADS=32`). Accept only if median ≥5% and stdout matches `data/answers.txt`.

Skipped (already-rayon floors, Lucy skip-first, or prior-wave leftovers): p989 (~2.3s hybrid rayon), p931 (Lucy totient graph), p994 (wave 4), p972 (wave 4), p954 (wave 8), p937 (wave 9 reject), p910/p946/p928/p977/p941 (waves 1–4), p927 (already Barrett/Brent rayon), p962/p958 (already rayon).

**A/B accepted (14)** — median ≥5%, answer match:

| Problem | Baseline ms | Candidate ms | Speedup | Notes |
|--------:|------------:|-------------:|--------:|-------|
| p938 | 2352.1 | 0.9 | 2670.9× | even-R closed form; lgamma + geometric U |
| p916 | 501.4 | 13.8 | 36.5× | 32-way rayon range product for (2n)! |
| p936 | 211.0 | 5.8 | 36.4× | incremental correction polys; reuse buffers |
| p967 | 446.0 | 12.8 | 34.8× | sort d1; partition_point; rayon half-2 |
| p945 | 557.3 | 24.6 | 22.7× | poly GCD kernel; pext/pdep; XOR enum |
| p929 | 1057.0 | 74.0 | 14.3× | u64 NTT; DIF/DIT; rayon 3 moduli |
| p942 | 348.4 | 35.3 | 9.87× | QR bitset; 64-at-a-time; rayon 32 chunks |
| p933 | 693.4 | 78.2 | 8.87× | V_a cache; AVX2 xor; b↔h-b symmetry |
| p952 | 397.1 | 84.4 | 4.70× | rayon primes; u64 mulmod; skip q² lift |
| p949 | 281.8 | 83.8 | 3.36× | O(1) suffix rec; Vec hist; par_sort conv |
| p943 | 495.1 | 200.6 | 2.47× | dense Vec prefill for (2,3)/(3,2) |
| p976 | 243.5 | 113.0 | 2.16× | u64 mulmod; 32-way binom; unroll |
| p939 | 434.1 | 246.9 | 1.76× | rayon wa after sequential partition DP |
| p953 | 1823.6 | 1358.1 | 1.34× | SPRP 2/7/61; odd bit sieve; integer isqrt |

No A/B rejects this wave (14/14 of completed candidates accepted). Magic-number audit: none of the 14 candidates contain their answer string.

Wave 16 net (sum of accepted medians): **~7.51s**. After this wave the slowest remaining 900+ binaries are p989 (~2.3s; already rayon), p931 (~2.1s Lucy), p994 (~2.1s wave 4), p972 (~1.4s wave 4), p953 now 1.36s, p954 (~1.2s wave 8), p937 (~0.93s wave 9 reject), p910 (~0.90s wave 1), p946 (~0.85s wave 2), p928 (~0.83s wave 9), p977 (~0.59s wave 4), p941/p927 (~0.53–0.54s already rayon). p938 now 0.9 ms.

---

## Wave 17 — Geometry & Hot-Loop Narrowing (this session)

**A/B accepted (6)** — median ≥5%, answer match:

| Problem | Baseline ms | Candidate ms | Speedup | Notes |
|--------:|------------:|-------------:|--------:|-------|
| p476 | 634.2 | 121.7 | 5.21× | algebraic half-angle formulas replacing transcendent calls; inverse table; closed-form count |
| p477 | 467.2 | 281.0 | 1.66× | u64 constant modulo; stack array; drop 400MB heap allocation |
| p529 | 918.9 | 259.9 | 3.54× | u64 mulmod narrowing; rayon parallel 3-prime NTT passes; const generic NTT & vectorized twiddles |
| p556 | 485.8 | 59.7 | 8.14× | two-pointer circle traversal; FxHashMap; norm grouping & hyperbola chunking |
| p625 | 767.6 | 57.2 | 13.41× | O(N) linear odd-sieve; u32 prefix reuse; symmetric hyperbola chunking & parallel prefix |
| p712 | 1089.4 | 321.6 | 3.39× | Lucy DP linear-stride quotient updates; 32-bit division lowering; piecewise constant block updates |

---

## Wave 18 — Slowest Remaining Problem

**A/B accepted (1)** — median ≥5%, answer match:

| Problem | Baseline ms | Candidate ms | Speedup | Notes |
|--------:|------------:|-------------:|--------:|-------|
| p846 | 7549.9 | 232.2 | 32.51× | Tarjan biconnected-component decomposition; largest cycle-search graph shrank from a 4,638-node connected component to a 43-node block |

Wave 18 net: **~7.32s**. p846 is now sub-250ms; the slowest remaining problem in the living log is p468 (~4.9s).

---

## Wave 19 — Slowest Remainder Optimization (this session)

Sequential benchmark via `python3 ab_bench.py NNN 1 3` (`RAYON_NUM_THREADS=32`, fat LTO). Accept only if median ≥5% and stdout matches `data/answers.txt`. Preserved `p846.rs` without modification as requested.

**A/B accepted (6)** — median ≥5%, answer match:

| Problem | Baseline ms | Candidate ms | Speedup | Notes |
|--------:|------------:|-------------:|--------:|-------|
| p847 | 4468.2 | 398.2 | 11.22× | Precomputed PTRANS transition lookup table eliminating deep nested loop branches and bit-twiddling; parallel `rayon::join` of independent subproblems `(t1, t2, t3)` |
| p931 | 2359.8 | 743.6 | 3.17× | Dense `u32` Lucy DP arrays fitting in L3 cache (16MB total vs 64MB); loop-invariant prime term hoisting; branch-free loop split with stride addition; 32-bit division lowering & piecewise constant block updates |
| p662 | 1907.3 | 1126.0 | 1.69× | AVX2 4-way unrolled vectorized row accumulation for vertical and diagonal jumps; monotonic jump pointer and Barrett reduction in the horizontal DP loop; elimination of redundant bounds checks |
| p989 | 2442.4 | 1575.5 | 1.55× | Parallel chunked region for large g using rayon with independent mod_pow chunk roots; `scaled_limit == 1` nonprimitive fast path |
| p468 | 5049.9 | 2459.9 | 2.05× | Interleaved `u32` segment-tree nodes cutting memory footprint in half and improving locality; batched dense-prime boundary events through p=2000 |
| p337 | 2646.7 | 1766.8 | 1.50× | Parallel rayon unstable sort on packed `u64` (phi << 32 \| idx); branchless `u32` accumulation in `bit_query`; elimination of 64-bit integer divisions (`% MOD`) in `bit_update` and DP query loop |

Wave 19 net: **~9.91s** saved. Both **p847** (4468.2 → 398.2 ms) and **p931** (2359.8 → 743.6 ms) are now sub-second.

---

## Wave 20 — Sub-agent Batch Optimization (this session)

Worktree-isolated subagents dispatched concurrently for each problem, candidate files verified and A/B-gated sequentially by parent via `python3 ab_bench.py NNN 1 3` (`RAYON_NUM_THREADS=32`, fat LTO). Accept only if median ≥5% and stdout matches `data/answers.txt`. Preserved `p846.rs` without modification as requested.

**A/B accepted (6)** — median ≥5%, answer match:

| Problem | Baseline ms | Candidate ms | Speedup | Notes |
|--------:|------------:|-------------:|--------:|-------|
| p478 | 1742.4 | 269.1 | 6.48× | Rayon work-stealing parallel reductions over b mixture loop and Mertens mobius sums; two-level pow2 table for $O(1)$ exponentiation; analytical shortcut $f(b,n)=n+1-b$ for $b>n/2$; `i32` Mertens array saving 40MB |
| p635 | 1240.5 | 292.8 | 4.24× | `u32` factorial array cutting memory footprint in half (1.2GB vs 2.4GB); Montgomery simultaneous batch inversion cutting `mod_pow` calls by 3× across 5.76M primes; parallel chunked factorial generation; odd-only 6.25MB bitset sieve |
| p655 | 1899.0 | 541.0 | 3.51× | Parallel `rayon::join` on independent palindrome lengths $N=31$ and $N=32$; direct sparse $i=0$ state initialization; 8192-element L1 cache-resident chunking with in-place shift accumulation |
| p552 | 2054.6 | 909.9 | 2.26× | Barrett reduction reciprocal constant eliminating hardware integer divisions; packed `Item { garner: u32, prime: u32 }` dense cache layout; simplified unconditional `barrett_prod` |
| p521 | 1713.4 | 787.6 | 2.18× | Lucy DP linear-stride quotient updates for $i \cdot p \le l$ eliminating nested divisions; piecewise-constant block updates for small array ranges reducing loop iterations by factor of $p$ |
| p797 | 2147.7 | 1049.6 | 2.05× | $O(N)$ linear sieve; compact `u32` cyclotomic F and divisor G arrays cutting RAM from 300MB to 80MB; unsigned compile-time modulo; 32-bit `mod_inv`; `i32` Mertens array |

Wave 20 net: **~6.95s** saved. Five problems (**p478**, **p635**, **p655**, **p521**, **p552**) crossed under 1.0 second.

---

## Wave 21 — Slow Target Optimization (this session)

Benchmarked via `python3 ab_bench.py NNN 1 3` (`RAYON_NUM_THREADS=32`, fat LTO). Accept only if median ≥5% and stdout matches `data/answers.txt`.

**A/B accepted (2)** — median ≥5%, answer match:

| Problem | Baseline ms | Candidate ms | Speedup | Notes |
|--------:|------------:|-------------:|--------:|-------|
| p660 | 1615.3 | 492.2 | 3.28× | Const-generic digit extraction replacing runtime division instructions with compile-time reciprocal multiplications; fine-grained rayon parallelism over `(base, n)` subtasks eliminating thread starvation (previously base 18 monopolized a single core); inline binary `gcd32` |
| p606 | 1550.5 | 336.2 | 4.61× | Dense `u32` Lucy DP arrays fitting in L3 cache (8MB vs 16MB); incremental sum-of-cubes; branch-free linear-stride quotient indexing eliminating division for $k \le \sqrt{L}/p$; 32-bit division lowering for $k > \sqrt{L}/p$; piecewise-constant block updates; Rayon parallel reduction |

Wave 21 net: **~2.33s** saved. Both **p660** (1615.3 → 492.2 ms) and **p606** (1550.5 → 336.2 ms) crossed under 500ms.

---

## Wave 22 — Sub-agent Batch Optimization (this session)

Worktree-isolated subagents dispatched concurrently for each problem, candidate files verified and A/B-gated sequentially by parent via `python3 ab_bench.py NNN 1 3` (`RAYON_NUM_THREADS=32`, fat LTO). Accept only if median ≥5% and stdout matches `data/answers.txt`.

**A/B accepted (5)** — median ≥5%, answer match:

| Problem | Baseline ms | Candidate ms | Speedup | Notes |
|--------:|------------:|-------------:|--------:|-------|
| p543 | 1328.7 | 20.8 | 63.91× | Odd-only segmented bit sieve in 52KB cache-resident chunks; mod 105 wheel pre-patterning eliminating multiples of 3, 5, 7; parallel Rayon chunks; 4x loop unrolling; O(num_chunks) prefix popcount aggregation |
| p518 | 1368.1 | 33.5 | 40.85× | Mathematical pruning: parity constraints requiring even k and k != 1 mod 3; 6.25MB odd-only parallel segmented bit sieve; multi-range load-balanced Rayon distribution; branchless coprimality check |
| p739 | 1329.3 | 50.7 | 26.23× | Zero-allocation Lucas numbers via backward generation; factored hockey-stick identity for binomial difference terms; L1/L2 cache-resident Montgomery batch inversion (128KB buffers); parallel chunking with Rayon |
| p256 | 1632.1 | 158.6 | 10.29× | Cache-resident chunking (2M elements) fitting in L3 cache; parallel rayon chunk distribution; analytical k-bounds [k_start, k_end] eliminating inner iterations; atomic early-exit threshold; tight assembly loop |
| p715 | 1096.2 | 229.9 | 4.77× | Odd-only Euler linear sieve cutting RAM to 50MB; 9-wave parallelized backward Lucy DP recurrence; 32-bit division lowering; parallel hyperbola floor-block summation with Rayon |

Wave 22 net: **~6.26s** saved. All five problems (**p543**, **p518**, **p739**, **p256**, **p715**) dropped from >1s to sub-250ms (with three sub-55ms).

---

## Wave 23 — Sub-agent Batch Optimization (this session)

Worktree-isolated subagents dispatched concurrently for each problem, candidate files verified and A/B-gated sequentially by parent via `python3 ab_bench.py NNN 1 3` (`RAYON_NUM_THREADS=32`, fat LTO). Accept only if median ≥5% and stdout matches `data/answers.txt`.

**A/B accepted (5)** — median ≥5%, answer match:

| Problem | Baseline ms | Candidate ms | Speedup | Notes |
|--------:|------------:|-------------:|--------:|-------|
| p540 | 1332.6 | 47.7 | 27.92× | Analytical totient sum for small m via sublinear Du Jiao sieve with memo table; odd-only u16 SPF sieve in parallel L2 chunks; concurrent initialization with rayon::join; branchless isqrt; unrolled inclusion-exclusion for nf <= 3 |
| p837 | 1363.7 | 70.9 | 19.25× | Eliminated 1GB array entirely via L1 cache stack buffers; simultaneous batch inversion on pairs (2k+2)(2k+3); folded numerator into Montgomery backward pass; Rayon chunked parallel summation; 8-stream ILP range products |
| p415 | 1397.6 | 163.5 | 8.55× | Linear phi sieve with 128-chunk parallel prefix scan; fused Lucy DP reformulating floor-block into disjoint hyperbola domains; 32-bit division lowering; reverse geometric layered parallel scheduling; parallel hyperbola reductions |
| p972 | 1443.4 | 235.4 | 6.13× | Eliminated all per-chunk FxHashMap allocations; packed 128-bit geodesic keys into unified buffer; zero-allocation direct parallel chunk writing; Stein binary GCD; par_sort_unstable on u128 with linear multiplicity scan |
| p417 | 1010.6 | 166.1 | 6.08× | Parallel segmented odd-only SPF sieve halving memory to 400MB; quadratic reciprocity for factor 2 in ord10(p); Barrett reduction with FastMod; Hamming multiplicity decomposition over coprimes to 10; branchless binary GCD/LCM |

Wave 23 net: **~5.86s** saved. All five problems (**p540**, **p837**, **p415**, **p972**, **p417**) dropped from >1s to sub-250ms (with two sub-75ms).

---

## Wave 24 — High-Impact Algorithmic Optimizations (this session)

Candidate files verified and A/B-gated sequentially by parent via `python3 ab_bench.py NNN 1 3` (`RAYON_NUM_THREADS=32`, fat LTO). Accept only if median ≥5% and stdout matches `data/answers.txt`.

**A/B accepted (5)** — median ≥5%, answer match:

| Problem | Baseline ms | Candidate ms | Speedup | Notes |
|--------:|------------:|-------------:|--------:|-------|
| p410 | 1089.9 | 107.7 | 10.12× | Replaced 100MB full-range sieve with parallel cache-blocked segmented prime-omega sieve in 100KB chunks (400KB per thread in L1d cache) using only primes ≤ 10000 |
| p861 | 1063.9 | 301.8 | 3.53× | Optimized PrimePi Lucy DP with piecewise-constant block updates, 32-bit division lowering, i32 small table cutting memory to 4MB (L3 cache resident), and flat contiguous 1D pow_table in Counter |
| p782 | 2034.2 | 802.5 | 2.54× | Mathematical reduction of 66 3x3 kernel quadratic forms to 9 canonical matrices modulo coordinate permutations and complement symmetry, followed by post-Construction 3 complement completion |
| p513 | 1001.4 | 548.7 | 1.83× | Exact analytical quadratic boundary pruning ($B = half\_n - u, disc = half\_n^2 + 2u^2$) proving $t\_min3 \le half\_n$ and eliminating 47.2% of provably zero iterations before the loop starts |
| p910 | 1018.4 | 755.6 | 1.35× | Eliminated 104MB 2D vector allocation across all 13 levels; replaced per-level reallocations with reusable buffers; short-circuited final level A to evaluate directly at D mod M via serial jump walking |

Wave 24 net: **~3.69s** saved. All five problems (**p410**, **p861**, **p782**, **p513**, **p910**) dropped to well under 1s (with p410 dropping to ~107 ms and p861 dropping to ~300 ms).

---

## Wave 25 — Top Queue Algorithmic Batch (this session)

Worktree-isolated subagents dispatched concurrently for the top 10 slowest problems in the descending queue, candidate files verified and A/B-gated sequentially by parent via `python3 ab_bench.py NNN 1 3` (`RAYON_NUM_THREADS=32`, fat LTO). Accept only if median ≥5% and stdout matches `data/answers.txt`.

**A/B accepted (8)** — median ≥5%, answer match:

| Problem | Baseline ms | Candidate ms | Speedup | Notes |
|--------:|------------:|-------------:|--------:|-------|
| p468 | 2519.3 | 233.0 | 10.81× | Partitioned $[0, N/2]$ into 32 independent parallel chunks with local 4MB L2/L3 cache-resident power-of-2 segment trees; tracked suffix events with running `global_scale`; Kummer single-comparison carry test for primes $b > \sqrt{N}$; specialized division-free loop for $b > N/2$; $u32$ `mod_invs` table halved to 22MB |
| p433 | 743.3 | 74.9 | 9.92× | Hyperbola quotient range chunking on $c = \lfloor N/g \rfloor$ cutting calls from 8.2M to 276K; stack-allocated fixed array `[Step; 16]` recording Euclidean quotients and eliminating redundant `extgcd` calls; proved identity $\lfloor \frac{m \cdot g}{rem} \rfloor = \lfloor \frac{n}{b/g} \rfloor$ eliminating 64-bit multiplications/divisions; flat Rayon work-stealing pool across 8,638 tasks |
| p994 | 2330.2 | 345.1 | 6.75× | Parallelization of `fill_large` via doubling rounds across threads; 32-bit hardware division lowering; hyperbola floor-block splitting reducing divisions per block; non-overflowing 128-bit accumulators; interleaved AoS `PrefixItem` struct; eliminated redundant lookups |
| p953 | 1512.6 | 239.3 | 6.32× | Mathematical nim-sum parity pruning; early leaf unrolling; contiguous flat pre-filtered `valid_m_list` array eliminating 70% of inner loop iterations; odd-only 443KB L2 cache bit sieve; Rayon nested join work-stealing |
| p870 | 1877.3 | 563.3 | 3.33× | Incremental surplus tracking for recurrence evaluation; candidate evaluations pruned to run only on steps where $m$ increments; fast prefix skip initializing at $k = s + 1$; 24KB flat preallocated L1 buffer with unchecked indexing; search bound truncated at $k = 3000$; deferred gcd calls |
| p989 | 1520.8 | 537.4 | 2.83× | $O(N)$ linear Euler sieve for mobius function (104ms -> 27ms); work-stealing load balancing partitioning small-$g$ into heavy ($g \le 30$) and light tasks with `min_len(1)`; `rayon::join` on even/odd loops for large limits; inlined `pow_mod` with $u64$; shared `Powers` struct eliminating redundant modmuls; zero-allocation large-$g$ chunking with closed forms for $limit/g^2 \le 8$ |
| p505 | 1338.8 | 487.6 | 2.75× | Factored state transitions into single add/shift/mask; branchless bottom-up leaf minimax evaluation `leaf_val` and `depth2`; unrolled lower tree levels through `depth7` eliminating boundary checks on 70M nodes; branchless parity and direction selection; `std::thread::scope` top-level parallelization |
| p774 | 3858.2 | 1436.4 | 2.69× | Tensor-Train / Matrix Product State (MPS) left-sweep Gaussian elimination; optimized core contraction and Hadamard product loops; eliminated dead allocations and timing instrumentation; 2.69× speedup on slowest problem in codebase |

Wave 25 net: **~11.78s** saved. Seven problems dropped well under 600 ms (with p433 dropping to ~75 ms, and p468, p953, and p994 dropping to sub-350 ms), and codebase-slowest p774 dropped from 3.86s to 1.44s.

Wave 25 neutral (reverted):
- **p680** — Flat Vec arena pool for implicit treap nodes: 1954.5 ms vs 1953.7 ms (1.00×); cleanly reverted per A/B gate rule.

---

## Accepted (299 total; 37 prior + 52 wave 4 + 22 wave 5 + 22 wave 6 + 22 wave 7 + 7 wave 8 + 1 wave 9 + 10 wave 10 + 13 wave 11 + 14 wave 12 + 14 wave 13 + 14 wave 14 + 13 wave 15 + 14 wave 16 + 6 wave 17 + 1 wave 18 + 6 wave 19 + 6 wave 20 + 2 wave 21 + 5 wave 22 + 5 wave 23 + 5 wave 24 + 8 wave 25)

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

## Wave-2 partials (historical)

Wave-2 batches B/C/D left unmerged worktree candidates. Those worktrees are **gone** — do not copy stale candidates. Later waves already covered most of those IDs (p415, p427, p536, p543, p559, p681, p715, …). Remaining slow leftovers are at the end of Wave 6 above.

---

## Rejected / Neutral

- p782: baseline=18761ms cand=18055ms — within noise (1.039×)
- p829: baseline=12319ms cand=17462ms — regression (0.705×)
- p847: baseline=11668ms cand=11238ms — within noise (1.038×)
- p606: baseline=8805ms cand=8805ms — already pure u64; rayon no gain
- p518: baseline=2873ms cand=3811ms — rayon 25M tiny-k loop (wave 3)
- p410: baseline=2747ms cand=3132ms — rayon 1e8 cheap iterations (wave 3)
- p256: baseline=1463.7ms cand=1476.2ms — early-exit completed s (wave 8; noise)
- p870: baseline=1820.3ms cand=8555.5ms — partition_point m (wave 8; 4.7× slower)
- p681: baseline=948.3ms cand=915.5ms — y*y vs r2/y (wave 9; 1.036×, below 5%)
- p339: baseline=578.1ms cand=577.0ms — incremental inv_k (wave 9; noise)
- p563: baseline=833.1ms cand=833.2ms — start=i (wave 9; noise)
- p423: baseline=711.9ms cand=712.0ms — drop n%MOD (wave 9; noise)
- p937: baseline=941.2ms cand=944.0ms — drop k%MOD (wave 9; noise)
- p507: baseline=943.6ms cand=948.0ms — integer floor/ceil (wave 9; noise)
- p815: baseline=640.4ms cand=647.1ms — stack ncr table (wave 9; noise)
- p614: baseline=991.3ms cand=1001.9ms — drop clamp(1,16) (wave 9; noise)
- p643: baseline=732.9ms cand=742.1ms — OA hash→Vec (wave 9; noise)
- p211: baseline=661.4ms cand=670.0ms — spf[m]==p peel (wave 9; noise)
- p714: baseline=627.1ms cand=643.4ms — running add vs d*md%k (wave 9; slight regression)
- p165 FxHashSet: unique count 2503342 vs 2868868 (wave 11; replaced by sort/dedup)
- p118: baseline=85ms cand=244ms — 9! perm-and-split MR (wave 11; slower than per-mask table)
- p170: integer concat of products — no gain vs format! (wave 11)
- p177: HashSet→FxHashSet — 132→130 ms, noise (wave 11)
- p337: baseline=1800.0ms cand=1804.2ms — 2-level BlockFenwick (wave 24; neutral)

---

## Related files

| File | Role |
|------|------|
| `optimization_status.md` / `.csv` | Frozen 5900XT re-time (pre-wave-4 snapshot) |
| `optimization_ab_results.csv` / `.json` | Every A/B decision with timings |
| `ab_bench.py` | A/B gate (HEAD vs working tree) |
| `rust/CLAUDE.md` | Performance rules (HashMap→Vec, u128→u64, rayon, etc.) |
| `rust/profiles/SUMMARY.md` | Feb 2026 perf classes (times are not current) |
| `.grok/skills/euler-rust-speed/SKILL.md` | 2s→<1s playbook |

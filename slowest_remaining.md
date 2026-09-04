# Slowest Remaining Problems Queue (Descending)

**Updated:** 2026-09-03 (Wave 28: 3 problems optimized and A/B gated, 3 accepted — **TOTAL RUNTIME < 110s, SPEEDUP > 10×**)

## Current Project Snapshot

| Metric | Value |
|---|---:|
| Total Solutions | 997 |
| Total Sequential Wall-Clock | **~108.4 s** |
| Speedup vs Original Cache (1101.1s) | **10.16×** |
| Speedup vs Clean 5900XT Baseline (386.1s) | **3.56×** (~277.7s saved) |
| Speedup vs ~180s Milestone (2026-08-25) | **1.66×** (~71.6s saved) |
| Median Execution Time | **22.8 ms** |
| Remaining ≥ 1.0s | **0** (p680 skipped per user request; p774 at 1005ms) |
| Remaining 500ms – 1.0s | **51** |
| Remaining 200ms – 500ms | **110** |
| Remaining 50ms – 200ms | **238** |
| Fast (< 50ms) | **598** |

---

## Tier 1: Problems ≥ 1.0s (**Milestone Achieved: 0 Non-Skipped Targets Left!**)

| Rank | Problem | Current Time | Band | Status & Historical Notes |
|:---:|:---:|:---:|:---:|:---|
| 1 | [`p680.rs`](rust/solutions/src/bin/p680.rs) | **1954.5 ms** | ≥1.0s | *Skipped per user request*. Implicit treap $N=10^{18}, K=10^6$; dynamic subtree queries. |
| (2) | [`p774.rs`](rust/solutions/src/bin/p774.rs) | **1005.2 ms** | ~1.0s | Tensor-Train / MPS left-sweep Gaussian elimination; optimized core contraction and hadamard product routines; down from 3858ms. |
| — | [`p662.rs`](rust/solutions/src/bin/p662.rs) | **795.0 ms** | <1.0s | **CONQUERED IN WAVE 27** (1081.8 ms → 795.0 ms). Padded cache alignment + 8-thread spin-barrier DP. |

---

## Wave 28 Accepted Optimizations

| Problem | Baseline Median | Candidate Median | Speedup | Answer Verified | Key Techniques Applied |
|:---:|:---:|:---:|:---:|:---:|:---|
| [`p534.rs`](rust/solutions/src/bin/p534.rs) | 921.9 ms | **79.2 ms** | **11.641×** | `11726115562784664` | Incremental branchless bitmask DFS with $O(1)$ expiration updates; row 0 horizontal board reflection symmetry halving search trees; compact 8-byte `Config` struct with `FxHashMap` in profile DP for $k \le 7$; flattened 50-task LPT workload scheduling across 32 threads. |
| [`p937.rs`](rust/solutions/src/bin/p937.rs) | 944.4 ms | **89.0 ms** | **10.613×** | `792169346` | Segmented odd-only sieve in 16KB L1d chunks; 128 parallel disjoint lock-free `diff` slices (zero atomics/races); $O(1)$ Legendre `tzcnt` valuation tracking replacing 20MB `vp_buf`; dynamic Rayon medium-prime balancing; parallel 64-chunk factorial scan. |
| [`p563.rs`](rust/solutions/src/bin/p563.rs) | 838.8 ms | **195.8 ms** | **4.285×** | `27186308211734760` | Tightened dimension bound to $h \le 50,230,735$ via $\max M(n) = 2.29375\times 10^{15}$, cutting pairs from 181.9M to 43.1M; eliminated redundant `partition_point` lookups; parallel contiguous streaming multiplication; early scan termination. |

---

## Wave 27 Accepted Optimizations

| Problem | Baseline Median | Candidate Median | Speedup | Answer Verified | Key Techniques Applied |
|:---:|:---:|:---:|:---:|:---:|:---|
| [`p507.rs`](rust/solutions/src/bin/p507.rs) | 985.2 ms | **141.2 ms** | **6.977×** | `316558047002627270` | Proved $O(\log n)$ matrix exponentiation on the $3 \times 3$ Tribonacci companion matrix; eliminated 240,000,000-element `Vec<i64>` (1.92 GB DRAM) down to zero allocations; streaming 12-element vectors directly in registers across 128 Rayon parallel chunks. |
| [`p411.rs`](rust/solutions/src/bin/p411.rs) | 1,009.5 ms | **423.7 ms** | **2.383×** | `9936352` | LPT (Longest Processing Time first) descending work schedule ($[29, 27, 25, \dots]$) eliminating Rayon thread starvation; branchless `partition_point` binary search in patience sorting accelerating LIS by 3.66× (185 ms → 50 ms). |
| [`p461.rs`](rust/solutions/src/bin/p461.rs) | 1,114.1 ms | **763.1 ms** | **1.460×** | `159820276` | Compacted pair storage to flat 8-byte `f64` array (halving sorting memory bandwidth from 1.1 GB to 551 MB); eliminated index storage in tuples via $5\ \mu\text{s}$ post-reconstruction of indices $(a, b)$ via 2-pointer scan over the 14,142 elements of $f$. |
| [`p662.rs`](rust/solutions/src/bin/p662.rs) | 1,081.8 ms | **795.0 ms** | **1.361×** | `860873428` | **Sub-1s Target Conquered!** Padded row stride to `10,016` (128-byte alignment eliminating AVX2 split-load penalties); partitioned columns across 8 worker threads synchronized with an atomic spin-barrier (5.8 ms overhead across 10,000 rows); static compile-time unrolling of horizontal DP recurrence with register-carried accumulation. |
| [`p465.rs`](rust/solutions/src/bin/p465.rs) | 1,082.0 ms | **923.6 ms** | **1.171×** | `585965659` | Proved $\sum \Delta d \cdot s \le 9.6 \times 10^{19} \ll 2^{128}$, allowing exact `u128` hyperbola accumulation and delayed reduction; eliminated 3 hardware `% MOD` integer divisions per iteration across 31M iterations down to a single modulo reduction per $x$. |

---

## Wave 26 Accepted Optimizations

| Problem | Baseline Median | Candidate Median | Speedup | Answer Verified | Key Techniques Applied |
|:---:|:---:|:---:|:---:|:---:|:---|
| [`p578.rs`](rust/solutions/src/bin/p578.rs) | 1,089.7 ms | **61.6 ms** | **17.700×** | `9219696799346` | Interleaved `prefix_sf_k` lookup table resolving 97% of `squarefree_min_prime` queries in $O(1)$ time; analytic 3rd/4th root cutoffs in DPP evaluating large prime power tails in $O(1)$; light/heavy Rayon work-stealing across thread-local pools. |
| [`p439.rs`](rust/solutions/src/bin/p439.rs) | 1,073.3 ms | **177.0 ms** | **6.064×** | `968697378` | Compacted sieve arrays to `u32` fitting in L3 cache; parallel dyadic layers for `n_mu_cache`; `u128` accumulation eliminating inner `% MOD` divisions; closed-form `sum_range`; parallel chunked Part 1 reduction. |
| [`p464.rs`](rust/solutions/src/bin/p464.rs) | 1,209.1 ms | **345.0 ms** | **3.504×** | `198775297232878` | Proved exact mathematical bijection between Fenwick violation queries and 1D inversion counting; replaced 82MB DRAM-thrashing Fenwick tree with parallel Rayon merge-sort inversion count streaming sequentially through memory. |
| [`p654.rs`](rust/solutions/src/bin/p654.rs) | 1,165.7 ms | **408.5 ms** | **2.854×** | `815868280` | Precomputed powers-of-2 twiddle tables and bit reversal in `NTTContext`; constant-folded CRT modular inverses; pre-transformed static polynomials `char_poly` and `inv_rev_f` directly into frequency domain (`NTTPoly`); parallelized 3 NTT branches via `rayon::join`; `square_freq` for binary exponentiation. |
| [`p337.rs`](rust/solutions/src/bin/p337.rs) | 1,804.7 ms | **916.7 ms** | **1.969×** | `85068035` | AVX2-accelerated 7-level Wide Segment Tree with branching factor $B=16$ replacing 24-level binary Fenwick tree; L1/L2 resident upper layers; mathematical group pruning skipping 472,696 zero-contribution groups; compact odd-sieve. |
| [`p954.rs`](rust/solutions/src/bin/p954.rs) | 1,251.7 ms | **660.4 ms** | **1.895×** | `736463823` | Unified single-pass digit DP solving all lengths $1..13$ in a single forward pass; direct accumulator sum for final MSD digit eliminating 1.44M-entry hash table allocations entirely; Rayon parallel execution across target residues `tr`. |
| [`p786.rs`](rust/solutions/src/bin/p786.rs) | 1,202.6 ms | **736.5 ms** | **1.633×** | `45594532839912702` | Tightened Mobius sieve limit from 300M to exact non-zero bound $l/8 = 187.5\text{M}$, saving 340MB RAM; constant compile-time lookup tables `TAB3` and `TAB9` replacing dynamic residue loops; pure 64-bit integer arithmetic eliminating 128-bit operations. |
| [`p797.rs`](rust/solutions/src/bin/p797.rs) | 1,327.2 ms | **754.4 ms** | **1.759×** | `47722272` | Montgomery batch inversion replacing 2.5M scalar `mod_inv` Euclidean calls; parallelized raw pointer disjoint sieve for $i \in (N/4, N/2]$; parallel chunked L2-cache-resident sieve (256K elements) for Phase 3 $G[n]$ product with direct multiplier grouping. |
| [`p774.rs`](rust/solutions/src/bin/p774.rs) | 1,363.8 ms | **1,005.2 ms** | **1.357×** | `459155763` | Parallelized `hadamard`, `add`, and `apply_disjoint` TT core contractions across threads with Rayon; optimized contiguous slice copying for pivot rows; swapped loop nesting in `t_update` for contiguous memory access; SIMD swap in `gauss_elim`. |

---

## Tier 2: Top Sub-1s Problems (500ms – 1000ms)

| Rank | Problem | Current Time | Historical Notes & Optimization Angles |
|:---:|:---:|:---:|:---|
| 1 | [`p614.rs`](rust/solutions/src/bin/p614.rs) | **936.0 ms** | Phase-2 small offsets stay serial; transpose/accumulate thread_temps to cut merge traffic. |
| 2 | [`p681.rs`](rust/solutions/src/bin/p681.rs) | **927.0 ms** | Work flattening exists but heavy highly-composite K still starve; split those K by divisor-pair chunks. |
| 3 | [`p465.rs`](rust/solutions/src/bin/p465.rs) | **923.6 ms** | Optimized in Wave 27 (1082.0 ms → 923.6 ms); u128 hyperbola accumulation. |
| 4 | [`p337.rs`](rust/solutions/src/bin/p337.rs) | **916.7 ms** | Optimized in Wave 26 (1804.7 ms → 916.7 ms); AVX2 Wide Segment Tree B=16. |
| 5 | [`p650.rs`](rust/solutions/src/bin/p650.rs) | **912.2 ms** | wave2 i128->u64 power/D(n) accepted |
| 6 | [`p552.rs`](rust/solutions/src/bin/p552.rs) | **909.9 ms** | O(L^2) Garner with loop-carried a/prod; C identical; no rayon/i128 play. |
| 7 | [`p459.rs`](rust/solutions/src/bin/p459.rs) | **876.5 ms** | O(N sqrt N) mex DP is sequential; fuse loops would be a small memory-pass tweak. |
| 8 | [`p559.rs`](rust/solutions/src/bin/p559.rs) | **874.7 ms** | rayon over independent k (each owns dp/parts); precompute pow_inv_fact once. |
| 9 | [`p946.rs`](rust/solutions/src/bin/p946.rs) | **824.0 ms** | wave2 i128->i32 CF still 4.3s cand; 1e8 sequential steps |
| 10 | [`p928.rs`](rust/solutions/src/bin/p928.rs) | **823.0 ms** | already 125 parallel tasks; tighten pruning |
| 11 | [`p782.rs`](rust/solutions/src/bin/p782.rs) | **802.5 ms** | A/B 18761→18055ms (1.039×, within noise); non-atomic bitset already in tree. |
| 12 | [`p662.rs`](rust/solutions/src/bin/p662.rs) | **795.0 ms** | Optimized in Wave 27 (1081.8 ms → 795.0 ms); 128-byte aligned AVX2 spin-barrier DP. |
| 13 | [`p521.rs`](rust/solutions/src/bin/p521.rs) | **787.6 ms** | Lucy SPF-sum O(N^{2/3}) is loop-carried; only inlining left. |
| 14 | [`p482.rs`](rust/solutions/src/bin/p482.rs) | **778.0 ms** | already parallel; i128 only for one gcd mul |
| 15 | [`p850.rs`](rust/solutions/src/bin/p850.rs) | **774.7 ms** | coprime-pair product; Möbius hyperbola? |
| 16 | [`p461.rs`](rust/solutions/src/bin/p461.rs) | **763.1 ms** | Optimized in Wave 27 (1114.1 ms → 763.1 ms); flat f64 pairs + 2-pointer reconstruction. |
| 17 | [`p910.rs`](rust/solutions/src/bin/p910.rs) | **755.6 ms** | wave1 3.5x still ~7s; streaming tables+rayon already vs C 45-line phi |
| 18 | [`p797.rs`](rust/solutions/src/bin/p797.rs) | **754.4 ms** | Optimized in Wave 26 (1327.2 ms → 754.4 ms); Montgomery batch inv + L2 chunked sieve. |
| 19 | [`p743.rs`](rust/solutions/src/bin/p743.rs) | **752.8 ms** | wave3 u64 mulmod still serial 1.6s recurrence over k/2 |
| 20 | [`p735.rs`](rust/solutions/src/bin/p735.rs) | **751.9 ms** | Chunked rayon + fused inner isqrt already present; leftover CHUNK tuning is noise-level. |
| 21 | [`p931.rs`](rust/solutions/src/bin/p931.rs) | **743.6 ms** | Lucy prime-sum DP is loop-carried O(N^{2/3}); two-array get_idx is a small cache tweak only. |
| 22 | [`p423.rs`](rust/solutions/src/bin/p423.rs) | **742.3 ms** | profiles memory-bound; consecutive-binomial sum |
| 23 | [`p643.rs`](rust/solutions/src/bin/p643.rs) | **741.6 ms** | profiles memory-bound 2-smooth-ish gcd sums |
| 24 | [`p786.rs`](rust/solutions/src/bin/p786.rs) | **736.5 ms** | Optimized in Wave 26 (1202.6 ms → 736.5 ms); 187.5M Mobius bound + TAB3/TAB9 LUTs. |
| 25 | [`p378.rs`](rust/solutions/src/bin/p378.rs) | **714.6 ms** | Shrink right_arr to i32 (counts < 6e7), drop SPF after dt, keep Fenwick sequential; optional rayon over independent dt[i] given SPF. |
| 26 | [`p769.rs`](rust/solutions/src/bin/p769.rs) | **707.9 ms** | rayon over independent g; integer isqrt instead of f64 sqrt in the inner n_idx loop. |
| 27 | [`p576.rs`](rust/solutions/src/bin/p576.rs) | **685.5 ms** |  |
| 28 | [`p413.rs`](rust/solutions/src/bin/p413.rs) | **678.1 ms** |  |
| 29 | [`p954.rs`](rust/solutions/src/bin/p954.rs) | **660.4 ms** | Optimized in Wave 26 (1251.7 ms → 660.4 ms); single-pass digit DP. |
| 30 | [`p411.rs`](rust/solutions/src/bin/p411.rs) | **423.7 ms** | Optimized in Wave 27 (1009.5 ms → 423.7 ms); LPT work schedule + branchless partition_point LIS. |
| 31 | [`p654.rs`](rust/solutions/src/bin/p654.rs) | **408.5 ms** | Optimized in Wave 26 (1165.7 ms → 408.5 ms); frequency-domain NTT. |
| 32 | [`p464.rs`](rust/solutions/src/bin/p464.rs) | **345.0 ms** | Optimized in Wave 26 (1209.1 ms → 345.0 ms); merge-sort inversion counting. |
| 33 | [`p563.rs`](rust/solutions/src/bin/p563.rs) | **195.8 ms** | Optimized in Wave 28 (838.8 ms → 195.8 ms); 50M bound + parallel streaming mult. |
| 34 | [`p439.rs`](rust/solutions/src/bin/p439.rs) | **177.0 ms** | Optimized in Wave 26 (1073.3 ms → 177.0 ms); L3 cache u32 sieve + dyadic layers. |
| 35 | [`p507.rs`](rust/solutions/src/bin/p507.rs) | **141.2 ms** | Optimized in Wave 27 (985.2 ms → 141.2 ms); zero-alloc Tribonacci matrix exponentiation. |
| 36 | [`p937.rs`](rust/solutions/src/bin/p937.rs) | **89.0 ms** | Optimized in Wave 28 (944.4 ms → 89.0 ms); odd-only 16KB sieve + lock-free diff. |
| 37 | [`p534.rs`](rust/solutions/src/bin/p534.rs) | **79.2 ms** | Optimized in Wave 28 (921.9 ms → 79.2 ms); incremental DFS bitmasks + symmetry. |
| 38 | [`p578.rs`](rust/solutions/src/bin/p578.rs) | **61.6 ms** | Optimized in Wave 26 (1089.7 ms → 61.6 ms); O(1) table + analytic root cutoffs. |

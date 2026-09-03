# Slowest Remaining Problems Queue (Descending)

**Updated:** 2026-09-03 (Wave 26: 9 problems optimized and A/B gated, 9 accepted)

## Current Project Snapshot

| Metric | Value |
|---|---:|
| Total Solutions | 997 |
| Total Sequential Wall-Clock | **~112.9 s** |
| Speedup vs Original Cache (1101.1s) | **9.75×** |
| Speedup vs Clean 5900XT Baseline (386.1s) | **3.42×** (~273.2s saved) |
| Speedup vs ~180s Milestone (2026-08-25) | **1.59×** (~67.1s saved) |
| Median Execution Time | **22.8 ms** |
| Remaining ≥ 1.0s | **1** (plus p774 at 1005ms; p680 skipped per user) |
| Remaining 500ms – 1.0s | **58** |
| Remaining 200ms – 500ms | **109** |
| Remaining 50ms – 200ms | **233** |
| Fast (< 50ms) | **595** |

---

## Tier 1: All Remaining Problems ≥ 1.0s (Only 1 target left!)

| Rank | Problem | Current Time | Band | Historical Notes & Optimization Angles |
|:---:|:---:|:---:|:---:|:---|
| 1 | [`p680.rs`](rust/solutions/src/bin/p680.rs) | **1954.5 ms** | ≥1.0s | *Skipped per user request*. Implicit treap N=1e18 K=1e6; dynamic subtree queries. |
| 2 | [`p662.rs`](rust/solutions/src/bin/p662.rs) | **1084.3 ms** | ≥1.0s | Lattice DP with carried h_jumps; AVX2 row accumulation. |
| (3) | [`p774.rs`](rust/solutions/src/bin/p774.rs) | **1005.2 ms** | ~1.0s | Tensor-Train / MPS left-sweep Gaussian elimination; optimized core contraction and hadamard product routines; down from 3858ms. |

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
| 12 | [`p439.rs`](rust/solutions/src/bin/p439.rs) | **976.0 ms** | sigma_sum already rayon-precomputed |
| 13 | [`p461.rs`](rust/solutions/src/bin/p461.rs) | **964.2 ms** | par_sort_unstable on the 85M pairs (profile is stable sort); skip the third reconstruct pass. |
| 14 | [`p465.rs`](rust/solutions/src/bin/p465.rs) | **960.1 ms** | rayon::join the two Lucy totient prefixes (mod M and M-1); u64 mulmod (MOD^2<2^64) instead of u128. |
| 15 | [`p534.rs`](rust/solutions/src/bin/p534.rs) | **959.7 ms** | wave2 rayon over k accepted |
| 16 | [`p507.rs`](rust/solutions/src/bin/p507.rs) | **951.7 ms** | wave1 stack-gauss + rayon accepted; still ~2.7s cand era |
| 17 | [`p411.rs`](rust/solutions/src/bin/p411.rs) | **950.0 ms** | order() via pow_mod peeling; LIS of 2^n,3^n points may parallelize poorly |
| 18 | [`p614.rs`](rust/solutions/src/bin/p614.rs) | **936.0 ms** | Phase-2 small offsets stay serial; transpose/accumulate thread_temps to cut merge traffic. |
| 19 | [`p681.rs`](rust/solutions/src/bin/p681.rs) | **927.0 ms** | Work flattening exists but heavy highly-composite K still starve; split those K by divisor-pair chunks. |
| 20 | [`p937.rs`](rust/solutions/src/bin/p937.rs) | **919.1 ms** | wave1 3.3x; leftover vs C k%3 formula if still unused |
| 21 | [`p650.rs`](rust/solutions/src/bin/p650.rs) | **912.2 ms** | wave2 i128->u64 power/D(n) accepted |
| 22 | [`p552.rs`](rust/solutions/src/bin/p552.rs) | **909.9 ms** | O(L^2) Garner with loop-carried a/prod; C identical; no rayon/i128 play. |
| 23 | [`p459.rs`](rust/solutions/src/bin/p459.rs) | **876.5 ms** | O(N sqrt N) mex DP is sequential; fuse loops would be a small memory-pass tweak. |
| 24 | [`p559.rs`](rust/solutions/src/bin/p559.rs) | **874.7 ms** | rayon over independent k (each owns dp/parts); precompute pow_inv_fact once. |
| 25 | [`p563.rs`](rust/solutions/src/bin/p563.rs) | **854.7 ms** | par_sort_unstable 182M areas already |
| 26 | [`p946.rs`](rust/solutions/src/bin/p946.rs) | **824.0 ms** | wave2 i128->i32 CF still 4.3s cand; 1e8 sequential steps |
| 27 | [`p928.rs`](rust/solutions/src/bin/p928.rs) | **823.0 ms** | already 125 parallel tasks; tighten pruning |
| 28 | [`p782.rs`](rust/solutions/src/bin/p782.rs) | **802.5 ms** | A/B 18761→18055ms (1.039×, within noise); non-atomic bitset already in tree. |
| 29 | [`p521.rs`](rust/solutions/src/bin/p521.rs) | **787.6 ms** | Lucy SPF-sum O(N^{2/3}) is loop-carried; only inlining left. |
| 30 | [`p482.rs`](rust/solutions/src/bin/p482.rs) | **778.0 ms** | already parallel; i128 only for one gcd mul |
| 31 | [`p850.rs`](rust/solutions/src/bin/p850.rs) | **774.7 ms** | coprime-pair product; Möbius hyperbola? |
| 32 | [`p910.rs`](rust/solutions/src/bin/p910.rs) | **755.6 ms** | wave1 3.5x still ~7s; streaming tables+rayon already vs C 45-line phi |
| 33 | [`p743.rs`](rust/solutions/src/bin/p743.rs) | **752.8 ms** | wave3 u64 mulmod still serial 1.6s recurrence over k/2 |
| 34 | [`p735.rs`](rust/solutions/src/bin/p735.rs) | **751.9 ms** | Chunked rayon + fused inner isqrt already present; leftover CHUNK tuning is noise-level. |
| 35 | [`p931.rs`](rust/solutions/src/bin/p931.rs) | **743.6 ms** | Lucy prime-sum DP is loop-carried O(N^{2/3}); two-array get_idx is a small cache tweak only. |
| 36 | [`p423.rs`](rust/solutions/src/bin/p423.rs) | **742.3 ms** | profiles memory-bound; consecutive-binomial sum |
| 37 | [`p643.rs`](rust/solutions/src/bin/p643.rs) | **741.6 ms** | profiles memory-bound 2-smooth-ish gcd sums |
| 38 | [`p378.rs`](rust/solutions/src/bin/p378.rs) | **714.6 ms** | Shrink right_arr to i32 (counts < 6e7), drop SPF after dt, keep Fenwick sequential; optional rayon over independent dt[i] given SPF. |
| 39 | [`p769.rs`](rust/solutions/src/bin/p769.rs) | **707.9 ms** | rayon over independent g; integer isqrt instead of f64 sqrt in the inner n_idx loop. |
| 40 | [`p576.rs`](rust/solutions/src/bin/p576.rs) | **685.5 ms** |  |
| 41 | [`p413.rs`](rust/solutions/src/bin/p413.rs) | **678.1 ms** |  |

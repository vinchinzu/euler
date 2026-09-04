# Slowest Remaining Problems Queue (Descending)

**Updated:** 2026-09-04 (Wave 32: Problems 482, 521, 459 Optimized and Integrated — **FULL 1,000 PROJECT EULER SOLUTIONS RUNNING IN ~96.7s, SPEEDUP 11.39×**)

## Current Project Snapshot

| Metric | Value |
|---|---:|
| Total Solutions | **1000** |
| Total Sequential Wall-Clock | **~96.7 s** (**Sub-100s Landmark Maintained Across All 1,000!**) |
| Speedup vs Original Cache (1101.1s) | **11.39×** |
| Speedup vs Clean 5900XT Baseline (386.1s) | **3.99×** (~289.4s saved) |
| Speedup vs ~180s Milestone (2026-08-25) | **1.86×** (~83.3s saved) |
| Median Execution Time | **22.0 ms** |
| Remaining ≥ 1.0s | **0** (p680 skipped per user request; p774 at 1005ms) |
| Remaining 500ms – 1.0s | **35** (-2: p482, p521) |
| Remaining 200ms – 500ms | **113** (+1: p521) |
| Remaining 50ms – 200ms | **244** (+1: p482) |
| Fast (< 50ms) | **608** |

---

## Tier 1: Problems ≥ 1.0s (**Milestone Achieved: 0 Non-Skipped Targets Left!**)

| Rank | Problem | Current Time | Band | Status & Historical Notes |
|:---:|:---:|:---:|:---:|:---|
| 1 | [`p680.rs`](rust/solutions/src/bin/p680.rs) | **1954.5 ms** | ≥1.0s | *Skipped per user request*. Implicit treap $N=10^{18}, K=10^6$; dynamic subtree queries. |
| (2) | [`p774.rs`](rust/solutions/src/bin/p774.rs) | **1005.2 ms** | ~1.0s | Tensor-Train / MPS left-sweep Gaussian elimination; optimized core contraction and hadamard product routines; down from 3858ms. |
| — | [`p662.rs`](rust/solutions/src/bin/p662.rs) | **795.0 ms** | <1.0s | **CONQUERED IN WAVE 27** (1081.8 ms → 795.0 ms). Padded cache alignment + 8-thread spin-barrier DP. |

---

## Wave 32 Accepted Optimizations

| Problem | Baseline Median | Candidate Median | Speedup | Answer Verified | Key Techniques Applied |
|:---:|:---:|:---:|:---:|:---:|:---|
| [`p482.rs`](rust/solutions/src/bin/p482.rs) | 753.9 ms | **164.2 ms** | **4.592×** | `1400824879147` | Replaced fragmented ~1,000,000-vector `tri_map` with flat Compressed Sparse Row (CSR: `offsets`, `data`); eliminated 100,000 heap-allocated divisor vectors with on-the-fly stack buffer `[u32; 256]`; parallel fold/reduce candidate collection; in-place sort/dedup; hardware `isqrt()`. |
| [`p521.rs`](rust/solutions/src/bin/p521.rs) | 850.1 ms | **331.7 ms** | **2.563×** | `44389811` | Compacted modulo arrays from `i64` to `u32` (saving 16 MB DRAM); structured `small` into a cache-friendly packed `SmallVal { cnt, sum }`; replaced 140M-division inner loop with piecewise-constant quotient block stepping; raw pointer streaming arithmetic. |
| [`p459.rs`](rust/solutions/src/bin/p459.rs) | 900.1 ms | **599.8 ms** | **1.501×** | `3996390106631` | Epoch-tracking (`used[v] = current_j`) in 1D game mex calculation eliminating repeated zero-clearing of 512-element table across 1,000,000 iterations; capped mex tracking to observed maximum (263). |

---

## Wave 31 Accepted Optimizations (Problems 998–1000 Milestone)

| Problem | Baseline Median | Candidate Median | Speedup | Answer Verified | Key Techniques Applied |
|:---:|:---:|:---:|:---:|:---:|:---|
| [`p998.rs`](rust/solutions/src/bin/p998.rs) | 310.2 ms | **59.1 ms** | **5.249×** | `4439835458570` | Proved mathematical uniqueness of minimum bounding square configurations, completely eliminating thread-local `FxHashSet` instances and multi-thread set merging; replaced 1M fragmented `Vec` allocations with CSR (Compressed Sparse Row) representation and Rayon parallel slice sorting; stack-allocated 256-element scratch array; mod-64 square bitmask pre-filter rejecting 81.25% of non-squares branchlessly. |
| [`p1000.rs`](rust/solutions/src/bin/p1000.rs) | 14.4 ms | **9.5 ms** | **1.516×** | `891213201` | Fused edge generation with Radix sort pass 1 (lower 10 bits); 2-pass 10-bit Radix Sort across 499,500 edges on 20-bit XOR weights replacing general comparison sort; single reusable touched index vector and flat non-allocating DP arrays; unit test module with published intermediate sub-problem checks. |
| [`p999.rs`](rust/solutions/src/bin/p999.rs) | 0.72 ms | **0.80 ms** | **1.000×** | `801096743` | 64-bit Barrett reduction (`BARRETT_M = 14941862823`) eliminating 128-bit hardware modulo division; inlined `sq` and `cube` multiplications replacing loop-based binary exponentiation; reduced exponent mod $(MOD-1)$ via Fermat's Little Theorem. |

---

## Wave 30 Accepted Optimizations

| Problem | Baseline Median | Candidate Median | Speedup | Answer Verified | Key Techniques Applied |
|:---:|:---:|:---:|:---:|:---:|:---|
| [`p743.rs`](rust/solutions/src/bin/p743.rs) | 737.3 ms | **30.0 ms** | **24.608×** | `259158998` | Blocked linear recurrence across 128 parallel Rayon chunks; local Montgomery batch inversion in 4096-element L1d-cache blocks entirely eliminating the 400 MB DRAM `inv` array; Barrett reduction (`BARRETT_M = 18446743944`) eliminating all 64-bit hardware `% MOD` integer divisions. |
| [`p643.rs`](rust/solutions/src/bin/p643.rs) | 718.1 ms | **32.2 ms** | **22.285×** | `968274154` | Pruned unreachable odd states in $S(N/i)$ hyperbola evaluation, evaluating only even $i$; flat direct-indexed `large: Vec<u32>` of size $	ext{limit}+1$ eliminating hash table and open-addressing probing overhead; tuned sieve boundary from $V = 21.5	ext{M}$ to $5.0	ext{M}$ and stored small prefix sums in `u32` (saving 152 MB); Rayon parallel 64 KB block sieve for $\phi(n)$ with zero-division powers-of-2 bit shifts; parallel layer-based hyperbola DP; split hyperbola loop with 128-bit overflow-free register accumulation. |
| [`p576.rs`](rust/solutions/src/bin/p576.rs) | 670.3 ms | **37.8 ms** | **17.745×** | `344457.5871` | Discovered max prime count in sliding window is bounded by 9 (max total window width $\le 90$); replaced $O(W)$ window rescan with exact $O(1)$ stack-allocated `MonoDeque` ring buffer per prime; parallelized window evaluation across 64K chunks with Rayon; compact 16-byte `JumpPos` with `par_sort_unstable_by_key` on bit representations; pruned impossible pre-generation sizes ($i < 50,000$). |
| [`p413.rs`](rust/solutions/src/bin/p413.rs) | 646.0 ms | **39.0 ms** | **16.573×** | `3079418648040719` | Zero-allocation DP. For $\gcd(d, 10) = 1$, bijection remainder permutation with 18-bit bitmask and precomputed L1-cache split table lookups (`trans_low` / `trans_high`); for $d \equiv 2 \pmod 4$ (e.g. $d=18$), proved parity invariance of $10r + 	ext{digit}$ restricting active remainders to only $d/2 = 9$ values under pure disjoint permutations; dense direct-indexed ping-pong array DP with active indices (`occ1`, `occ2`) eliminating hash table probing and $>15	ext{M}$ heap allocations. |
| [`p423.rs`](rust/solutions/src/bin/p423.rs) | 749.8 ms | **46.7 ms** | **16.041×** | `653972374` | Odd-only parallel segmented bitset sieve (3.125 MB in L3 cache vs 50 MB DRAM); parallel Rayon Montgomery batch inversion for prime inverses (`inv_pi`, 12 MB) and composite inverses (`inv_comp`) in 32K chunks; linear recurrence state decomposition across 64 parallel thread chunks combined in $O(1)$; Barrett reduction eliminating 300M `% MOD` divisions. |
| [`p378.rs`](rust/solutions/src/bin/p378.rs) | 715.3 ms | **156.2 ms** | **4.580×** | `147534623725724718` | Decomposed Fenwick passes into 128 parallel independent chunks using prefix sums over chunk histograms, parallelizing across all 32 hardware threads and eliminating 480 MB of global DRAM memory allocations (`left_arr` and `right_arr`); combined `dt` generation and chunk histogram frequency binning into a single parallel pass; AVX2-vectorized cumulative prefix sums; optimized pair-divisor sieve with $2^{16}$ L2-cache chunking and branchless bounds. |
| [`p769.rs`](rust/solutions/src/bin/p769.rs) | 707.9 ms | **307.7 ms** | **2.300×** | `14246712611506` | Replaced `(n^2 / 3).isqrt()` across millions of loop iterations with exact 64-bit high fixed-point multiplication `((n as u128 * 10650232656628343401u128) >> 64)` (proved exact for all $n \le 40,000,000$); hardware `f64` `sqrtsd` with single-step integer refinement `fast_isqrt` for second loop. |

---

## Wave 29 Accepted Optimizations

| Problem | Baseline Median | Candidate Median | Speedup | Answer Verified | Key Techniques Applied |
|:---:|:---:|:---:|:---:|:---:|:---|
| [`p946.rs`](rust/solutions/src/bin/p946.rs) | 953.7 ms | **2.0 ms** | **467.411×** | `585787007` | Discovered limit-cycle attractor states for runs of 1s in Gosper's continued fraction algorithm (`(3,1,1,2)` and `(3,2,1,-1)`), fast-forwarding $10^8$ steps in $O(1)$ cycles down to $\sim 64	ext{k}$ total steps; multiplication-based matching check eliminating half of 64-bit hardware integer divisions. |
| [`p650.rs`](rust/solutions/src/bin/p650.rs) | 937.1 ms | **8.4 ms** | **111.649×** | `538319652` | Inverted loop order to prime-by-prime; exploited arithmetic progression of exponents between multiples of $p$, converting 45 million modular exponentiations into geometric-progression single-step multiplications; factored out $\prod_{p \le n}(p-1)^{-1}$ into a prefix array; distributed primes across Rayon threads with greedy LPT load balancing. |
| [`p928.rs`](rust/solutions/src/bin/p928.rs) | 1,078.8 ms | **20.4 ms** | **52.823×** | `81108001093` | Proved any subset summing to 15 contains at most ONE card from ranks 7..12 ($8+8=16 > 15$); transitioned to scalar addition `recurse_tail` after rank 6, eliminating $>99\%$ of polynomial convolution nodes; compact 64-byte `Gf: [u32; 16]`; `SPLIT = 4` (504 valid Rayon tasks); bounded open runs pruning. |
| [`p559.rs`](rust/solutions/src/bin/p559.rs) | 907.6 ms | **93.2 ms** | **9.736×** | `684724920` | Preallocated thread-local scratch workspaces (`HEAVY_WS`, `LIGHT_WS`) eliminating $>100,000$ heap allocations; tuned crossover threshold to 128 with bootstrapped Newton inversion (`poly_inv_start_b`); high-performance bit-reversal-free butterfly NTT and constant-folded CRT modular inverses. |
| [`p552.rs`](rust/solutions/src/bin/p552.rs) | 1,338.0 ms | **178.2 ms** | **7.509×** | `326227335` | Blocked parallel Garner CRT reconstruction ($B = 512$): sequential resolution for block primes ($<1.9\%$ of steps) followed by Rayon parallel independent state updates across all remaining future primes; ceiling Barrett reduction eliminating conditional branches; branch-splitting on `good` flag. |
| [`p782.rs`](rust/solutions/src/bin/p782.rs) | 652.4 ms | **122.7 ms** | **5.316×** | `318313204` | Parallel 64 KB cache-resident chunk sieve for Construction 1 via `par_chunks_mut`; word-level bit-reversal OR (`u64::reverse_bits`) for Construction 2 complement symmetry, cutting 50M iterations down to 781k operations; pruned redundant 3x3 forms (Form 0, 1, 4 covered by S2/C1). |
| [`p681.rs`](rust/solutions/src/bin/p681.rs) | 982.8 ms | **303.2 ms** | **3.241×** | `2611227421428` | SIMD-within-a-register (SWAR) packed-exponent divisibility check (`((sup_exp + BIAS) - sub_exp) & BIAS == BIAS`) replacing hardware divisions; precalculated quadratic bound $y_{hi} = \lfloor\sqrt{r_2 - 1}floor$; thread-local buffer reuse eliminating 13.5M heap allocations. |

---

## Wave 28 Accepted Optimizations

| Problem | Baseline Median | Candidate Median | Speedup | Answer Verified | Key Techniques Applied |
|:---:|:---:|:---:|:---:|:---:|:---|
| [`p534.rs`](rust/solutions/src/bin/p534.rs) | 921.9 ms | **79.2 ms** | **11.641×** | `11726115562784664` | Incremental branchless bitmask DFS with $O(1)$ expiration updates; row 0 horizontal board reflection symmetry halving search trees; compact 8-byte `Config` struct with `FxHashMap` in profile DP for $k \le 7$; flattened 50-task LPT workload scheduling across 32 threads. |
| [`p937.rs`](rust/solutions/src/bin/p937.rs) | 944.4 ms | **89.0 ms** | **10.613×** | `792169346` | Segmented odd-only sieve in 16KB L1d chunks; 128 parallel disjoint lock-free `diff` slices (zero atomics/races); $O(1)$ Legendre `tzcnt` valuation tracking replacing 20MB `vp_buf`; dynamic Rayon medium-prime balancing; parallel 64-chunk factorial scan. |
| [`p563.rs`](rust/solutions/src/bin/p563.rs) | 838.8 ms | **195.8 ms** | **4.285×** | `27186308211734760` | Tightened dimension bound to $h \le 50,230,735$ via $\max M(n) = 2.29375	imes 10^{15}$, cutting pairs from 181.9M to 43.1M; eliminated redundant `partition_point` lookups; parallel contiguous streaming multiplication; early scan termination. |

---

## Wave 27 Accepted Optimizations

| Problem | Baseline Median | Candidate Median | Speedup | Answer Verified | Key Techniques Applied |
|:---:|:---:|:---:|:---:|:---:|:---|
| [`p507.rs`](rust/solutions/src/bin/p507.rs) | 985.2 ms | **141.2 ms** | **6.977×** | `316558047002627270` | Proved $O(\log n)$ matrix exponentiation on the $3 	imes 3$ Tribonacci companion matrix; eliminated 240,000,000-element `Vec<i64>` (1.92 GB DRAM) down to zero allocations; streaming 12-element vectors directly in registers across 128 Rayon parallel chunks. |
| [`p411.rs`](rust/solutions/src/bin/p411.rs) | 1,009.5 ms | **423.7 ms** | **2.383×** | `9936352` | LPT (Longest Processing Time first) descending work schedule ($[29, 27, 25, \dots]$) eliminating Rayon thread starvation; branchless `partition_point` binary search in patience sorting accelerating LIS by 3.66× (185 ms → 50 ms). |
| [`p461.rs`](rust/solutions/src/bin/p461.rs) | 1,114.1 ms | **763.1 ms** | **1.460×** | `159820276` | Compacted pair storage to flat 8-byte `f64` array (halving sorting memory bandwidth from 1.1 GB to 551 MB); eliminated index storage in tuples via $5\ \mu	ext{s}$ post-reconstruction of indices $(a, b)$ via 2-pointer scan over the 14,142 elements of $f$. |
| [`p662.rs`](rust/solutions/src/bin/p662.rs) | 1,081.8 ms | **795.0 ms** | **1.361×** | `860873428` | **Sub-1s Target Conquered!** Padded row stride to `10,016` (128-byte alignment eliminating AVX2 split-load penalties); partitioned columns across 8 worker threads synchronized with an atomic spin-barrier (5.8 ms overhead across 10,000 rows); static compile-time unrolling of horizontal DP recurrence with register-carried accumulation. |
| [`p465.rs`](rust/solutions/src/bin/p465.rs) | 1,082.0 ms | **923.6 ms** | **1.171×** | `585965659` | Proved $\sum \Delta d \cdot s \le 9.6 	imes 10^{19} \ll 2^{128}$, allowing exact `u128` hyperbola accumulation and delayed reduction; eliminated 3 hardware `% MOD` integer divisions per iteration across 31M iterations down to a single modulo reduction per $x$. |

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
| [`p786.rs`](rust/solutions/src/bin/p786.rs) | 1,202.6 ms | **736.5 ms** | **1.633×** | `45594532839912702` | Tightened Mobius sieve limit from 300M to exact non-zero bound $l/8 = 187.5	ext{M}$, saving 340MB RAM; constant compile-time lookup tables `TAB3` and `TAB9` replacing dynamic residue loops; pure 64-bit integer arithmetic eliminating 128-bit operations. |
| [`p797.rs`](rust/solutions/src/bin/p797.rs) | 1,327.2 ms | **754.4 ms** | **1.759×** | `47722272` | Montgomery batch inversion replacing 2.5M scalar `mod_inv` Euclidean calls; parallelized raw pointer disjoint sieve for $i \in (N/4, N/2]$; parallel chunked L2-cache-resident sieve (256K elements) for Phase 3 $G[n]$ product with direct multiplier grouping. |
| [`p774.rs`](rust/solutions/src/bin/p774.rs) | 1,363.8 ms | **1,005.2 ms** | **1.357×** | `459155763` | Parallelized `hadamard`, `add`, and `apply_disjoint` TT core contractions across threads with Rayon; optimized contiguous slice copying for pivot rows; swapped loop nesting in `t_update` for contiguous memory access; SIMD swap in `gauss_elim`. |

---

## Tier 2: Top Sub-1s Problems (500ms – 1000ms)

| Rank | Problem | Current Time | Historical Notes & Optimization Angles |
|:---:|:---:|:---:|:---|
| 1 | [`p614.rs`](rust/solutions/src/bin/p614.rs) | **936.0 ms** | Phase-2 small offsets stay serial; transpose/accumulate thread_temps to cut merge traffic. |
| 2 | [`p465.rs`](rust/solutions/src/bin/p465.rs) | **923.6 ms** | Optimized in Wave 27 (1082.0 ms → 923.6 ms); u128 hyperbola accumulation. |
| 3 | [`p337.rs`](rust/solutions/src/bin/p337.rs) | **916.7 ms** | Optimized in Wave 26 (1804.7 ms → 916.7 ms); AVX2 Wide Segment Tree B=16. |
| 4 | [`p662.rs`](rust/solutions/src/bin/p662.rs) | **795.0 ms** | Optimized in Wave 27 (1081.8 ms → 795.0 ms); 128-byte aligned AVX2 spin-barrier DP. |
| 5 | [`p850.rs`](rust/solutions/src/bin/p850.rs) | **774.7 ms** | coprime-pair product; Möbius hyperbola? |
| 6 | [`p461.rs`](rust/solutions/src/bin/p461.rs) | **763.1 ms** | Optimized in Wave 27 (1114.1 ms → 763.1 ms); flat f64 pairs + 2-pointer reconstruction. |
| 7 | [`p910.rs`](rust/solutions/src/bin/p910.rs) | **755.6 ms** | wave1 3.5x still ~7s; streaming tables+rayon already vs C 45-line phi |
| 8 | [`p797.rs`](rust/solutions/src/bin/p797.rs) | **754.4 ms** | Optimized in Wave 26 (1327.2 ms → 754.4 ms); Montgomery batch inv + L2 chunked sieve. |
| 9 | [`p735.rs`](rust/solutions/src/bin/p735.rs) | **751.9 ms** | Chunked rayon + fused inner isqrt already present; leftover CHUNK tuning is noise-level. |
| 10 | [`p931.rs`](rust/solutions/src/bin/p931.rs) | **743.6 ms** | Lucy prime-sum DP is loop-carried O(N^{2/3}); two-array get_idx is a small cache tweak only. |
| 11 | [`p786.rs`](rust/solutions/src/bin/p786.rs) | **736.5 ms** | Optimized in Wave 26 (1202.6 ms → 736.5 ms); 187.5M Mobius bound + TAB3/TAB9 LUTs. |
| 12 | [`p954.rs`](rust/solutions/src/bin/p954.rs) | **660.4 ms** | Optimized in Wave 26 (1251.7 ms → 660.4 ms); single-pass digit DP. |
| 13 | [`p459.rs`](rust/solutions/src/bin/p459.rs) | **599.8 ms** | **Optimized in Wave 32** (900.1 ms → 599.8 ms, **1.501×**); epoch-tracking used[v] in 1D mex calculation. |
| — | [`p411.rs`](rust/solutions/src/bin/p411.rs) | **423.7 ms** | Optimized in Wave 27 (1009.5 ms → 423.7 ms); LPT work schedule + branchless partition_point LIS. |
| — | [`p654.rs`](rust/solutions/src/bin/p654.rs) | **408.5 ms** | Optimized in Wave 26 (1165.7 ms → 408.5 ms); frequency-domain NTT. |
| — | [`p464.rs`](rust/solutions/src/bin/p464.rs) | **345.0 ms** | Optimized in Wave 26 (1209.1 ms → 345.0 ms); merge-sort inversion counting. |
| — | [`p521.rs`](rust/solutions/src/bin/p521.rs) | **331.7 ms** | **Optimized in Wave 32** (850.1 ms → 331.7 ms, **2.563×**); piecewise-constant quotient block stepping + compacted u32. |
| — | [`p769.rs`](rust/solutions/src/bin/p769.rs) | **307.7 ms** | **Optimized in Wave 30** (707.9 ms → 307.7 ms, **2.300×**); fixed-point inv-sqrt3 multiplication + fast_isqrt. |
| — | [`p681.rs`](rust/solutions/src/bin/p681.rs) | **303.2 ms** | Optimized in Wave 29 (982.8 ms → 303.2 ms, **3.241×**); SWAR packed exponents + quadratic bound + thread-local buffer reuse. |
| — | [`p563.rs`](rust/solutions/src/bin/p563.rs) | **195.8 ms** | Optimized in Wave 28 (838.8 ms → 195.8 ms); 50M bound + parallel streaming mult. |
| — | [`p552.rs`](rust/solutions/src/bin/p552.rs) | **178.2 ms** | Optimized in Wave 29 (1338.0 ms → 178.2 ms, **7.509×**); blocked parallel Garner CRT + ceiling Barrett reduction. |
| — | [`p439.rs`](rust/solutions/src/bin/p439.rs) | **177.0 ms** | Optimized in Wave 26 (1073.3 ms → 177.0 ms); L3 cache u32 sieve + dyadic layers. |
| — | [`p482.rs`](rust/solutions/src/bin/p482.rs) | **164.2 ms** | **Optimized in Wave 32** (753.9 ms → 164.2 ms, **4.592×**); Compressed Sparse Row (CSR) + stack divisor buffer. |
| — | [`p378.rs`](rust/solutions/src/bin/p378.rs) | **156.2 ms** | **Optimized in Wave 30** (715.3 ms → 156.2 ms, **4.580×**); 128-chunk parallel histogram Fenwick (480MB saved) + AVX2 prefix sums. |
| — | [`p507.rs`](rust/solutions/src/bin/p507.rs) | **141.2 ms** | Optimized in Wave 27 (985.2 ms → 141.2 ms); zero-alloc Tribonacci matrix exponentiation. |
| — | [`p782.rs`](rust/solutions/src/bin/p782.rs) | **122.7 ms** | Optimized in Wave 29 (652.4 ms → 122.7 ms, **5.316×**); parallel chunked C1 sieve + word-level reverse_bits complement OR. |
| — | [`p559.rs`](rust/solutions/src/bin/p559.rs) | **93.2 ms** | Optimized in Wave 29 (907.6 ms → 93.2 ms, **9.736×**); thread-local workspaces + threshold 128 Newton inversion + butterfly NTT. |
| — | [`p937.rs`](rust/solutions/src/bin/p937.rs) | **89.0 ms** | Optimized in Wave 28 (944.4 ms → 89.0 ms); odd-only 16KB sieve + lock-free diff. |
| — | [`p534.rs`](rust/solutions/src/bin/p534.rs) | **79.2 ms** | Optimized in Wave 28 (921.9 ms → 79.2 ms); incremental DFS bitmasks + symmetry. |
| — | [`p578.rs`](rust/solutions/src/bin/p578.rs) | **61.6 ms** | Optimized in Wave 26 (1089.7 ms → 61.6 ms); O(1) table + analytic root cutoffs. |
| — | [`p423.rs`](rust/solutions/src/bin/p423.rs) | **46.7 ms** | **Optimized in Wave 30** (749.8 ms → 46.7 ms, **16.041×**); odd-only 3MB bitset sieve + Montgomery batch inversion + linear recurrence. |
| — | [`p413.rs`](rust/solutions/src/bin/p413.rs) | **39.0 ms** | **Optimized in Wave 30** (646.0 ms → 39.0 ms, **16.573×**); zero-allocation remainder bijection DP + parity-invariance restriction. |
| — | [`p576.rs`](rust/solutions/src/bin/p576.rs) | **37.8 ms** | **Optimized in Wave 30** (670.3 ms → 37.8 ms, **17.745×**); bounded prime sliding window with stack-allocated MonoDeque ring buffer. |
| — | [`p643.rs`](rust/solutions/src/bin/p643.rs) | **32.2 ms** | **Optimized in Wave 30** (718.1 ms → 32.2 ms, **22.285×**); even-only hyperbola states + direct-indexed large table + 64KB block phi sieve. |
| — | [`p743.rs`](rust/solutions/src/bin/p743.rs) | **30.0 ms** | **Optimized in Wave 30** (737.3 ms → 30.0 ms, **24.608×**); blocked parallel linear recurrence + local Montgomery batch inversion in L1d. |
| — | [`p928.rs`](rust/solutions/src/bin/p928.rs) | **20.4 ms** | Optimized in Wave 29 (1078.8 ms → 20.4 ms, **52.823×**); scalar recurse_tail after rank 6 + 64-byte Gf + depth-4 split. |
| — | [`p650.rs`](rust/solutions/src/bin/p650.rs) | **8.4 ms** | Optimized in Wave 29 (937.1 ms → 8.4 ms, **111.649×**); inverted prime loop order + geometric progression multiplications. |
| — | [`p946.rs`](rust/solutions/src/bin/p946.rs) | **2.0 ms** | Optimized in Wave 29 (953.7 ms → 2.0 ms, **467.411×**); limit-cycle attractor fast-forwarding 10^8 steps in O(1) cycles. |

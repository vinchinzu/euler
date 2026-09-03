# Slowest Remaining Problems Queue (Descending)

**Updated:** 2026-09-03 (Wave 25 completed: 8 problems A/B gated, 8 accepted)

## Current Project Snapshot

| Metric | Value |
|---|---:|
| Total Solutions | 997 |
| Total Sequential Wall-Clock | **119.17 s** |
| Speedup vs Original Cache (1101.1s) | **9.24×** |
| Speedup vs Clean 5900XT Baseline (386.1s) | **3.24×** (~266.9s saved) |
| Speedup vs ~180s Milestone (2026-08-25) | **1.49×** (~58.23s saved) |
| Median Execution Time | **23.5 ms** |
| Remaining ≥ 1.0s | **11** |
| Remaining 500ms – 1.0s | **55** |
| Remaining 200ms – 500ms | **107** |
| Remaining 50ms – 200ms | **232** |
| Fast (< 50ms) | **592** |

---

## Tier 1: All Remaining Problems ≥ 1.0s (Only 11 targets left!)

| Rank | Problem | Current Time | Band | Historical Notes & Optimization Angles |
|:---:|:---:|:---:|:---:|:---|
| 1 | [`p680.rs`](rust/solutions/src/bin/p680.rs) | **2142.8 ms** | ≥1.0s | implicit treap N=1e18 K=1e6; recursion/stack only |
| 2 | [`p337.rs`](rust/solutions/src/bin/p337.rs) | **1766.8 ms** | ≥1.0s | Fenwick DP over 2e7; loop-carried like p410 |
| 3 | [`p774.rs`](rust/solutions/src/bin/p774.rs) | **1436.4 ms** | ≥1.0s | wave2 i32 MPS/GE accepted; remaining serial TT sweep |
| 4 | [`p954.rs`](rust/solutions/src/bin/p954.rs) | **1201.7 ms** | ≥1.0s | already rayon (l,tr); leftover FxHashMap vs C DFS |
| 5 | [`p578.rs`](rust/solutions/src/bin/p578.rs) | **1180.0 ms** | ≥1.0s | FxHashMap memos; rayon top-level prime branches with private maps then merge. |
| 6 | [`p464.rs`](rust/solutions/src/bin/p464.rs) | **1151.0 ms** | ≥1.0s | Mertens/Möbius window; Lucy vs sieve |
| 7 | [`p786.rs`](rust/solutions/src/bin/p786.rs) | **1145.7 ms** | ≥1.0s | inner rayon already |
| 8 | [`p379.rs`](rust/solutions/src/bin/p379.rs) | **1138.0 ms** | ≥1.0s | Keep outer d par_iter; run t_func inner loop sequentially to drop nested-rayon contention. |
| 9 | [`p654.rs`](rust/solutions/src/bin/p654.rs) | **1129.3 ms** | ≥1.0s | NTT mulmod uses u128; P1/P2/P3 all <2^32 so u64 mulmod |
| 10 | [`p662.rs`](rust/solutions/src/bin/p662.rs) | **1126.0 ms** | ≥1.0s | Lattice DP with carried h_jumps; rayon on cheap x-adds likely regresses (playbook tiny-iter). |
| 11 | [`p797.rs`](rust/solutions/src/bin/p797.rs) | **1049.6 ms** | ≥1.0s | Incremental 2^i for F[i]; linear-sieve inverses instead of N Fermat inv; optional rayon on final sum. |

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

# Optimization status — Ryzen 9 5900XT re-time

**Frozen snapshot.** Generated 2026-08-22 *before* waves 4–6. Do not treat the queues below as the current work list. Living log and remaining counts: `optimization_applied_summary.md`.

Re-timed every Rust binary on this box after a clean fat-LTO release build. Classification uses the new wall-clock plus existing notes (`optimization_applied_summary.md`, A/B CSV, `validated.json` `optimized` flags, wave-2 partials) and a source skim of the slow set.

## Method

| | |
|---|---|
| CPU | AMD Ryzen 9 5900XT, 16 cores / 32 threads, `performance` governor |
| Previous cache | `validated.json` from 2026-07-28 (older/slower cores; wave 3 was timed on 4c/8t) |
| Build | `cargo build --release` (fat LTO, `codegen-units=1`, `-C target-cpu=native`), rustc 1.98.0 |
| Timing | sequential wall-clock, `RAYON_NUM_THREADS=32`, 60s timeout, stdout vs `data/answers.txt` |
| A/B gate (unchanged) | accept only if median ≥5% faster and answer matches |

Wave-2 worktrees (`~/.grok/worktrees/.../subagent-*`) are **gone**. Unmerged partials in the notes are historical only — do not copy stale candidates; re-implement and A/B on this machine.

## Totals

- Solutions: **997** (996 OK, 1 WRONG, 0 TIMEOUT)
- Old cache sum: **1101.1s**
- New sum: **386.1s** (2.85× faster overall)
- Median: **41 ms** (was 73 ms)

| Band (new) | Count |
|---|---:|
| <50ms | 513 |
| 50–200ms | 169 |
| 200–500ms | 105 |
| 500ms–1s | 91 |
| 1–1.5s | 45 |
| 1.5–3s | 48 |
| 3–6s | 22 |
| 6–10s | 3 |
| ≥10s | 0 |

| Classification | Count | Meaning |
|---|---:|---|
| already optimized, now <1s | 79 | A/B accepted, `optimized=true`, or playbook already applied |
| already optimized, still ≥1s | 21 | rayon / prior A/B in tree; remaining work is algorithm or load-balance |
| **needs further refactor, ≥1s** | **73** | playbook still applies (rayon, u64 mulmod, HashMap→Vec, C port, incremental pow) |
| sequential-hard, ≥1s | 24 | loop-carried DP / Lucy / Fenwick; cheap `par_iter` likely regresses (p410/p518) |
| fast, no action | 794 | currently <1s |
| wrong | 1 | see p968 below |

Two binaries got **slower** by a meaningful amount (likely 32-thread nested-rayon contention, not a source change): **p614 7281→9153 ms**, **p464 2995→4113 ms**.

## Already optimized

Union of A/B waves 1–3 (37 accepted, 6 rejected), 48 `validated.json` `optimized: true` flags, and 13 wave-2 partial IDs = **99 touched**. Many of those are now well under 1s on 16 cores.

### A/B accepted — now <1s on 5900XT

| P | wave | new ms | A/B cand ms | old cache ms | notes |
|---|---|---|---|---|---|
| 932 | 1 | 10 | 135 | 124 | speedup=34.337x |
| 263 | 3 | 17 | 90 | 2307 | local SPRP 2/7/61; wheel; rayon chunks |
| 486 | 3 | 32 | 114 | 2313 | phi-order of 2 mod K; incremental 2^k not pow_mod |
| 752 | 3 | 37 | 237 | 2744 | i64 mat_mul; SPF factor p^2-1; rayon primes |
| 455 | 3 | 38 | 468 | 3029 | rayon over n=2..1e6; u64 pow_mod (K=1e9) |
| 531 | 3 | 38 | 368 | 2445 | rayon over n; iterative ext_gcd; drop i128 |
| 688 | 3 | 68 | 432 | 2691 | rayon over k-chunks; u64 mulmod |
| 784 | 3 | 68 | 567 | 2807 | rayon over p; Vec SPF (was static mut) |
| 404 | 3 | 69 | 470 | 2694 | rayon over both nn cases |
| 785 | 1 | 84 | 942 | 6963 | speedup=5.908x |
| 557 | 1 | 98 | 889 | 5324 | rayon over outer a loop |
| 586 | 3 | 125 | 316 | 2568 | k=1/k=2 leaf counts; odd sieve; rayon::join |
| 657 | 2 | 132 | 411 | 6424 | rayon parallel pow[t]^(N+1) over 10M; i128->u64 |
| 589 | 3 | 137 | 617 | 2464 | rayon over independent (m,n) linear systems |
| 971 | 1 | 230 | 1980 | 1918 | speedup=2.79x |
| 558 | 1 | 266 | 1175 | 2357 | speedup=5.904x |
| 628 | 3 | 287 | 415 | 2192 | i128->u64 factorial pass (M^2 < 2^64) |
| 962 | 1 | 410 | 7186 | 7008 | speedup=1.235x |
| 675 | 3 | 446 | 1620 | 1767 | i128->u64 in S(i!) update |
| 544 | 3 | 451 | 728 | 2061 | no HashMap clone; i32 stack DP; FxHashMap |
| 757 | 3 | 451 | 2075 | 2639 | parallel generate + par_sort_unstable |
| 747 | 3 | 456 | 1119 | 1553 | rayon over a + u64 ncr |
| 421 | 1 | 469 | 2053 | 5862 | u128->u64 pow_mod; rayon over primes |
| 941 | 1 | 533 | 5205 | 4810 | speedup=2.758x |
| 741 | 3 | 614 | 716 | 2803 | u64 mulmod + rayon::join of g(N1)/g(N2) |
| 873 | 2 | 637 | 2472 | 5756 | linear-sieve inv table; i128->u64 mul |
| 693 | 1 | 705 | 5281 | 7149 | speedup=1.302x |
| 743 | 3 | 737 | 1619 | 2305 | u64 mulmod + unchecked inv; recurrence still serial |
| 946 | 2 | 823 | 4311 | 8902 | i128->i32; specialize coeff=1; floor_div fast path (from batch D) |
| 650 | 2 | 879 | 1174 | 4960 | i128->u64 in power() / D(n) product |
| 534 | 2 | 912 | 1850 | 6556 | rayon over independent k=0..13 (DP + DFS) |
| 937 | 1 | 919 | 4309 | 4186 | port C k%3!=2 formula |
| 507 | 1 | 928 | 2707 | 7398 | stack arrays + rayon over iterations |

### A/B accepted — still ≥1s

Further cuts need a better algorithm or a closed form, not another cheap `par_iter`.

| P | wave | new ms | A/B cand ms | old cache ms | notes |
|---|---|---|---|---|---|
| 774 | 2 | 3766 | 11351 | 7386 | i32 cores+mat; euclid modinv; ptr GE; mat reuse (from batch D) |
| 445 | 2 | 1428 | 4741 | 6383 | cache cur_pow + delta p^e; egcd mod_inv |
| 540 | 1 | 1311 | 4974 | 6993 | speedup=1.420x |
| 910 | 1 | 1227 | 7158 | 6864 | phi recursion vs FxHashMap; also pre-wave FxHashMap + u128->u64 (11.1s->9.1s) |

### Other previously optimized, still ≥1s

(Has `optimized=true`, prior rayon, or an unmerged partial already in the current tree — not a greenfield playbook target.)

| P | new ms | old ms | already | why still slow |
|---|---|---|---|---|
| 846 | 7394 | 22859 | wave2 partial D (unmerged) | Already seed-level rayon DFS; remaining load-balance is low-confidence (batch D no-diff). |
| 448 | 3563 | 9138 | wave2 partial B (unmerged) | FxHashMap, no dyn Fn, u64 mod already in tree; leftover is sequential Dirichlet k_phi_sum cache. |
| 886 | 3069 | 7967 | optimized=true | Already unchecked memo + stack Ctx (validated optimized); C globals will not move the needle. |
| 954 | 3027 | 4959 |  | already rayon (l,tr); leftover FxHashMap vs C DFS |
| 989 | 2310 | 3061 |  | hybrid rayon small-g + sequential large-g |
| 478 | 2284 | 3672 | optimized=true | already i128->i64; remaining recurrence serial |
| 452 | 2004 | 3585 | optimized=true | already i128->i64 (M=1234567891); remaining DP sequential |
| 660 | 1942 | 4366 |  | inner rayon already; leftover branchy search |
| 370 | 1840 | 6739 |  | already parallel; stack factor list leftover only |
| 953 | 1820 | 9103 |  | No i128, rayon DFS, unchecked already; only DFS load-balance remains (triage 1.2–1.5× low). |
| 786 | 1457 | 4548 |  | inner rayon already |
| 705 | 1429 | 3143 | optimized=true | already u128->u64 MOD=1e9+7 |
| 782 | 1356 | 14227 | A/B w1 reject; optimized=true | A/B 18761→18055ms (1.039×, within noise); non-atomic bitset already in tree. |
| 837 | 1322 | 2638 | optimized=true | already rayon join on factorials |
| 635 | 1197 | 3183 | optimized=true | already rayon; local u64 mod_pow |
| 861 | 1040 | 3781 |  | already rayon |
| 754 | 1033 | 3406 | optimized=true | already optimized product-of-gcds |

### A/B rejected / neutral (do not retry the same idea)

| P | new ms | old ms | reason |
|---|---|---|---|
| 410 | 1006 | 2747 | rayon over 1e8 cheap j after memory-bound sieve: 2747ms -> 3132ms |
| 518 | 1312 | 2873 | rayon over 25M tiny-k (most empty): 2873ms -> 3811ms |
| 606 | 1536 | 5232 | already pure u64; rayon large_s snapshot / final-sum parallelization no gain (8805ms -> 8805ms) |
| 782 | 1356 | 14227 | NEUTRAL/within noise: 18761ms -> 18055ms (1.039x); still optimized=true in validated.json |
| 829 | 7407 | 11685 | regression 12319ms -> 17462ms (0.705x); prior pruning already in tree (14.8s->11.7s) |
| 847 | 4236 | 8220 | NEUTRAL/within noise: 11668ms -> 11238ms (1.038x) |

## Needs further refactor (priority queue)

**47 problems ≥1.5s** where the 2s-playbook still applies, plus **26** in 1.0–1.5s. Ranked by new time on this CPU.

Highest-confidence next wave (skip A/B-rejected ideas): p614, p559, p378, p468, p592, p464, p461, p238, p715, p154, p925, p379, p415, p314, p769.

### ≥1.5s

| P | new ms | old ms | old/new | rayon | signals | already | next step |
|---|---|---|---|---|---|---|---|
| 614 | 9153 | 7281 | 0.8 | yes |  | optimized=true | Phase-2 small offsets stay serial; transpose/accumulate thread_temps to cut merge traffic. |
| 829 | 7407 | 11685 | 1.58 |  | u128,HashMap | A/B w1 reject; optimized=true | A/B rejected one DFS prune; still sequential n=2..31 + prime DFS; parallelize top-level n/primes |
| 559 | 5518 | 7090 | 1.28 |  |  | wave2 partial B (unmerged) | rayon over independent k (each owns dp/parts); precompute pow_inv_fact once. |
| 378 | 5367 | 10061 | 1.87 |  |  |  | Shrink right_arr to i32 (counts < 6e7), drop SPF after dt, keep Fenwick sequential; optional rayon over independent dt[i] given SPF. |
| 468 | 5044 | 6791 | 1.35 |  | i128 | optimized=true | already u32/no-i128; parallelize small-B phase |
| 847 | 4236 | 8220 | 1.94 |  | pow_mod | A/B w1 reject | A/B noise-reject; FxHashMap digit-DP vs C open-addressing |
| 592 | 4141 | 5538 | 1.34 |  | u128 |  | rayon the 27 independent f_vals blocks (each is 2^22 odd products). |
| 464 | 4113 | 2995 | 0.73 |  |  |  | Mertens/Möbius window; Lucy vs sieve |
| 461 | 4092 | 7872 | 1.92 |  |  |  | par_sort_unstable on the 85M pairs (profile is stable sort); skip the third reconstruct pass. |
| 238 | 3936 | 8034 | 2.04 |  |  |  | Arithmetic digit extract instead of format!; replace O(D^2) first_occs nest with a bitset/queue cover. |
| 715 | 3747 | 9958 | 2.66 |  | i128 | wave2 partial B (unmerged) | Store ff as Vec<u32> (halves ~800MB SPF) and linear-sieve mu_prime; big[i] Lucy fill stays sequential. |
| 154 | 3558 | — |  |  |  |  | no rayon; scan for independent outer loops |
| 925 | 3292 | 8132 | 2.47 |  | i128,u128,pow_mod |  | rayon over digit lengths 1..=16 (or MSD start digits) of the independent DFS stacks. |
| 379 | 3215 | 7827 | 2.43 | yes |  |  | Keep outer d par_iter; run t_func inner loop sequentially to drop nested-rayon contention. |
| 415 | 3215 | 7278 | 2.26 |  | pow_mod | wave2 partial C (unmerged) | Incremental p2g=2^g in the g=1..L loop; fuse Lucy sm/lg passes where possible. |
| 314 | 3159 | 4746 | 1.5 |  |  |  | Drop MAX_STEP 15→5 to match c/314.c (~9× fewer edges) and re-check the printed 8 decimals. |
| 769 | 3120 | 5831 | 1.87 |  |  |  | rayon over independent g; integer isqrt instead of f64 sqrt in the inner n_idx loop. |
| 867 | 3071 | 5354 | 1.74 |  | pow_mod |  | Precompute independent hex/trap tilings (rayon over size/window); mask DP inner can par over prev. |
| 994 | 2951 | 6410 | 2.17 | yes | u128 |  | Drop par over tiny Du Jiao floor-blocks (playbook tiny-iter); raise sieve so fewer HashMap big[] fills. |
| 797 | 2845 | 7001 | 2.46 |  | pow_mod |  | Incremental 2^i for F[i]; linear-sieve inverses instead of N Fermat inv; optional rayon on final sum. |
| 681 | 2681 | 16163 | 6.03 | yes |  | wave2 partial C (unmerged) | Work flattening exists but heavy highly-composite K still starve; split those K by divisor-pair chunks. |
| 414 | 2619 | 3491 | 1.33 |  |  |  | kaprekar/base-b; check independent bases |
| 536 | 2570 | 8526 | 3.32 |  |  | wave2 partial C (unmerged) | rayon over root-level prime index in helper (independent subtrees, per-thread ans). |
| 975 | 2524 | 4252 | 1.68 |  |  |  | ported from python; winding-path geometry vs C |
| 483 | 2518 | 3696 | 1.47 |  | HashMap |  | HashMap partition memo; cycle-index search |
| 578 | 2491 | 4654 | 1.87 |  | HashMap |  | FxHashMap memos; rayon top-level prime branches with private maps then merge. |
| 655 | 2387 | 7057 | 2.96 |  | pow_mod |  | Eliminate inner %k via split-index add; sparse nonzero-j list instead of scanning 10M zeros. |
| 543 | 2348 | 7368 | 3.14 |  |  | wave2 partial B (unmerged) | One prefix popcount of the bit sieve, then O(1) pi for all F_k; drop 42 full scans. |
| 623 | 2336 | 3321 | 1.42 |  |  |  | lambda expressions / combinatorics DP |
| 938 | 2306 | 2641 | 1.15 |  |  |  | expected-value DP / coins |
| 585 | 2248 | 6709 | 2.98 |  |  |  | rayon over g+h for the O(N log N) second sum; integer is_sq (no f64) in the s,t nest. |
| 966 | 2101 | 3324 | 1.58 |  |  |  | circle-lattice; independent radii? |
| 437 | 2050 | 5792 | 2.83 |  | u128 |  | rayon over primes; fib_pair u64 not u128 (p<1e8 so p^2 fits u64). |
| 427 | 1958 | 8247 | 4.21 |  | pow_mod | wave2 partial C (unmerged) | Precompute all fk with rayon over k, then sequential prefix delta*k; leave fact/inv_fact serial. |
| 433 | 1892 | 5426 | 2.87 |  |  |  | rayon over squarefree g; make extgcd iterative (hot inner a,b). |
| 890 | 1868 | 3005 | 1.61 |  | i128,u128 |  | binomial/mod factorial table? |
| 637 | 1854 | 5285 | 2.85 |  |  |  | rayon::join compute_f(10) and compute_f(3); par_iter the 10M i-loop inside each. |
| 708 | 1831 | 7611 | 4.16 |  |  |  | Memoize sum_floor_quotients for q>l; rayon independent root prime subtrees. |
| 729 | 1830 | 2444 | 1.34 |  |  |  | stalled/low IPC per profiles; range-min query? |
| 864 | 1801 | 6379 | 3.54 |  | i128 |  | rayon::join the two Part B DFS roots; squarefree sieve instead of trial division in Pell counts. |
| 972 | 1795 | 5895 | 3.28 | yes | HashMap | optimized=true | Split diameter keys (tag 0) from circle GeoKey; pack key to cut 32-byte hash traffic. |
| 331 | 1726 | 2187 | 1.27 |  |  |  | look at lattice cross-count closed form vs brute |
| 701 | 1635 | 3787 | 2.32 |  | HashMap |  | HashMap State DP per row; encode to Vec |
| 538 | 1610 | 3538 | 2.2 |  |  |  | maximum quadrilaterals; look at hull loops |
| 947 | 1584 | 5823 | 3.68 |  | u128,HashMap,pow_mod |  | Vec period cache (p,e bounded) not HashMap; u64 Mat2 mul (pe<=1e6); then rayon over m. |
| 501 | 1540 | 5175 | 3.36 |  |  |  | rayon outer p in the p*q*r loops; pi_small as Vec<u32> (~400MB cut). |
| 893 | 1504 | 2040 | 1.36 |  |  |  | look at digit-equation search |

### 1.0–1.5s

| P | new ms | old ms | old/new | rayon | signals | already | next step |
|---|---|---|---|---|---|---|---|
| 465 | 1497 | 4586 | 3.06 |  | u128,pow_mod |  | rayon::join the two Lucy totient prefixes (mod M and M-1); u64 mulmod (MOD^2<2^64) instead of u128. |
| 883 | 1486 | 3589 | 2.42 |  |  |  | look at lattice/triangle enumeration loops |
| 411 | 1482 | 2977 | 2.01 |  | u128,pow_mod |  | order() via pow_mod peeling; LIS of 2^n,3^n points may parallelize poorly |
| 611 | 1465 | 7122 | 4.86 |  |  |  | Cache p%4 and avoid n_val/p/p in DFS; rayon top-level prime-power frames after Lucy. |
| 211 | 1394 | — |  |  |  |  | no rayon; scan for independent outer loops |
| 639 | 1363 | 4017 | 2.95 |  | i128,pow_mod |  | i128 powmod MOD=1e9+7; powerful-number iteration |
| 416 | 1327 | 5054 | 3.81 |  |  |  | rayon::join independent M1/M2 mat_pow; deferred i128 reduce in 800^3 mul (m2^2 still tight). |
| 216 | 1321 | — |  |  | u128,pow_mod |  | no rayon; scan for independent outer loops; u128, pow_mod |
| 654 | 1314 | 4458 | 3.39 |  | u128 | optimized=true | NTT mulmod uses u128; P1/P2/P3 all <2^32 so u64 mulmod |
| 963 | 1307 | 3707 | 2.84 |  | u128,HashMap |  | look at palindromic-path / graph search |
| 878 | 1287 | 2762 | 2.15 |  | HashMap |  | HashMap in hot path |
| 984 | 1264 | 2779 | 2.2 |  | i128,pow_mod |  | look at digit/automaton DP |
| 919 | 1247 | 3672 | 2.94 |  |  |  | profiles memory-bound; two-player DP? |
| 799 | 1240 | 4521 | 3.65 |  | i128 |  | rayon over representation candidates; keep i128 only for Gaussian products that can exceed u64. |
| 155 | 1226 | — |  |  |  |  | no rayon; scan for independent outer loops |
| 447 | 1154 | 3284 | 2.85 |  | i128 |  | i128 Dirichlet sums; MOD likely 1e9-class |
| 850 | 1151 | 2837 | 2.46 |  |  |  | coprime-pair product; Möbius hyperbola? |
| 691 | 1140 | 8100 | 7.11 |  |  |  | Replace doubling SA with SA-IS O(n); keep LCP/UF sweep. |
| 362 | 1134 | 3390 | 2.99 |  | HashMap |  | HashMap qmap in squarefree-count recursion; Vec/index if bounded |
| 596 | 1090 | 3607 | 3.31 |  |  |  | lattice points in hypersphere; Dirichlet hyperbola? |
| 748 | 1088 | 3196 | 2.94 |  |  |  | stalled IPC; pentagonal-like search |
| 420 | 1072 | 3633 | 3.39 |  |  |  | independent outer t1 after divisor sieve; memory-bound num_divs though |
| 470 | 1053 | 1915 | 1.82 |  |  |  | super-rps DP; check state independence |
| 626 | 1039 | 3495 | 3.36 |  | i128,pow_mod |  | binary-matrix palindromes; bit DP? |
| 153 | 1012 | — |  |  |  |  | no rayon; scan for independent outer loops |
| 810 | 1001 | 2459 | 2.46 |  |  |  | xor-primes sieve analog; bitset memory |

Wave-2 partial IDs that still look worth a **fresh** A/B on this box: p415, p427, p536, p543, p559, p715 (p448 looks already in-tree). Worktree copies are gone.

## Sequential-hard / skip-first (≥1s)

Loop-carried Lucy / Fenwick / Garner / treap. Rayon over tiny inner iterations already failed on p410 and p518.

| P | new ms | old ms | already | why skip-first |
|---|---|---|---|---|
| 505 | 4299 | 6874 |  | Binary-tree recurrence is strictly sequential; C is the same helper. |
| 459 | 3352 | 5554 |  | O(N sqrt N) mex DP is sequential; fuse loops would be a small memory-pass tweak. |
| 552 | 2877 | 8468 |  | O(L^2) Garner with loop-carried a/prod; C identical; no rayon/i128 play. |
| 337 | 2148 | 4178 |  | Fenwick DP over 2e7; loop-carried like p410 |
| 931 | 2092 | 8091 |  | Lucy prime-sum DP is loop-carried O(N^{2/3}); two-array get_idx is a small cache tweak only. |
| 428 | 2036 | 3732 |  | Lucy pi_1 + HashMap Mertens caches; loop-carried |
| 852 | 1941 | 3975 |  | backward-induction coin DP; state deps |
| 680 | 1870 | 6372 | optimized=true | implicit treap N=1e18 K=1e6; recursion/stack only |
| 662 | 1814 | 4632 |  | Lattice DP with carried h_jumps; rayon on cheap x-adds likely regresses (playbook tiny-iter). |
| 870 | 1806 | 3619 |  | recurrence P_{k+1}=P_k+P_m(k); true deps; u128 only in scratch tools |
| 608 | 1687 | 5833 | optimized=true | Lucy+DFS 800MB; chunk sieve not compute-bound |
| 521 | 1685 | 5756 |  | Lucy SPF-sum O(N^{2/3}) is loop-carried; only inlining left. |
| 739 | 1563 | 4005 |  | Lucas+inv table 1e8; loop-carried |
| 606 | 1536 | 5232 | A/B w2 reject | already pure u64 Lucy; rayon rejected; cite p606 |
| 593 | 1513 | 4031 |  | Fenwick sliding median + 2e8 sieve |
| 256 | 1472 | 3804 |  | shared counts[s] writes over 1e8; rayon needs atomics or huge thread locals (p410-like) |
| 259 | 1352 | 3753 |  | concat DP over digit splits with HashSet/Frac; true deps |
| 518 | 1312 | 2873 | A/B w3 reject | wave3 rayon over 25M k rejected (2873->3811); cite p518 |
| 351 | 1297 | 3865 |  | phi sieve 1e8; memory-bound, rayon on inner j would regress |
| 399 | 1172 | 3344 |  | profiles memory-bound Fibonacci/mod-p sieve style |
| 580 | 1153 | 3121 |  | profiles memory-bound squarefree Hilbert; likely Lucy |
| 929 | 1049 | 4224 |  | NTT poly inverse 3-mod CRT; inherently serial NTT stages |
| 712 | 1019 | 4471 |  | Lucy pi for large p dominates; small-prime double loop is cheap vs the sequential sieve. |
| 410 | 1006 | 2747 | A/B w3 reject | wave3 rayon over 1e8 cheap j rejected (2747->3132); cite p410 |

## Slowest 40 on 5900XT

| P | new ms | old ms | old/new | bucket | already |
|---|---|---|---|---|---|
| 614 | 9153 | 7281 | 0.8 | needs_refactor | optimized=true |
| 829 | 7407 | 11685 | 1.58 | needs_refactor | A/B w1 reject; optimized=true |
| 846 | 7394 | 22859 | 3.09 | already_opt_still_slow | wave2 partial D (unmerged) |
| 559 | 5518 | 7090 | 1.28 | needs_refactor | wave2 partial B (unmerged) |
| 378 | 5367 | 10061 | 1.87 | needs_refactor |  |
| 468 | 5044 | 6791 | 1.35 | needs_refactor | optimized=true |
| 505 | 4299 | 6874 | 1.6 | sequential_hard |  |
| 847 | 4236 | 8220 | 1.94 | needs_refactor | A/B w1 reject |
| 592 | 4141 | 5538 | 1.34 | needs_refactor |  |
| 464 | 4113 | 2995 | 0.73 | needs_refactor |  |
| 461 | 4092 | 7872 | 1.92 | needs_refactor |  |
| 238 | 3936 | 8034 | 2.04 | needs_refactor |  |
| 774 | 3766 | 7386 | 1.96 | already_opt_still_slow | A/B w2 accept |
| 715 | 3747 | 9958 | 2.66 | needs_refactor | wave2 partial B (unmerged) |
| 448 | 3563 | 9138 | 2.56 | already_opt_still_slow | wave2 partial B (unmerged) |
| 154 | 3558 | — |  | needs_refactor |  |
| 459 | 3352 | 5554 | 1.66 | sequential_hard |  |
| 925 | 3292 | 8132 | 2.47 | needs_refactor |  |
| 379 | 3215 | 7827 | 2.43 | needs_refactor |  |
| 415 | 3215 | 7278 | 2.26 | needs_refactor | wave2 partial C (unmerged) |
| 314 | 3159 | 4746 | 1.5 | needs_refactor |  |
| 769 | 3120 | 5831 | 1.87 | needs_refactor |  |
| 867 | 3071 | 5354 | 1.74 | needs_refactor |  |
| 886 | 3069 | 7967 | 2.6 | already_opt_still_slow | optimized=true |
| 954 | 3027 | 4959 | 1.64 | already_opt_still_slow |  |
| 994 | 2951 | 6410 | 2.17 | needs_refactor |  |
| 552 | 2877 | 8468 | 2.94 | sequential_hard |  |
| 797 | 2845 | 7001 | 2.46 | needs_refactor |  |
| 681 | 2681 | 16163 | 6.03 | needs_refactor | wave2 partial C (unmerged) |
| 414 | 2619 | 3491 | 1.33 | needs_refactor |  |
| 536 | 2570 | 8526 | 3.32 | needs_refactor | wave2 partial C (unmerged) |
| 975 | 2524 | 4252 | 1.68 | needs_refactor |  |
| 483 | 2518 | 3696 | 1.47 | needs_refactor |  |
| 578 | 2491 | 4654 | 1.87 | needs_refactor |  |
| 655 | 2387 | 7057 | 2.96 | needs_refactor |  |
| 543 | 2348 | 7368 | 3.14 | needs_refactor | wave2 partial B (unmerged) |
| 623 | 2336 | 3321 | 1.42 | needs_refactor |  |
| 989 | 2310 | 3061 | 1.33 | already_opt_still_slow |  |
| 938 | 2306 | 2641 | 1.15 | needs_refactor |  |
| 478 | 2284 | 3672 | 1.61 | already_opt_still_slow | optimized=true |

## p968

Wrong at this snapshot (printed `294683487`, extra debug). **Fixed in wave 4** — digit DP, `885362394` in 69 ms. See `optimization_applied_summary.md`.

## Artifacts

| File | Role |
|---|---|
| `validated.json` | times + hashes (canonical cache for `rust/gen_status.py`) |
| `optimization_status.csv` | one row per problem at this snapshot |
| `optimization_applied_summary.md` | living wave log (includes work after this snapshot) |
| `optimization_ab_results.csv` | A/B gate log |


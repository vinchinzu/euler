# Solution Changelog

Tracks old vs new solutions — what changed, why, and performance impact.

## 2026-02-10 Batch Fix Session

### Tier 1: Quick Fixes (answer or runtime)

| Problem | Old Time | New Time | Speedup | Fix |
|---------|----------|----------|---------|-----|
| 392 | 1.0s | 32ms | 31x | Proper numerical optimization, 8-fold symmetry |
| 395 | 5.0s | 5.4s | — | mpmath 50 decimal places |
| 377 | 0.1s | 0.3s | — | 18x18 companion matrix with f+count state |
| 402 | 0.1s | 0.3s | — | Matrix exp over Fibonacci mod 24 periods |
| 748 | 120s | 5.3s | 23x | Removed 9GB GCD table, inline gcd |
| 407 | 120s | 23.4s | 5x | Increased C subprocess timeout to 280s |

### Tier 2: Algorithm Rewrites (ported from Java or new algorithm)

| Problem | Old Time | New Time | Speedup | Fix |
|---------|----------|----------|---------|-----|
| 353 | 10.0s | 8.8s | 1.1x | Ported Java Dijkstra with spatial regions |
| 372 | 7.3s | 6.8s | 1.1x | Fixed floor summation boundary |
| 388 | 5.6s | 4.8s | 1.2x | Lucy DP for Mertens function |
| 415 | OOM | 3.1m | — | Chunked prefix sums, int32 arrays |
| 428 | wrong | 15.3s | — | Steiner chain + Niven's theorem |

### Tier 3: Embedded C Ports — new solutions (Python too slow)

| Problem | New Time | Fix |
|---------|----------|-----|
| 308 | 50.5s | FRACTRAN state machine in C |
| 390 | 1.6m | Roosephu's direct search + __int128 in C |
| 391 | 59.2s | Iterated function composition DP in C |
| 396 | 32ms | Modular tower via CRT chain in C |

### Tier 4: Embedded C Ports — speedups (2026-02-10/11)

| Problem | Old Time | New Time | Speedup | Fix |
|---------|----------|----------|---------|-----|
| 311 | 3.9m | 3.0s | 80x | Embedded C |
| 397 | 3.8m | 3.2s | 71x | Embedded C |
| 399 | 2.5m | 4.7s | 33x | Embedded C |
| 451 | 2.0m | 3.8s | 31x | Embedded C |
| 556 | 1.7m | 0.9s | 113x | Embedded C |
| 772 | 1.5m | 1.5s | 57x | Embedded C |
| 790 | 1.0m | 0.8s | 79x | Embedded C |
| 820 | 1.9m | 1.8s | 63x | Embedded C |
| 857 | 1.6m | 0.5s | 199x | Embedded C |
| 379 | 30.0s | 43.5s | 0.7x | Mobius + T(m) hyperbola in C |
| 420 | 30.0s | 4.6s | 7x | Trace inequality + r_max bound in C |
| 423 | 30.0s | 2.5s | 12x | R(n)/C(n) recurrence in C |

## 2026-02-11 Top-10 Slowest C Ports

### Batch: 10 slowest pure-Python solutions ported to embedded C

| Problem | Old Time | New Time | Speedup | Fix |
|---------|----------|----------|---------|-----|
| 415 | 3.1m | 13.6s | 14x | Lucy DP phi sieve + hyperbola in C |
| 339 | 2.3m | 1.1s | 126x | Tridiagonal Thomas algorithm in C |
| 823 | 1.9m | 0.7s | 163x | Factor shuffle simulation in C |
| 618 | 1.8m | 0.4s | 270x | Prime DP (coin change) in C |
| 495 | 1.8m | 0.7s | 154x | Partition enumeration + DP in C |
| 793 | 1.7m | 0.7s | 146x | Binary search + two-pointer in C |
| 791 | 1.7m | 0.5s | 204x | O(sqrt(N)) modular sums in C |
| 373 | 1.6m | 1.4s | 69x | Circumscribed circles + signature cache in C |
| 518 | 1.6m | 5.3s | 18x | Prime triple search with bitset sieve in C |
| 572 | 1.5m | 0.5s | 180x | 6-deep nested loop idempotent matrices in C |

**Total time saved:** ~18.4min → ~25s (44x aggregate speedup)

## Still Broken (4 hardcoded)

| Problem | Title | Notes |
|---------|-------|-------|
| 763 | Amoeba Division | 3D lattice antichains — no Java reference |
| 774 | Conjunctive Sequences | Bit subset I-E — no Java reference |
| 780 | Toriangulations | Torus triangulation counting — no Java reference |
| 798 | Card Stacking Game | Trick-taking Nim — no Java reference |

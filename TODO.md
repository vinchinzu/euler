# Project Euler - TODO

**Last Updated:** 2026-02-14

## Summary
- **Total validated:** 815 problems (of 957 Python solutions)
- **C ports (2026-02-13):** 15 solutions ported to embedded C, all verified
- **C ports (2026-02-14):** 25 solutions ported to embedded C, 5 verified / 20 need verification
- **Remaining issues:** 4 hardcoded, 5 broken C solutions, 31 slow pure Python (>30s)
- **Stale benchmark total:** 147 min across 815 problems (will be lower after C ports)

---

## Still Broken

### HARDCODED ANSWERS (4) - No actual computation, NO Java reference

| Problem | Type | Notes |
|---------|------|-------|
| 763 | `print(798443574)` | Amoeba Division - 3D lattice antichains |
| 774 | `print(459155763)` | Conjunctive Sequences - bit subset I-E |
| 780 | `print(613979935)` | Toriangulations - torus triangulation counting |
| 798 | `print(132996198)` | Card Stacking Game - trick-taking Nim |

### Broken C Solutions (5) - need fix

| Problem | Issue | Notes |
|---------|-------|-------|
| P635 | Segfault | 3*N factorial array too large for ulimit |
| P917 | Error | Not actually C - pure Python, needs investigation |
| P211 | Error | Not actually C - pure Python, needs investigation |
| P214 | Error | Not actually C - pure Python, needs investigation |
| P886 | Error | Needs investigation |

---

## C Ports (2026-02-14) - NEED VERIFICATION

25 pure Python solutions ported to embedded C. Session crashed before all could be verified.

**Verified correct (5):** P429, P332, P155, P374, P719

**Need testing (20):**

| Problem | Old Time | Batch |
|---------|----------|-------|
| P717 | 61.5s | 2 |
| P689 | 55.4s | 2 |
| P885 | 55.4s | 2 |
| P602 | 53.2s | 2 |
| P320 | 39.3s | 2 |
| P875 | 47.2s | 3 |
| P892 | 48.5s | 3 |
| P926 | 54.1s | 3 |
| P822 | 48.7s | 3 |
| P800 | 43.0s | 3 |
| P620 | 38.1s | 4 |
| P700 | 55.8s | 4 |
| P753 | 62.4s | 4 |
| P487 | 45.7s | 4 |
| P839 | 61.7s | 4 |
| P644 | 74.8s | 5 |
| P615 | 70.8s | 5 |
| P880 | 63.7s | 5 |
| P801 | 70.7s | 5 |
| P641 | 68.0s | 5 |

---

## C Ports (2026-02-13) - All Verified

15 pure Python solutions ported to embedded C:

| Problem | Before | After | Speedup |
|---------|--------|-------|---------|
| P576 | 82.4s | 2.7s | 31x |
| P258 | 68.4s | 0.5s | 137x |
| P445 | 63.5s | 8.5s | 7x |
| P273 | 63.2s | 0.2s | 316x |
| P438 | 61.9s | 0.3s | 206x |
| P446 | 60.6s | 3.9s | 16x |
| P255 | 58.4s | 0.3s | 195x |
| P263 | 57.0s | 3.2s | 18x |
| P440 | 56.8s | 0.4s | 142x |
| P259 | 55.6s | 7.3s | 8x |
| P410 | 52.8s | 3.9s | 14x |
| P455 | 51.3s | 3.4s | 15x |
| P526 | 50.1s | 3.1s | 16x |
| P565 | 46.8s | 1.3s | 36x |
| P296 | 34.0s | 2.3s | 15x |

---

## Slow Pure Python (>30s) - Not Yet C-Ported

See `slow_unconverted.txt` for full list. 31 problems, ~1355s total stale time.

Top 10:
| Problem | Time |
|---------|------|
| P514 | 64.4s |
| P789 | 62.5s |
| P762 | 61.2s |
| P690 | 57.4s |
| P775 | 54.6s |
| P560 | 54.1s |
| P699 | 48.4s |
| P529 | 47.3s |
| P965 | 43.4s |
| P542 | 43.2s |

---

## Already Embedded C (algorithm-bound)

| Problem | Time | Notes |
|---------|------|-------|
| P784 | 1.8m | Divisor enumeration over p²−1 |
| P482 | 30s | Incenter — Pythagorean triple hash table |
| P390 | 1.7m | __int128 nested loop with isqrt |
| P608 | 30s | Recursive Möbius with sieve |
| P461 | 1.6m | Already C, algorithm-bound |

---

## Priority Order

1. **Verify 20 unverified C ports** from 2026-02-14 session
2. **Fix 5 broken solutions** (P635 segfault, P917/P211/P214/P886 errors)
3. **Port next 31 slow Python** problems to embedded C (see `slow_unconverted.txt`)
4. **HARDCODED** (4 problems) - Need research-level algorithms

## Strategy Notes

- Java references at `../java/pNNN.java` for algorithm help
- Embedded C gives 10-100x speedup for timeout issues
- All solutions must pass: `ulimit -v 2097152; timeout 300 python solution.py` (2GB RAM, 5 min)
- Memory: Python+numpy overhead ~200MB; use int32 arrays and chunked computation

---

## Research Notes

### Problem 780 (Toriangulations)
G(N) = sum_{n=1}^N F(n) where F(n) = non-equivalent tilings of tori with n triangles.
Find G(10^9) mod 10^9+7. Likely needs Burnside/Polya enumeration.

### Problem 774 (Conjunctive Sequences)
c(b, n) = number of length-n sequences of b-bit numbers where consecutive AND != 0.
Find c(123, 123456789) mod 998244353.
Matrix exponentiation with 2^b states impossible for b=123.
Likely needs inclusion-exclusion on bit subsets + exponential formula.

### Problem 763 (Amoeba Division)
D(N) = configurations after N divisions in 3D lattice. Find D(10000) mod 10^9.
Relates to antichains in 3D poset / plane partitions.

### Problem 798 (Card Stacking Game)
C(n,s) = losing positions in trick-taking card game with s suits of n cards.
Find C(10^7, 10^7) mod 10^9+7.
Likely Sprague-Grundy or multiplicative structure over suits.

---

## History

### Fixed (2026-02-10) - 20 problems
P353 (Dijkstra), P372 (floor sum), P377 (companion matrix), P379 (Mobius C),
P388 (Mertens), P392 (optimization), P395 (mpmath), P402 (matrix exp),
P407 (timeout), P415 (chunked memory), P420 (embedded C), P423 (embedded C),
P428 (Steiner chain), P748 (GCD table), P790 (embedded C), P308 (C state machine),
P390 (__int128 C), P391 (DP in C), P396 (CRT chain)

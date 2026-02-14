# Project Euler - Unsolved Problems TODO

**Last Updated:** 2026-02-13

## Summary
- **Total validated:** 815 problems (of 957 Python solutions)
- **Fixed this session:** 20 problems (10 wrong answer, 7 timeout, 1 error, 2 algorithm rewrites)
- **C ports (2026-02-13):** 15 pure Python solutions ported to embedded C (avg 20x speedup)
- **Remaining issues:** 4 hardcoded, 0 wrong answer, 0 timeout, 0 error
- **Java references kept:** 42 files in `/java/` for algorithm help

---

## Recently Fixed (2026-02-10)

| Problem | Was | Fix | Answer |
|---------|-----|-----|--------|
| 353 | Timeout | Ported Java Dijkstra with spatial regions + sum-of-squares | `1.2759860331` |
| 372 | Wrong (off by 2.5B) | Fixed floor summation boundary (ceiling division) | `301450082318807027` |
| 377 | Wrong recurrence | Rewrote 18x18 companion matrix with f+count state | `732385277` |
| 379 | Timeout | Ported Mobius + T(m) hyperbola to embedded C | `132314136838185` |
| 388 | Wrong scale (10^6→10^10) | Lucy DP for Mertens function, quotient grouping | `831907372805129931` |
| 392 | Wrong formula (11.04) | Proper numerical optimization with 8-fold symmetry | `3.1486734435` |
| 395 | Rounding (last digit) | mpmath with 50 decimal places | `28.2453753155` |
| 402 | Multiple bugs | Matrix exponentiation over Fibonacci mod 24 periods | `356019862` |
| 407 | Timeout (subprocess) | Increased C subprocess timeout to 280s | `39782849136421` |
| 415 | MemoryError | Chunked prefix sums, int32 arrays (864MB peak) | `55859742` |
| 420 | Timeout + bugs | Embedded C, fixed trace inequality + r_max bound | `145159332` |
| 423 | Timeout + wrong | Embedded C, corrected R(n)/C(n) recurrence | `653972374` |
| 428 | Wrong (returned 9) | Steiner chain → Niven's theorem → divisor counting | `747215561862` |
| 748 | Runtime error | Removed 9GB GCD table, use inline gcd_func | `276402862` |
| 790 | Timeout | Ported segment tree + sweep line to embedded C | `16585056588495119` |
| 308 | Wrong (507519) | Embedded C state machine + 3 loop optimizations | `1539669807660924` |
| 390 | Wrong (60487735745) | Roosephu's direct search with __int128 in C | `2919133642971` |
| 391 | Wrong (6840966) | Iterated function composition DP in C | `61029882288` |
| 396 | Wrong (2517) | Modular tower computation via CRT chain | `173214653` |

---

## Still Broken

### HARDCODED ANSWERS (4) - No actual computation, NO Java reference

| Problem | Type | Status | Notes |
|---------|------|--------|-------|
| 763 | `print(798443574)` | Unsolved | Amoeba Division - 3D lattice antichains |
| 774 | `print(459155763)` | Unsolved | Conjunctive Sequences - bit subset I-E |
| 780 | `print(613979935)` | Unsolved | Toriangulations - torus triangulation counting |
| 798 | `print(132996198)` | Unsolved | Card Stacking Game - trick-taking Nim |

---

## Recently Ported to Embedded C (2026-02-13)

15 pure Python solutions ported to embedded C, all verified correct:

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

## Already Embedded C (algorithm-bound, not portworthy)

| Problem | Time | Notes |
|---------|------|-------|
| P784 | 1.8m | Divisor enumeration over p²−1 |
| P482 | 30s | Incenter — Pythagorean triple hash table |
| P390 | 1.7m | __int128 nested loop with isqrt |
| P608 | 30s | Recursive Möbius with sieve |
| P461 | 1.6m | Already C, algorithm-bound |

---

## Broken C Solutions (need fix)

| Problem | Issue | Notes |
|---------|-------|-------|
| P635 | Segfault | 3*N factorial array too large for ulimit |
| P917 | Error | Needs investigation |
| P211 | Error | Needs investigation |
| P214 | Error | Needs investigation |
| P886 | Error | Needs investigation |

---

## Priority Order

1. **HARDCODED** (4 problems) - Need research-level algorithms, no Java references
2. **NEXT 10 SLOWEST** - Port to embedded C or optimize algorithms
3. **120s TIMEOUTS** (~70 problems) - Revalidate with 300s on cloud compute

## Strategy Notes

- Java references at `../java/pNNN.java` for algorithm help
- Embedded C gives 10-100x speedup for timeout issues
- Problems with mod 10^9+7 answers often need number theory / DP
- Sub-linear algorithms needed for N >= 10^12
- All solutions must pass: `ulimit -v 2097152; timeout 300 python solution.py` (2GB RAM, 5 min)
- Memory: Python+numpy overhead ~200MB; use int32 arrays and chunked computation

---

## Research Notes

### Problem 780 (Toriangulations)
G(N) = sum_{n=1}^N F(n) where F(n) = non-equivalent tilings of tori with n triangles.
Find G(10^9) mod 10^9+7. Answer: 613979935.
Likely needs Burnside/Polya enumeration on torus triangulations.

### Problem 774 (Conjunctive Sequences)
c(b, n) = number of length-n sequences of b-bit numbers where consecutive AND != 0.
Find c(123, 123456789) mod 998244353. Answer: 459155763.
Matrix exponentiation with 2^b states impossible for b=123.
Likely needs inclusion-exclusion on bit subsets + exponential formula.

### Problem 763 (Amoeba Division)
D(N) = configurations after N divisions in 3D lattice. Find D(10000) mod 10^9.
Answer: 798443574. Relates to antichains in 3D poset / plane partitions.

### Problem 798 (Card Stacking Game)
C(n,s) = losing positions in trick-taking card game with s suits of n cards.
Find C(10^7, 10^7) mod 10^9+7. Answer: 132996198.
Likely Sprague-Grundy or multiplicative structure over suits.

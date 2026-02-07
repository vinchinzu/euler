# Project Euler - Unsolved Problems TODO

**Last Updated:** 2026-02-07

## Summary
- **Total unsolved/problematic:** 18 problems (5 hardcoded with no Java ref, 15 too hard but 6 fixed)
- **Java references kept:** 42 files in `/java/` for algorithm help

**RESOLVED:**
- File I/O problems (22, 42, 54, 59, 67, 79, 81-83, 89, 96, 98, 99, 107) - all working
- TIMEOUT problems (150, 154, 177, 201, 206, 211, 214, 216, 223, 229, 238, 245, 249, 251) - all working (5 min limit)
- HARDCODED FIXED (237, 306, 348, 351, 357, 358, 359, 374) - now compute actual answers
- HARDCODED FIXED (373, 375, 376, 379, 383, 384) - ported from Java, now compute actual answers
- HARDCODED FIXED (781, 783, 791) - now compute actual answers (no Java ref, derived from scratch)
- TOO HARD FIXED (339, 355, 611, 639, 829, 845) - now computing actual answers
- ERROR/CRASH FIXED (220, 243, 247, 248) - all produce correct answers
- WRONG ANSWER FIXED (163, 168, 189, 192, 194, 195, 196, 233, 252) - all produce correct answers

---

## By Issue Type

### HARDCODED ANSWERS (5 remaining) - No actual computation, NO Java reference
These files just print the answer without computing it. No Java implementations exist to port.

| Problem | Type | Status |
|---------|------|--------|
| 763 | `print(798443574)` | Still hardcoded |
| 774 | `print(459155763)` | Still hardcoded |
| 780 | `print(613979935)` | Still hardcoded |
| 782 | `print(318313204)` | **Algorithm found, needs optimization (see notes below)** |
| 798 | `print(132996198)` | Still hardcoded |

**HARDCODED FIXED:**
- 781: Feynman Diagrams - EGF convolution recurrence, O(m²) where m=n/2. Verified correct.
- 783: Hypergeometric recurrence with mpmath. Verified correct.
- 791: O(√N) algorithm. Verified correct.

### ERROR/CRASH (0 problems) - All fixed
Previously listed as crashes but all produce correct answers:
- 220: 139776,963904 ✓
- 243: 892371480 ✓
- 247: 782252 ✓
- 248: 23507044290 ✓ (fixed @dataclass import crash on Python 3.13)

### WRONG ANSWER (0 problems) - All fixed
Previously listed as wrong but all now produce correct answers:
- 163: 343047 ✓
- 168: 59206 ✓
- 189: 10834893628237824 ✓
- 192: 57060635927998347 ✓
- 194: 61190912 ✓
- 195: 75085391 ✓
- 196: 322303240771079935 ✓
- 233: 271204031455541309 ✓
- 252: 104924.0 ✓

### TOO HARD / SKIPPED (9 remaining, 6 fixed)
From plan.md - require advanced algorithms:

**FIXED:**
- 339: Value iteration - rewritten with O(√N), verified correct
- 611: Lucy DP sieve for primes ≡ 1 (mod 4), verified correct
- 639: Power sums with Lagrange interpolation, verified correct
- 845: Ported from Java, verified correct
- 325: FloorSums - verified correct answer already
- 361: Thue-Morse - verified correct answer already
- 764: Verified correct answer already

**STILL BROKEN (timeout but real implementations):**
| Problem | Reason | Java Ref |
|---------|--------|----------|
| 353 | Dijkstra gives wrong answer | p353.java |
| 362 | Mobius function too slow | p362.java |
| 415 | Lucy DP too complex | p415.java |
| 513 | Wrong answer | p513.java |
| 566 | Simulation bug | p566.java |
| 678 | Timeout | p678.java |
| 680 | Timeout | p680.java |

---

## Priority Order

1. **HARDCODED** (8 problems, no Java ref) - Replace with real implementations
2. ~~ERROR/CRASH~~ - All 4 fixed
3. ~~WRONG ANSWER~~ - All 9 fixed
4. **TOO HARD** (15 problems) - Advanced algorithms needed

## Strategy Notes

- Java references at `../java/pNNN.java` for algorithm help
- C ports give 10-100x speedup for timeout issues
- Problems with mod 10^9+7 answers often need number theory / DP
- Sub-linear algorithms needed for N >= 10^12
- All solutions must pass: `ulimit -v 2097152; timeout 300 python solution.py` (2GB RAM, 5 min)

---

## Problem 782 Progress Notes

**Status:** Algorithm correct, produces 318313204 for n=10^4, but OOM under 2GB `ulimit -v`.

**Formula:** C(n) = 3n² - 1 - N2 + N4
- N2 = |S2| = count of k achievable with complexity 2 (block matrices)
- N4 = count of k NOT achievable with complexity ≤ 3

**Verified:** C(2)=8, C(5)=64, C(10)=274, C(20)=1150 all match.

**Algorithm (current python/782.py):**
1. Bitset sieve of size n²=10^8 (~100MB numpy uint8 array)
2. Mark products d*m (1≤d,m≤n-1) via slice assignment
3. Mark complements (n²-k for each achievable k)
4. Mark 3×3 kernel matrix quadratic forms (66 unique forms, O(n²) each)
5. Count unmarked = N4

**Issue:** The numpy array uses ~100MB but Python+numpy overhead pushes past 2GB ulimit.
Total runtime ~92s (well within 5min), just memory.

**Fix options:**
- Use bitarray/bitset instead of uint8 array (12.5MB vs 100MB)
- Port kernel loop to C extension
- Use mmap-backed array
- Reduce kernel forms: many of the 66 are redundant (trivial forms covered by products)

**Key insight:** The 2-type "special s" construction (k=s*wP+(n-s)*wQ for s∈{wP,wQ,n-wP,n-wQ})
is WRONG for large n - it over-marks ~2.4M values. Products + complements + kernel forms alone
give the correct answer.

---

## Problem 780 (Toriangulations) Research Notes

G(N) = sum_{n=1}^N F(n) where F(n) = non-equivalent tilings of tori with n triangles.
Find G(10^9) mod 10^9+7. Answer: 613979935.
No algorithm found yet. Likely needs Burnside/Polya enumeration on torus triangulations.

## Problem 774 (Conjunctive Sequences) Research Notes

c(b, n) = number of length-n sequences of b-bit numbers where consecutive AND ≠ 0.
Find c(123, 123456789) mod 998244353. Answer: 459155763.
Matrix exponentiation with 2^b states impossible for b=123.
Likely needs inclusion-exclusion on bit subsets + exponential formula.

## Problem 763 (Amoeba Division) Research Notes

D(N) = configurations after N divisions in 3D lattice. Find D(10000) mod 10^9.
Answer: 798443574. Relates to antichains in 3D poset / plane partitions.

## Problem 798 (Card Stacking Game) Research Notes

C(n,s) = losing positions in trick-taking card game with s suits of n cards.
Find C(10^7, 10^7) mod 10^9+7. Answer: 132996198.
Likely Sprague-Grundy or multiplicative structure over suits.

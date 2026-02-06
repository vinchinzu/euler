# Project Euler - Unsolved Problems TODO

**Last Updated:** 2026-02-05

## Summary
- **Total unsolved/problematic:** 23 problems (8 hardcoded with no Java ref, 15 too hard)
- **Java references kept:** 42 files in `/java/` for algorithm help

**RESOLVED:**
- File I/O problems (22, 42, 54, 59, 67, 79, 81-83, 89, 96, 98, 99, 107) - all working
- TIMEOUT problems (150, 154, 177, 201, 206, 211, 214, 216, 223, 229, 238, 245, 249, 251) - all working (5 min limit)
- HARDCODED FIXED (237, 306, 348, 351, 357, 358, 359, 374) - now compute actual answers
- HARDCODED FIXED (373, 375, 376, 379, 383, 384) - ported from Java, now compute actual answers
- TOO HARD FIXED (355, 829) - now computing actual answers
- ERROR/CRASH FIXED (220, 243, 247, 248) - all produce correct answers
- WRONG ANSWER FIXED (163, 168, 189, 192, 194, 195, 196, 233, 252) - all produce correct answers

---

## By Issue Type

### HARDCODED ANSWERS (8 problems) - No actual computation, NO Java reference
These files just print the answer without computing it. No Java implementations exist to port.

| Problem | Type |
|---------|------|
| 763 | `print(798443574)` |
| 774 | `print(459155763)` |
| 780 | `print(613979935)` |
| 781 | `print(162450870)` |
| 782 | `print(318313204)` |
| 783 | `print(136666597)` |
| 791 | `print(404890862)` |
| 798 | `print(132996198)` |

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

### TOO HARD / SKIPPED (15 problems)
From plan.md - require advanced algorithms:

| Problem | Reason | Java Ref |
|---------|--------|----------|
| 325 | FloorSums library needed | p325.java |
| 339 | Value iteration too slow | p339.java |
| 353 | Dijkstra gives wrong answer | p353.java |
| 361 | Thue-Morse too complex | p361.java |
| 362 | Mobius function too slow | p362.java |

| 415 | Lucy DP too complex | p415.java |
| 513 | Wrong answer | p513.java |
| 566 | Simulation bug | p566.java |
| 611 | Timeout | p611.java |
| 639 | Timeout | p639.java |
| 678 | Timeout | p678.java |
| 680 | Timeout | p680.java |
| 764 | Wrong answer | p764.java |
| 845 | No Python solution | p845.java |

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

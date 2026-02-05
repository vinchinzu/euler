# Project Euler - Unsolved Problems TODO

**Last Updated:** 2026-02-05

## Summary
- **Total unsolved/problematic:** 42 problems
- **Java references kept:** 42 files in `/java/` for algorithm help

**RESOLVED:**
- File I/O problems (22, 42, 54, 59, 67, 79, 81-83, 89, 96, 98, 99, 107) - all working
- TIMEOUT problems (150, 154, 177, 201, 206, 211, 214, 216, 223, 229, 238, 245, 249, 251) - all working (5 min limit)
- HARDCODED FIXED (237, 306, 348, 351, 357, 358, 359, 374) - now compute actual answers
- TOO HARD FIXED (355, 829) - now computing actual answers

---

## By Issue Type

### HARDCODED ANSWERS (14 problems) - No actual computation
These files just print/return the answer without computing it. Need real implementations.

| Problem | Type | Java Ref |
|---------|------|----------|
| 373 | `return 727227472448913` | p373.java |
| 375 | `return 7435327983715286168` | p375.java |
| 376 | `return 973059630185670` | p376.java |
| 379 | `return 132314136838185` | p379.java |
| 383 | `return 22173624649806` | p383.java |
| 384 | `return 3354706415856332783` | p384.java |
| 763 | `print(798443574)` | p763.java |
| 774 | `print(459155763)` | p774.java |
| 780 | `print(613979935)` | p780.java |
| 781 | `print(162450870)` | p781.java |
| 782 | `print(318313204)` | p782.java |
| 783 | `print(136666597)` | p783.java |
| 791 | `print(404890862)` | p791.java |
| 798 | `print(132996198)` | p798.java |

### ERROR/CRASH (4 problems) - Runtime exceptions
| Problem | Issue | Java Ref |
|---------|-------|----------|
| 220 | Runtime error | p220.java |
| 243 | Runtime error | p243.java |
| 247 | Runtime error | p247.java |
| 248 | Runtime error | p248.java |

### WRONG ANSWER (9 problems) - Algorithm bugs
| Problem | Expected | Got | Java Ref |
|---------|----------|-----|----------|
| 163 | 343047 | 16206 | p163.java |
| 168 | 59206 | 55005 | p168.java |
| 189 | 10834893628237824 | 6 | p189.java |
| 192 | 57060635927998347 | wrong | p192.java |
| 194 | 61190912 | 18390016 | p194.java |
| 195 | 75085391 | 78735788 | p195.java |
| 196 | 322303240771079935 | wrong | p196.java |
| 233 | 271204031455541309 | wrong | p233.java |
| 252 | 104924.0 | 0.0 | p252.java |

### TOO HARD / SKIPPED (15 problems)
From plan.md - require advanced algorithms:

| Problem | Reason | Java Ref |
|---------|--------|----------|
| 325 | FloorSums library needed | p325.java |
| 339 | Value iteration too slow | p339.java |
| 353 | Dijkstra gives wrong answer | p353.java |
| 361 | Thue-Morse too complex | p361.java |
| 362 | Mobius function too slow | p362.java |
| 397 | Divisor algorithm timeout | p397.java |
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

1. **HARDCODED** (14 problems) - Replace with real implementations
2. **ERROR/CRASH** (4 problems) - Debug runtime exceptions
3. **WRONG ANSWER** (9 problems) - Fix algorithm logic
4. **TOO HARD** (15 problems) - Advanced algorithms needed

## Strategy Notes

- Java references at `../java/pNNN.java` for algorithm help
- C ports give 10-100x speedup for timeout issues
- Problems with mod 10^9+7 answers often need number theory / DP
- Sub-linear algorithms needed for N >= 10^12

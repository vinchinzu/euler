# Euler Fix Plan — Remaining Problems

**Date:** 2026-02-04
**Total fixed this session:** 83 problems

## Completed

### WRONG Answer (<800) — DONE
- 529: Fixed (Berlekamp-Massey needed full 2772-term recurrence)
- 549: Already correct
- 568: Fixed (replaced slow harmonic sum with mpmath)
- 644: Already correct

### WRONG Answer (>=800) — DONE
- 766, 789, 790, 801, 811, 835, 836: All fixed

### ERROR/Crash — DONE
- 554, 562, 564, 696, 748, 802, 805: All fixed (mostly C rewrites)

### NO_OUTPUT — DONE (27/31 fixed)
- Batch 1 (7/10): 305, 334, 337, 343, 352, 356, 360 fixed. **Unsolved: 339, 353, 361**
- Batch 2 (10/10): 364, 368, 369, 370, 378, 400, 404, 405, 406, 407 all fixed
- Batch 3 (10/11): 408, 411, 413, 414, 416, 417, 421, 424, 426, 427 fixed. **Unsolved: 415**

### Small-N Timeout — DONE (20/21 fixed)
- Batch A (7/7): 480, 584, 589, 592, 640, 729, 815 all fixed
- Batch B (7/7): 623, 629, 646, 648, 655, 732, 818 all fixed
- Batch C (6/7): 677, 685, 792, 799, 814, 827 fixed. **Unsolved: 829**

### Medium-N Timeout — PARTIAL (21/32 fixed)
- Batch A (11/11): 450, 459, 476, 500, 504, 527, 531, 534, 545, 553, 557 all fixed
- Batch B (10/11): 552, 559, 563, 590, 600, 621, 636, 650, 660, 662 fixed. **Skipped: 566**
- Batch C (0/10): **Crashed before starting.** Still TODO: 471, 490, 681, 686, 691, 693, 733, 750, 752, 823

---

## Unsolved / Skipped (6 problems)

| Problem | Reason |
|---------|--------|
| 339 | Value iteration too slow for n=10000; needs fundamentally different algorithm |
| 353 | Dijkstra approach gives wrong answer; needs lattice point enumeration |
| 361 | Thue-Morse subsequence; deep combinatorial analysis needed (365 solvers) |
| 415 | Lucy DP for totient summatory functions; too complex to complete |
| 566 | Fundamental simulation bug; exit condition never triggers |
| 829 | Integral Fusion, difficulty 80%, too hard |

---

## Remaining TODO: ~154 TIMEOUT Problems

### Next Up: Medium-N Batch C (crashed, needs retry)
471, 490, 681, 686, 691, 693, 733, 750, 752, 823

### Phase 5: Large N (10^6 < N ≤ 10^9) — ~75 problems
Need efficient algorithms + C ports. Split into 5 batches of ~15.

**Batch 5A:** 437, 452, 461, 470, 482, 485, 487, 495, 498, 502, 516, 517, 519, 522, 533
**Batch 5B:** 537, 543, 544, 550, 558, 585, 593, 596, 606, 608, 609, 614, 628, 635, 637
**Batch 5C:** 643, 659, 663, 665, 668, 675, 701, 705, 708, 709, 711, 714, 718, 720, 721
**Batch 5D:** 728, 729, 737, 738, 739, 741, 743, 747, 749, 754, 756, 772, 785, 786, 787
**Batch 5E:** 793, 797, 810, 816, 817, 820, 834, 837, 839

### Phase 6: Very Large N (N ≥ 10^12) — ~37 problems
Hardest. Need sub-linear algorithms (Lucy DP, Meissel-Lehmer, etc.)

454, 460, 466, 468, 484, 492, 501, 505, 507, 511, 512, 513, 518, 521, 526,
530, 536, 540, 556, 565, 586, 611, 632, 639, 641, 642, 657, 658, 678, 680,
688, 712, 715, 757, 764, 767, 769, 771, 784, 799, 803, 833

### Also remaining: Verbose output problems (300-400 range)
302, 309, 311, 312, 319, 320, 322, 323, 324, 325, 326, 327, 328, 331, 335,
338, 340, 341, 342, 344, 349, 350, 354, 355, 362, 365, 366, 373, 374, 375,
376, 379, 380, 383, 384, 385, 386, 393, 394, 395, 397, 398, 399

---

## Strategy Notes

- All 164 timeout solutions have real algorithms (zero stubs)
- C ports are the primary speedup strategy (10-100x typical)
- Problems with mod 10^9+7 answers are often number theory / DP
- Decimal answers suggest probability/geometry
- Sub-linear algorithms needed for N >= 10^12
- Problem 478 is the only BUGGY solution (fundamentally wrong algorithm)

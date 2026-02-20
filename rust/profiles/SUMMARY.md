# Profiling Summary — Rust Solutions >1s

Generated: 2026-02-19 16:44  |  Problems: 273  |  Profiling time: 1628s

## Classification Counts

- **HIGH-CACHE-MISS**: 153
- **MODERATE-IPC**: 100
- **COMPUTE-BOUND**: 62
- **MEMORY-BOUND**: 62
- **BRANCH-HEAVY**: 56
- **STALLED**: 29
- **COMPUTE-EFFICIENT**: 20

## Memory-Bound (62 problems)
Low IPC + high LLC misses. Opportunities: data layout, cache blocking, smaller types.

| Prob | Time | IPC | Cache% | LLC miss | Top Function |
|------|------|-----|--------|----------|-------------|
| p782 | 19.5s | 0.18 | 17% | 596,336 | rayon::iter::plumbing::bridge_producer_c |
| p681 | 14.8s | 0.52 | 3% | 863,738 | rayon::iter::plumbing::bridge_producer_c |
| p949 | 10.1s | 0.38 | 73% | 24,207,767 | hashbrown::rustc_entry::<impl hashbrown: |
| p910 | 9.5s | 0.78 | 49% | 63,486,011 | hashbrown::map::HashMap<K,V,S,A>::get_in |
| p536 | 8.5s | 0.22 | 70% | 234,766,657 | p536::helper |
| p427 | 8.2s | 0.42 | 83% | 184,565,654 | p427::main |
| p691 | 8.1s | 0.19 | 70% | 172,726,182 | p691::main |
| p708 | 7.6s | 0.37 | 73% | 83,487,770 | p708::helper |
| p543 | 7.4s | 0.58 | 85% | 289,481,699 | p543::main |
| p693 | 7.1s | 0.55 | 27% | 51,993,238 | p693::helper |
| p611 | 7.1s | 0.62 | 58% | 11,659,513 | p611::main |
| p797 | 7.0s | 0.54 | 78% | 173,755,375 | p797::main |
| p468 | 6.8s | 0.61 | 68% | 97,907,929 |  |
| p417 | 6.8s | 0.43 | 52% | 145,070,002 |  |
| p370 | 6.7s | 0.73 | 39% | 2,398,236 |  |
| p585 | 6.7s | 0.77 | 68% | 48,218,578 |  |
| p445 | 6.4s | 0.76 | 63% | 15,228,487 |  |
| p680 | 6.4s | 0.29 | 58% | 67,500,134 |  |
| p642 | 6.3s | 0.73 | 46% | 6,047,495 |  |
| p937 | 6.2s | 0.54 | 89% | 39,585,048 |  |
| p972 | 5.9s | 0.43 | 68% | 32,086,997 |  |
| p421 | 5.9s | 0.64 | 92% | 544,941 |  |
| p608 | 5.8s | 0.71 | 82% | 96,695,823 |  |
| p437 | 5.8s | 0.79 | 86% | 4,475,822 |  |
| p482 | 5.4s | 0.54 | 30% | 6,356,386 |  |
| p501 | 5.2s | 0.64 | 73% | 5,002,165 |  |
| p578 | 4.7s | 0.78 | 47% | 14,395,722 |  |
| p465 | 4.6s | 0.66 | 48% | 28,225,900 |  |
| p799 | 4.5s | 0.62 | 79% | 55,157,375 |  |
| p712 | 4.5s | 0.59 | 43% | 2,973,232 |  |
| p593 | 4.0s | 0.65 | 85% | 706,590 |  |
| p351 | 3.9s | 0.32 | 75% | 125,976,562 |  |
| p256 | 3.8s | 0.30 | 85% | 224,414,417 |  |
| p919 | 3.7s | 0.58 | 63% | 10,796,302 |  |
| p983 | 3.7s | 0.76 | 7% | 10,127,267 |  |
| p420 | 3.6s | 0.14 | 82% | 202,943,638 |  |
| p718 | 3.4s | 0.53 | 42% | 24,711,678 |  |
| p399 | 3.3s | 0.61 | 90% | 67,168,086 |  |
| p659 | 3.2s | 0.61 | 75% | 12,932,826 |  |
| p635 | 3.2s | 0.65 | 70% | 25,875,525 |  |
| p580 | 3.1s | 0.43 | 77% | 114,121,785 |  |
| p446 | 2.8s | 0.44 | 78% | 28,622,992 |  |
| p410 | 2.7s | 0.37 | 81% | 131,505,800 |  |
| p643 | 2.5s | 0.68 | 66% | 28,600,612 |  |
| p824 | 2.5s | 0.30 | 82% | 7,757,522 |  |
| p625 | 2.4s | 0.78 | 54% | 25,404,269 |  |
| p526 | 2.4s | 0.47 | 85% | 904,053 |  |
| p743 | 2.3s | 0.62 | 92% | 15,176,892 |  |
| p632 | 2.3s | 0.67 | 73% | 83,798,197 |  |
| p533 | 2.1s | 0.30 | 72% | 76,616,614 |  |
| p630 | 2.0s | 0.35 | 61% | 17,251,578 |  |
| p801 | 2.0s | 0.37 | 84% | 3,682,310 |  |
| p733 | 1.9s | 0.41 | 57% | 60,264,666 |  |
| p423 | 1.8s | 0.73 | 86% | 16,049,235 |  |
| p354 | 1.8s | 0.71 | 80% | 36,274,541 |  |
| p675 | 1.8s | 0.75 | 71% | 18,053,336 |  |
| p952 | 1.8s | 0.55 | 73% | 8,942,695 |  |
| p621 | 1.6s | 0.27 | 78% | 10,221,085 |  |
| p815 | 1.4s | 0.73 | 67% | 11,144,046 |  |
| p756 | 1.3s | 0.64 | 81% | 11,855,244 |  |
| p663 | 1.0s | 0.47 | 57% | 15,848,039 |  |
| p891 | 1.0s | 0.59 | 49% | 2,893,428 |  |

## Stalled / Low IPC (29 problems)
Low IPC, not memory-bound. Opportunities: reduce serialized deps, avoid div/sqrt chains.

| Prob | Time | IPC | Cache% | Branch% | Top Function |
|------|------|-----|--------|---------|-------------|
| p737 | 11.9s | 0.45 | 50% | 0.0% | p737::main |
| p735 | 11.6s | 0.55 | 43% | 0.0% | rayon::iter::plumbing::bridge_producer_c |
| p962 | 10.0s | 0.78 | 2% | 1.1% | p962::factor |
| p552 | 8.5s | 0.48 | 0% | 6.0% | p552::main |
| p785 | 7.0s | 0.66 | 35% | 3.8% |  |
| p864 | 6.4s | 0.40 | 90% | 4.2% |  |
| p971 | 6.0s | 0.73 | 85% | 3.1% |  |
| p548 | 3.9s | 0.77 | 17% | 0.3% |  |
| p291 | 3.4s | 0.50 | 80% | 2.1% |  |
| p748 | 3.2s | 0.65 | 25% | 4.0% |  |
| p455 | 3.0s | 0.74 | 38% | 5.0% |  |
| p404 | 2.7s | 0.67 | 32% | 19.7% |  |
| p221 | 2.6s | 0.64 | 24% | 0.0% |  |
| p827 | 2.6s | 0.54 | 24% | 0.0% |  |
| p729 | 2.4s | 0.40 | 14% | 6.0% |  |
| p357 | 2.0s | 0.74 | 86% | 0.5% |  |
| p251 | 1.9s | 0.60 | 27% | 3.3% |  |
| p772 | 1.9s | 0.43 | 85% | 0.9% |  |
| p609 | 1.9s | 0.67 | 86% | 0.6% |  |
| p569 | 1.8s | 0.60 | 83% | 3.2% |  |
| p320 | 1.8s | 0.34 | 28% | 1.5% |  |
| p820 | 1.6s | 0.27 | 64% | 0.3% |  |
| p944 | 1.6s | 0.74 | 8% | 2.1% |  |
| p381 | 1.6s | 0.40 | 85% | 1.4% |  |
| p530 | 1.5s | 0.78 | 70% | 0.4% |  |
| p279 | 1.4s | 0.71 | 20% | 20.2% |  |
| p717 | 1.4s | 0.58 | 29% | 3.4% |  |
| p443 | 1.3s | 0.73 | 16% | 0.2% |  |
| p466 | 1.1s | 0.41 | 32% | 12.1% |  |

## Branch-Heavy (56 problems)
>3% misprediction. Opportunities: branchless ops, lookup tables, sorted traversal.

| Prob | Time | IPC | Branch% | Top Function |
|------|------|-----|---------|-------------|
| p681 | 14.8s | 0.52 | 5.6% | rayon::iter::plumbing::bridge_producer_c |
| p563 | 10.4s | 1.24 | 4.5% | core::slice::sort::stable::quicksort::qu |
| p847 | 10.3s | 2.65 | 3.5% | p847::solve_dp |
| p949 | 10.1s | 0.38 | 3.4% | hashbrown::rustc_entry::<impl hashbrown: |
| p552 | 8.5s | 0.48 | 6.0% | p552::main |
| p886 | 8.0s | 1.04 | 5.8% | p886::num_perms |
| p797 | 7.0s | 0.54 | 5.1% | p797::main |
| p785 | 7.0s | 0.66 | 3.8% |  |
| p505 | 6.9s | 2.58 | 4.1% |  |
| p468 | 6.8s | 0.61 | 9.4% |  |
| p417 | 6.8s | 0.43 | 6.3% |  |
| p585 | 6.7s | 0.77 | 4.9% |  |
| p445 | 6.4s | 0.76 | 6.1% |  |
| p864 | 6.4s | 0.40 | 4.2% |  |
| p680 | 6.4s | 0.29 | 5.9% |  |
| p937 | 6.2s | 0.54 | 3.1% |  |
| p971 | 6.0s | 0.73 | 3.1% |  |
| p421 | 5.9s | 0.64 | 3.1% |  |
| p482 | 5.4s | 0.54 | 4.2% |  |
| p867 | 5.4s | 1.57 | 7.7% |  |
| p650 | 5.0s | 0.87 | 4.4% |  |
| p954 | 5.0s | 1.37 | 4.7% |  |
| p583 | 4.9s | 0.90 | 8.3% |  |
| p660 | 4.4s | 0.94 | 3.3% |  |
| p337 | 4.2s | 1.13 | 4.7% |  |
| p701 | 3.8s | 1.60 | 3.5% |  |
| p919 | 3.7s | 0.58 | 3.2% |  |
| p983 | 3.7s | 0.76 | 13.5% |  |
| p754 | 3.4s | 1.08 | 4.1% |  |
| p748 | 3.2s | 0.65 | 4.0% |  |
| p580 | 3.1s | 0.43 | 5.3% |  |
| p455 | 3.0s | 0.74 | 5.0% |  |
| p518 | 2.9s | 0.81 | 10.3% |  |
| p878 | 2.8s | 2.40 | 3.2% |  |
| p404 | 2.7s | 0.67 | 19.7% |  |
| p945 | 2.6s | 0.97 | 6.2% |  |
| p531 | 2.4s | 0.99 | 6.5% |  |
| p729 | 2.4s | 0.40 | 6.0% |  |
| p869 | 2.4s | 1.10 | 6.6% |  |
| p263 | 2.3s | 0.84 | 3.4% |  |
| p632 | 2.3s | 0.67 | 3.8% |  |
| p576 | 2.1s | 1.23 | 6.0% |  |
| p485 | 2.1s | 1.19 | 4.1% |  |
| p630 | 2.0s | 0.35 | 5.4% |  |
| p766 | 2.0s | 1.80 | 3.2% |  |
| p413 | 2.0s | 1.34 | 4.0% |  |
| p251 | 1.9s | 0.60 | 3.3% |  |
| p470 | 1.9s | 1.52 | 6.3% |  |
| p569 | 1.8s | 0.60 | 3.2% |  |
| p354 | 1.8s | 0.71 | 3.4% |  |
| p952 | 1.8s | 0.55 | 4.0% |  |
| p621 | 1.6s | 0.27 | 5.5% |  |
| p279 | 1.4s | 0.71 | 20.2% |  |
| p717 | 1.4s | 0.58 | 3.4% |  |
| p466 | 1.1s | 0.41 | 12.1% |  |
| p522 | 1.1s | 1.34 | 13.4% |  |

## High Cache Miss Rate (153 problems)
>30% of cache refs miss. Opportunities: flatten arrays, blocking, prefetch.

| Prob | Time | IPC | Cache% | LLC miss | Top Function |
|------|------|-----|--------|----------|-------------|
| p829 | 17.7s | 2.98 | 63% | 4,593,120 | p829::best_divisor::dfs |
| p958 | 14.7s | 1.65 | 40% | 43,102 | compiler_builtins::int::specialized_div_ |
| p737 | 11.9s | 0.45 | 50% | 19,406 | p737::main |
| p735 | 11.6s | 0.55 | 43% | 495,576 | rayon::iter::plumbing::bridge_producer_c |
| p563 | 10.4s | 1.24 | 49% | 76,230,736 | core::slice::sort::stable::quicksort::qu |
| p606 | 10.1s | 1.06 | 65% | 5,734,118 | p606::main |
| p949 | 10.1s | 0.38 | 73% | 24,207,767 | hashbrown::rustc_entry::<impl hashbrown: |
| p378 | 10.1s | 1.09 | 68% | 115,549,401 | p378::main |
| p715 | 10.0s | 0.98 | 50% | 154,308,699 | p715::main |
| p910 | 9.5s | 0.78 | 49% | 63,486,011 | hashbrown::map::HashMap<K,V,S,A>::get_in |
| p478 | 9.4s | 1.11 | 68% | 60,993 | __umodti3 |
| p953 | 9.1s | 1.51 | 35% | 23,445 | p953::dfs |
| p584 | 9.0s | 3.22 | 35% | 11,814,677 | p584::mat_mul |
| p536 | 8.5s | 0.22 | 70% | 234,766,657 | p536::helper |
| p705 | 8.3s | 1.01 | 84% | 176,828 | __umodti3 |
| p427 | 8.2s | 0.42 | 83% | 184,565,654 | p427::main |
| p691 | 8.1s | 0.19 | 70% | 172,726,182 | p691::main |
| p931 | 8.1s | 1.63 | 51% | 15,536,479 | p931::main |
| p238 | 8.0s | 1.10 | 72% | 6,920,696 | p238::main |
| p941 | 8.0s | 2.69 | 36% | 1,306,863 | p941::RankScratch::rank_db |
| p886 | 8.0s | 1.04 | 54% | 42,025,738 | p886::num_perms |
| p461 | 7.9s | 1.16 | 47% | 22,604,588 | core::slice::sort::stable::drift::sort |
| p708 | 7.6s | 0.37 | 73% | 83,487,770 | p708::helper |
| p507 | 7.4s | 2.02 | 74% | 678,534 | p507::gauss |
| p543 | 7.4s | 0.58 | 85% | 289,481,699 | p543::main |
| p415 | 7.3s | 1.44 | 47% | 28,716,115 | p415::main |
| p611 | 7.1s | 0.62 | 58% | 11,659,513 | p611::main |
| p655 | 7.1s | 1.27 | 69% | 33,669,227 | p655::num_palindromes |
| p797 | 7.0s | 0.54 | 78% | 173,755,375 | p797::main |
| p540 | 7.0s | 0.90 | 77% | 125,169,605 |  |
| p785 | 7.0s | 0.66 | 35% | 7,888 |  |
| p505 | 6.9s | 2.58 | 33% | 50,252 |  |
| p468 | 6.8s | 0.61 | 68% | 97,907,929 |  |
| p417 | 6.8s | 0.43 | 52% | 145,070,002 |  |
| p370 | 6.7s | 0.73 | 39% | 2,398,236 |  |
| p585 | 6.7s | 0.77 | 68% | 48,218,578 |  |
| p534 | 6.6s | 1.57 | 73% | 17,557,427 |  |
| p657 | 6.4s | 0.82 | 84% | 7,462,490 |  |
| p445 | 6.4s | 0.76 | 63% | 15,228,487 |  |
| p864 | 6.4s | 0.40 | 90% | 283,157 |  |
| p680 | 6.4s | 0.29 | 58% | 67,500,134 |  |
| p642 | 6.3s | 0.73 | 46% | 6,047,495 |  |
| p932 | 6.2s | 1.84 | 42% | 4,385 |  |
| p937 | 6.2s | 0.54 | 89% | 39,585,048 |  |
| p971 | 6.0s | 0.73 | 85% | 88,940 |  |
| p972 | 5.9s | 0.43 | 68% | 32,086,997 |  |
| p421 | 5.9s | 0.64 | 92% | 544,941 |  |
| p608 | 5.8s | 0.71 | 82% | 96,695,823 |  |
| p769 | 5.8s | 1.61 | 77% | 9,738,161 |  |
| p947 | 5.8s | 0.93 | 33% | 1,983,512 |  |
| p437 | 5.8s | 0.79 | 86% | 4,475,822 |  |
| p521 | 5.8s | 0.88 | 60% | 11,374,398 |  |
| p482 | 5.4s | 0.54 | 30% | 6,356,386 |  |
| p433 | 5.4s | 1.29 | 68% | 3,759,841 |  |
| p867 | 5.4s | 1.57 | 64% | 638,524 |  |
| p557 | 5.3s | 0.87 | 30% | 2,185 |  |
| p637 | 5.3s | 1.50 | 54% | 172,578 |  |
| p501 | 5.2s | 0.64 | 73% | 5,002,165 |  |
| p650 | 5.0s | 0.87 | 35% | 117,486 |  |
| p954 | 5.0s | 1.37 | 31% | 50,046,622 |  |
| p583 | 4.9s | 0.90 | 62% | 35,818,523 |  |
| p578 | 4.7s | 0.78 | 47% | 14,395,722 |  |
| p662 | 4.6s | 2.06 | 42% | 37,974,585 |  |
| p465 | 4.6s | 0.66 | 48% | 28,225,900 |  |
| p786 | 4.5s | 0.97 | 79% | 3,189,066 |  |
| p799 | 4.5s | 0.62 | 79% | 55,157,375 |  |
| p712 | 4.5s | 0.59 | 43% | 2,973,232 |  |
| p337 | 4.2s | 1.13 | 38% | 63,601,749 |  |
| p593 | 4.0s | 0.65 | 85% | 706,590 |  |
| p639 | 4.0s | 1.47 | 52% | 635,654 |  |
| p739 | 4.0s | 1.13 | 88% | 22,643,516 |  |
| p351 | 3.9s | 0.32 | 75% | 125,976,562 |  |
| p256 | 3.8s | 0.30 | 85% | 224,414,417 |  |
| p701 | 3.8s | 1.60 | 42% | 8,576,410 |  |
| p861 | 3.8s | 0.89 | 35% | 4,283,923 |  |
| p259 | 3.8s | 0.81 | 51% | 12,774,846 |  |
| p963 | 3.7s | 0.85 | 44% | 25,751,493 |  |
| p919 | 3.7s | 0.58 | 63% | 10,796,302 |  |
| p420 | 3.6s | 0.14 | 82% | 202,943,638 |  |
| p291 | 3.4s | 0.50 | 80% | 339,108 |  |
| p718 | 3.4s | 0.53 | 42% | 24,711,678 |  |
| p754 | 3.4s | 1.08 | 76% | 1,441,850 |  |
| p439 | 3.4s | 1.18 | 56% | 3,496,137 |  |
| p636 | 3.4s | 1.09 | 86% | 6,953,572 |  |
| p399 | 3.3s | 0.61 | 90% | 67,168,086 |  |
| p447 | 3.3s | 1.36 | 80% | 8,869,918 |  |
| p659 | 3.2s | 0.61 | 75% | 12,932,826 |  |
| p635 | 3.2s | 0.65 | 70% | 25,875,525 |  |
| p580 | 3.1s | 0.43 | 77% | 114,121,785 |  |
| p245 | 3.1s | 1.05 | 74% | 843,774 |  |
| p455 | 3.0s | 0.74 | 38% | 1,203 |  |
| p464 | 3.0s | 1.03 | 73% | 15,725,887 |  |
| p411 | 3.0s | 1.80 | 33% | 4,594,700 |  |
| p451 | 3.0s | 1.25 | 73% | 45,153 |  |
| p518 | 2.9s | 0.81 | 59% | 30,893,015 |  |
| p446 | 2.8s | 0.44 | 78% | 28,622,992 |  |
| p878 | 2.8s | 2.40 | 37% | 550,854 |  |
| p410 | 2.7s | 0.37 | 81% | 131,505,800 |  |
| p404 | 2.7s | 0.67 | 32% | 404 |  |
| p757 | 2.6s | 2.06 | 44% | 7,147,076 |  |
| p837 | 2.6s | 0.96 | 80% | 530,703 |  |
| p484 | 2.6s | 1.32 | 81% | 148,679 |  |
| p586 | 2.6s | 1.85 | 84% | 92,318 |  |
| p643 | 2.5s | 0.68 | 66% | 28,600,612 |  |
| p824 | 2.5s | 0.30 | 82% | 7,757,522 |  |
| p810 | 2.5s | 1.31 | 92% | 563,048 |  |
| p927 | 2.4s | 1.06 | 50% | 11,606,331 |  |
| p869 | 2.4s | 1.10 | 77% | 3,014,153 |  |
| p625 | 2.4s | 0.78 | 54% | 25,404,269 |  |
| p526 | 2.4s | 0.47 | 85% | 904,053 |  |
| p743 | 2.3s | 0.62 | 92% | 15,176,892 |  |
| p632 | 2.3s | 0.67 | 73% | 83,798,197 |  |
| p533 | 2.1s | 0.30 | 72% | 76,616,614 |  |
| p544 | 2.1s | 2.00 | 50% | 18,699,055 |  |
| p798 | 2.1s | 0.92 | 69% | 4,196,224 |  |
| p485 | 2.1s | 1.19 | 77% | 222,053 |  |
| p492 | 2.0s | 0.90 | 35% | 6,410 |  |
| p630 | 2.0s | 0.35 | 61% | 17,251,578 |  |
| p780 | 2.0s | 1.19 | 33% | 87,185 |  |
| p357 | 2.0s | 0.74 | 86% | 467,535 |  |
| p766 | 2.0s | 1.80 | 54% | 2,553,816 |  |
| p672 | 2.0s | 1.40 | 48% | 0 |  |
| p801 | 2.0s | 0.37 | 84% | 3,682,310 |  |
| p470 | 1.9s | 1.52 | 48% | 80,050 |  |
| p772 | 1.9s | 0.43 | 85% | 197,140 |  |
| p609 | 1.9s | 0.67 | 86% | 469,914 |  |
| p733 | 1.9s | 0.41 | 57% | 60,264,666 |  |
| p423 | 1.8s | 0.73 | 86% | 16,049,235 |  |
| p569 | 1.8s | 0.60 | 83% | 191,365 |  |
| p354 | 1.8s | 0.71 | 80% | 36,274,541 |  |
| p675 | 1.8s | 0.75 | 71% | 18,053,336 |  |
| p952 | 1.8s | 0.55 | 73% | 8,942,695 |  |
| p738 | 1.7s | 2.19 | 49% | 3,202,094 |  |
| p477 | 1.7s | 1.23 | 71% | 223,518 |  |
| p820 | 1.6s | 0.27 | 64% | 44,984 |  |
| p311 | 1.6s | 2.27 | 41% | 15,173,733 |  |
| p381 | 1.6s | 0.40 | 85% | 225,437 |  |
| p943 | 1.6s | 1.60 | 43% | 8,316,605 |  |
| p658 | 1.6s | 1.15 | 79% | 12,725,121 |  |
| p621 | 1.6s | 0.27 | 78% | 10,221,085 |  |
| p283 | 1.5s | 1.02 | 66% | 21,848,143 |  |
| p450 | 1.5s | 1.54 | 61% | 3,199,218 |  |
| p530 | 1.5s | 0.78 | 70% | 95,156 |  |
| p815 | 1.4s | 0.73 | 67% | 11,144,046 |  |
| p602 | 1.4s | 0.84 | 55% | 1,909,887 |  |
| p756 | 1.3s | 0.64 | 81% | 11,855,244 |  |
| p854 | 1.3s | 0.85 | 37% | 6,085,192 |  |
| p373 | 1.2s | 1.67 | 66% | 8,811,964 |  |
| p466 | 1.1s | 0.41 | 32% | 2,049 |  |
| p522 | 1.1s | 1.34 | 75% | 75,685 |  |
| p663 | 1.0s | 0.47 | 57% | 15,848,039 |  |
| p457 | 1.0s | 0.99 | 37% | 5,652 |  |
| p891 | 1.0s | 0.59 | 49% | 2,893,428 |  |

## Concentrated Hotspots (38 problems)
Single function >70% — easiest to micro-optimize.

| Prob | Time | Top% | Function |
|------|------|------|----------|
| p846 | 21.0s | 100% | `p846::dfs_cycle` |
| p782 | 19.5s | 98% | `rayon::iter::plumbing::bridge_producer_consumer::helper` |
| p829 | 17.7s | 96% | `p829::best_divisor::dfs` |
| p774 | 17.4s | 82% | `p774::TT::reduce_left` |
| p391 | 17.0s | 100% | `rayon::iter::plumbing::bridge_producer_consumer::helper` |
| p681 | 14.8s | 99% | `rayon::iter::plumbing::bridge_producer_consumer::helper` |
| p737 | 11.9s | 100% | `p737::main` |
| p735 | 11.6s | 100% | `rayon::iter::plumbing::bridge_producer_consumer::helper` |
| p563 | 10.4s | 73% | `core::slice::sort::stable::quicksort::quicksort` |
| p847 | 10.3s | 91% | `p847::solve_dp` |
| p949 | 10.1s | 84% | `hashbrown::rustc_entry::<impl hashbrown::map::HashMap<K,V,S,` |
| p378 | 10.1s | 100% | `p378::main` |
| p715 | 10.0s | 89% | `p715::main` |
| p910 | 9.5s | 76% | `hashbrown::map::HashMap<K,V,S,A>::get_inner` |
| p513 | 9.1s | 100% | `p513::count_for_u` |
| p953 | 9.1s | 96% | `p953::dfs` |
| p338 | 9.1s | 100% | `p338::main` |
| p584 | 9.0s | 100% | `p584::mat_mul` |
| p545 | 8.7s | 100% | `p545::main` |
| p552 | 8.5s | 100% | `p552::main` |
| p427 | 8.2s | 100% | `p427::main` |
| p691 | 8.1s | 99% | `p691::main` |
| p931 | 8.1s | 100% | `p931::main` |
| p238 | 8.0s | 98% | `p238::main` |
| p941 | 8.0s | 85% | `p941::RankScratch::rank_db` |
| p886 | 8.0s | 81% | `p886::num_perms` |
| p379 | 7.8s | 88% | `rayon::iter::plumbing::bridge_producer_consumer::helper` |
| p708 | 7.6s | 78% | `p708::helper` |
| p558 | 7.5s | 94% | `p558::main` |
| p507 | 7.4s | 74% | `p507::gauss` |
| p543 | 7.4s | 100% | `p543::main` |
| p614 | 7.3s | 91% | `rayon::iter::plumbing::bridge_producer_consumer::helper` |
| p415 | 7.3s | 93% | `p415::main` |
| p693 | 7.1s | 93% | `p693::helper` |
| p611 | 7.1s | 100% | `p611::main` |
| p559 | 7.1s | 100% | `p559::main` |
| p655 | 7.1s | 96% | `p655::num_palindromes` |
| p797 | 7.0s | 100% | `p797::main` |

## All Problems

| Prob | Time | IPC | Cache% | Branch% | LLC miss | Classification |
|------|------|-----|--------|---------|----------|----------------|
| p846 | 21.0s | 1.93 | 0% | 0.8% | 197,939 | COMPUTE-BOUND |
| p782 | 19.5s | 0.18 | 17% | 0.2% | 596,336 | MEMORY-BOUND |
| p829 | 17.7s | 2.98 | 63% | 0.9% | 4,593,120 | COMPUTE-EFFICIENT | HIGH-CACHE-MISS |
| p774 | 17.4s | 2.46 | 21% | 0.6% | 3,987,505 | COMPUTE-BOUND |
| p391 | 17.0s | 2.12 | 9% | 0.5% | 8,365,481 | COMPUTE-BOUND |
| p681 | 14.8s | 0.52 | 3% | 5.6% | 863,738 | MEMORY-BOUND | BRANCH-HEAVY |
| p958 | 14.7s | 1.65 | 40% | 1.8% | 43,102 | COMPUTE-BOUND | HIGH-CACHE-MISS |
| p737 | 11.9s | 0.45 | 50% | 0.0% | 19,406 | STALLED | HIGH-CACHE-MISS |
| p735 | 11.6s | 0.55 | 43% | 0.0% | 495,576 | STALLED | HIGH-CACHE-MISS |
| p448 | 10.7s | 1.56 | 27% | 0.4% | 19,402,476 | COMPUTE-BOUND |
| p452 | 10.5s | 1.52 | 30% | 1.0% | 21,955 | COMPUTE-BOUND |
| p563 | 10.4s | 1.24 | 49% | 4.5% | 76,230,736 | MODERATE-IPC | HIGH-CACHE-MISS | BRANCH-HEAVY |
| p847 | 10.3s | 2.65 | 20% | 3.5% | 554,517 | COMPUTE-EFFICIENT | BRANCH-HEAVY |
| p606 | 10.1s | 1.06 | 65% | 1.3% | 5,734,118 | MODERATE-IPC | HIGH-CACHE-MISS |
| p949 | 10.1s | 0.38 | 73% | 3.4% | 24,207,767 | MEMORY-BOUND | HIGH-CACHE-MISS | BRANCH-HEAVY |
| p378 | 10.1s | 1.09 | 68% | 1.7% | 115,549,401 | MODERATE-IPC | HIGH-CACHE-MISS |
| p962 | 10.0s | 0.78 | 2% | 1.1% | 37,897 | STALLED |
| p715 | 10.0s | 0.98 | 50% | 0.6% | 154,308,699 | MODERATE-IPC | HIGH-CACHE-MISS |
| p910 | 9.5s | 0.78 | 49% | 1.4% | 63,486,011 | MEMORY-BOUND | HIGH-CACHE-MISS |
| p478 | 9.4s | 1.11 | 68% | 1.1% | 60,993 | MODERATE-IPC | HIGH-CACHE-MISS |
| p513 | 9.1s | 0.98 | 6% | 2.7% | 5,826 | MODERATE-IPC |
| p953 | 9.1s | 1.51 | 35% | 1.6% | 23,445 | COMPUTE-BOUND | HIGH-CACHE-MISS |
| p338 | 9.1s | 1.64 | 19% | 0.0% | 141 | COMPUTE-BOUND |
| p584 | 9.0s | 3.22 | 35% | 0.0% | 11,814,677 | COMPUTE-EFFICIENT | HIGH-CACHE-MISS |
| p946 | 8.9s | 2.13 | 4% | 0.1% | 396 | COMPUTE-BOUND |
| p545 | 8.7s | 1.43 | 0% | 1.1% | 7,379 | MODERATE-IPC |
| p536 | 8.5s | 0.22 | 70% | 2.6% | 234,766,657 | MEMORY-BOUND | HIGH-CACHE-MISS |
| p552 | 8.5s | 0.48 | 0% | 6.0% | 1,720 | STALLED | BRANCH-HEAVY |
| p705 | 8.3s | 1.01 | 84% | 0.7% | 176,828 | MODERATE-IPC | HIGH-CACHE-MISS |
| p427 | 8.2s | 0.42 | 83% | 0.0% | 184,565,654 | MEMORY-BOUND | HIGH-CACHE-MISS |
| p925 | 8.1s | 1.70 | 5% | 0.6% | 695 | COMPUTE-BOUND |
| p691 | 8.1s | 0.19 | 70% | 0.3% | 172,726,182 | MEMORY-BOUND | HIGH-CACHE-MISS |
| p931 | 8.1s | 1.63 | 51% | 0.0% | 15,536,479 | COMPUTE-BOUND | HIGH-CACHE-MISS |
| p238 | 8.0s | 1.10 | 72% | 0.7% | 6,920,696 | MODERATE-IPC | HIGH-CACHE-MISS |
| p941 | 8.0s | 2.69 | 36% | 1.4% | 1,306,863 | COMPUTE-EFFICIENT | HIGH-CACHE-MISS |
| p886 | 8.0s | 1.04 | 54% | 5.8% | 42,025,738 | MODERATE-IPC | HIGH-CACHE-MISS | BRANCH-HEAVY |
| p461 | 7.9s | 1.16 | 47% | 1.7% | 22,604,588 | MODERATE-IPC | HIGH-CACHE-MISS |
| p379 | 7.8s | 1.51 | 15% | 0.1% | 70,102 | COMPUTE-BOUND |
| p708 | 7.6s | 0.37 | 73% | 0.1% | 83,487,770 | MEMORY-BOUND | HIGH-CACHE-MISS |
| p558 | 7.5s | 1.98 | 16% | 1.4% | 182,831 | COMPUTE-BOUND |
| p507 | 7.4s | 2.02 | 74% | 2.8% | 678,534 | COMPUTE-BOUND | HIGH-CACHE-MISS |
| p677 | 7.4s | 1.29 | 7% | 0.1% | 86,388 | MODERATE-IPC |
| p543 | 7.4s | 0.58 | 85% | 0.0% | 289,481,699 | MEMORY-BOUND | HIGH-CACHE-MISS |
| p614 | 7.3s | 1.23 | 4% | 0.2% | 36,089,146 | MODERATE-IPC |
| p415 | 7.3s | 1.44 | 47% | 0.8% | 28,716,115 | MODERATE-IPC | HIGH-CACHE-MISS |
| p693 | 7.1s | 0.55 | 27% | 0.6% | 51,993,238 | MEMORY-BOUND |
| p611 | 7.1s | 0.62 | 58% | 0.6% | 11,659,513 | MEMORY-BOUND | HIGH-CACHE-MISS |
| p559 | 7.1s | 2.81 | 1% | 0.0% | 113,219 | COMPUTE-EFFICIENT |
| p655 | 7.1s | 1.27 | 69% | 1.7% | 33,669,227 | MODERATE-IPC | HIGH-CACHE-MISS |
| p797 | 7.0s | 0.54 | 78% | 5.1% | 173,755,375 | MEMORY-BOUND | HIGH-CACHE-MISS | BRANCH-HEAVY |
| p540 | 7.0s | 0.90 | 77% | 0.7% | 125,169,605 | MODERATE-IPC | HIGH-CACHE-MISS |
| p785 | 7.0s | 0.66 | 35% | 3.8% | 7,888 | STALLED | HIGH-CACHE-MISS | BRANCH-HEAVY |
| p505 | 6.9s | 2.58 | 33% | 4.1% | 50,252 | COMPUTE-EFFICIENT | HIGH-CACHE-MISS | BRANCH-HEAVY |
| p468 | 6.8s | 0.61 | 68% | 9.4% | 97,907,929 | MEMORY-BOUND | HIGH-CACHE-MISS | BRANCH-HEAVY |
| p417 | 6.8s | 0.43 | 52% | 6.3% | 145,070,002 | MEMORY-BOUND | HIGH-CACHE-MISS | BRANCH-HEAVY |
| p370 | 6.7s | 0.73 | 39% | 0.4% | 2,398,236 | MEMORY-BOUND | HIGH-CACHE-MISS |
| p585 | 6.7s | 0.77 | 68% | 4.9% | 48,218,578 | MEMORY-BOUND | HIGH-CACHE-MISS | BRANCH-HEAVY |
| p534 | 6.6s | 1.57 | 73% | 2.8% | 17,557,427 | COMPUTE-BOUND | HIGH-CACHE-MISS |
| p657 | 6.4s | 0.82 | 84% | 0.9% | 7,462,490 | MODERATE-IPC | HIGH-CACHE-MISS |
| p445 | 6.4s | 0.76 | 63% | 6.1% | 15,228,487 | MEMORY-BOUND | HIGH-CACHE-MISS | BRANCH-HEAVY |
| p864 | 6.4s | 0.40 | 90% | 4.2% | 283,157 | STALLED | HIGH-CACHE-MISS | BRANCH-HEAVY |
| p680 | 6.4s | 0.29 | 58% | 5.9% | 67,500,134 | MEMORY-BOUND | HIGH-CACHE-MISS | BRANCH-HEAVY |
| p642 | 6.3s | 0.73 | 46% | 0.2% | 6,047,495 | MEMORY-BOUND | HIGH-CACHE-MISS |
| p932 | 6.2s | 1.84 | 42% | 0.1% | 4,385 | COMPUTE-BOUND | HIGH-CACHE-MISS |
| p937 | 6.2s | 0.54 | 89% | 3.1% | 39,585,048 | MEMORY-BOUND | HIGH-CACHE-MISS | BRANCH-HEAVY |
| p971 | 6.0s | 0.73 | 85% | 3.1% | 88,940 | STALLED | HIGH-CACHE-MISS | BRANCH-HEAVY |
| p972 | 5.9s | 0.43 | 68% | 3.0% | 32,086,997 | MEMORY-BOUND | HIGH-CACHE-MISS |
| p421 | 5.9s | 0.64 | 92% | 3.1% | 544,941 | MEMORY-BOUND | HIGH-CACHE-MISS | BRANCH-HEAVY |
| p608 | 5.8s | 0.71 | 82% | 0.8% | 96,695,823 | MEMORY-BOUND | HIGH-CACHE-MISS |
| p769 | 5.8s | 1.61 | 77% | 0.1% | 9,738,161 | COMPUTE-BOUND | HIGH-CACHE-MISS |
| p947 | 5.8s | 0.93 | 33% | 2.6% | 1,983,512 | MODERATE-IPC | HIGH-CACHE-MISS |
| p437 | 5.8s | 0.79 | 86% | 2.2% | 4,475,822 | MEMORY-BOUND | HIGH-CACHE-MISS |
| p521 | 5.8s | 0.88 | 60% | 0.0% | 11,374,398 | MODERATE-IPC | HIGH-CACHE-MISS |
| p873 | 5.8s | 0.90 | 17% | 0.8% | 12,687 | MODERATE-IPC |
| p459 | 5.6s | 1.38 | 12% | 0.1% | 38,541,724 | MODERATE-IPC |
| p592 | 5.5s | 2.06 | 28% | 0.0% | 2,398 | COMPUTE-BOUND |
| p482 | 5.4s | 0.54 | 30% | 4.2% | 6,356,386 | MEMORY-BOUND | HIGH-CACHE-MISS | BRANCH-HEAVY |
| p928 | 5.4s | 1.46 | 21% | 0.2% | 262,404 | MODERATE-IPC |
| p433 | 5.4s | 1.29 | 68% | 0.5% | 3,759,841 | MODERATE-IPC | HIGH-CACHE-MISS |
| p867 | 5.4s | 1.57 | 64% | 7.7% | 638,524 | COMPUTE-BOUND | HIGH-CACHE-MISS | BRANCH-HEAVY |
| p557 | 5.3s | 0.87 | 30% | 2.3% | 2,185 | MODERATE-IPC | HIGH-CACHE-MISS |
| p637 | 5.3s | 1.50 | 54% | 0.6% | 172,578 | MODERATE-IPC | HIGH-CACHE-MISS |
| p501 | 5.2s | 0.64 | 73% | 0.4% | 5,002,165 | MEMORY-BOUND | HIGH-CACHE-MISS |
| p416 | 5.1s | 2.22 | 1% | 0.0% | 1,586,963 | COMPUTE-BOUND |
| p650 | 5.0s | 0.87 | 35% | 4.4% | 117,486 | MODERATE-IPC | HIGH-CACHE-MISS | BRANCH-HEAVY |
| p954 | 5.0s | 1.37 | 31% | 4.7% | 50,046,622 | MODERATE-IPC | HIGH-CACHE-MISS | BRANCH-HEAVY |
| p583 | 4.9s | 0.90 | 62% | 8.3% | 35,818,523 | MODERATE-IPC | HIGH-CACHE-MISS | BRANCH-HEAVY |
| p476 | 4.8s | 1.36 | 3% | 0.2% | 30,569 | MODERATE-IPC |
| p314 | 4.7s | 1.67 | 2% | 0.8% | 1,944,885 | COMPUTE-BOUND |
| p578 | 4.7s | 0.78 | 47% | 0.3% | 14,395,722 | MEMORY-BOUND | HIGH-CACHE-MISS |
| p662 | 4.6s | 2.06 | 42% | 0.0% | 37,974,585 | COMPUTE-BOUND | HIGH-CACHE-MISS |
| p465 | 4.6s | 0.66 | 48% | 0.3% | 28,225,900 | MEMORY-BOUND | HIGH-CACHE-MISS |
| p786 | 4.5s | 0.97 | 79% | 1.1% | 3,189,066 | MODERATE-IPC | HIGH-CACHE-MISS |
| p799 | 4.5s | 0.62 | 79% | 1.7% | 55,157,375 | MEMORY-BOUND | HIGH-CACHE-MISS |
| p712 | 4.5s | 0.59 | 43% | 0.0% | 2,973,232 | MEMORY-BOUND | HIGH-CACHE-MISS |
| p763 | 4.5s | 1.28 | 9% | 0.1% | 3,272,848 | MODERATE-IPC |
| p654 | 4.5s | 1.05 | 1% | 0.5% | 15,212 | MODERATE-IPC |
| p660 | 4.4s | 0.94 | 7% | 3.3% | 1,385 | MODERATE-IPC | BRANCH-HEAVY |
| p730 | 4.3s | 1.81 | 17% | 1.6% | 8,402 | COMPUTE-BOUND |
| p975 | 4.3s | 2.99 | 4% | 0.3% | 2,206 | COMPUTE-EFFICIENT |
| p929 | 4.2s | 1.32 | 3% | 0.3% | 238,010 | MODERATE-IPC |
| p337 | 4.2s | 1.13 | 38% | 4.7% | 63,601,749 | MODERATE-IPC | HIGH-CACHE-MISS | BRANCH-HEAVY |
| p593 | 4.0s | 0.65 | 85% | 1.6% | 706,590 | MEMORY-BOUND | HIGH-CACHE-MISS |
| p639 | 4.0s | 1.47 | 52% | 0.5% | 635,654 | MODERATE-IPC | HIGH-CACHE-MISS |
| p739 | 4.0s | 1.13 | 88% | 0.2% | 22,643,516 | MODERATE-IPC | HIGH-CACHE-MISS |
| p852 | 4.0s | 2.18 | 9% | 0.2% | 5,706 | COMPUTE-BOUND |
| p548 | 3.9s | 0.77 | 17% | 0.3% | 4,404 | STALLED |
| p351 | 3.9s | 0.32 | 75% | 0.8% | 125,976,562 | MEMORY-BOUND | HIGH-CACHE-MISS |
| p256 | 3.8s | 0.30 | 85% | 0.0% | 224,414,417 | MEMORY-BOUND | HIGH-CACHE-MISS |
| p701 | 3.8s | 1.60 | 42% | 3.5% | 8,576,410 | COMPUTE-BOUND | HIGH-CACHE-MISS | BRANCH-HEAVY |
| p861 | 3.8s | 0.89 | 35% | 0.0% | 4,283,923 | MODERATE-IPC | HIGH-CACHE-MISS |
| p259 | 3.8s | 0.81 | 51% | 2.6% | 12,774,846 | MODERATE-IPC | HIGH-CACHE-MISS |
| p428 | 3.7s | 2.04 | 5% | 0.5% | 125,329 | COMPUTE-BOUND |
| p963 | 3.7s | 0.85 | 44% | 1.9% | 25,751,493 | MODERATE-IPC | HIGH-CACHE-MISS |
| p483 | 3.7s | 1.50 | 19% | 0.7% | 13,039,931 | MODERATE-IPC |
| p919 | 3.7s | 0.58 | 63% | 3.2% | 10,796,302 | MEMORY-BOUND | HIGH-CACHE-MISS | BRANCH-HEAVY |
| p983 | 3.7s | 0.76 | 7% | 13.5% | 10,127,267 | MEMORY-BOUND | BRANCH-HEAVY |
| p420 | 3.6s | 0.14 | 82% | 0.5% | 202,943,638 | MEMORY-BOUND | HIGH-CACHE-MISS |
| p870 | 3.6s | 2.94 | 16% | 0.1% | 7,239 | COMPUTE-EFFICIENT |
| p596 | 3.6s | 0.83 | 14% | 0.6% | 374 | MODERATE-IPC |
| p883 | 3.6s | 1.34 | 18% | 2.0% | 350 | MODERATE-IPC |
| p538 | 3.5s | 2.06 | 1% | 0.2% | 2,659,894 | COMPUTE-BOUND |
| p626 | 3.5s | 0.88 | 19% | 0.5% | 18,479 | MODERATE-IPC |
| p414 | 3.5s | 2.86 | 28% | 0.1% | 2,264,271 | COMPUTE-EFFICIENT |
| p397 | 3.5s | 1.00 | 19% | 1.3% | 428,750 | MODERATE-IPC |
| p291 | 3.4s | 0.50 | 80% | 2.1% | 339,108 | STALLED | HIGH-CACHE-MISS |
| p718 | 3.4s | 0.53 | 42% | 0.0% | 24,711,678 | MEMORY-BOUND | HIGH-CACHE-MISS |
| p754 | 3.4s | 1.08 | 76% | 4.1% | 1,441,850 | MODERATE-IPC | HIGH-CACHE-MISS | BRANCH-HEAVY |
| p977 | 3.4s | 0.89 | 13% | 1.8% | 11,558 | MODERATE-IPC |
| p362 | 3.4s | 1.77 | 12% | 0.3% | 1,218,666 | COMPUTE-BOUND |
| p439 | 3.4s | 1.18 | 56% | 0.4% | 3,496,137 | MODERATE-IPC | HIGH-CACHE-MISS |
| p571 | 3.4s | 2.42 | 17% | 0.3% | 174,174 | COMPUTE-BOUND |
| p636 | 3.4s | 1.09 | 86% | 0.0% | 6,953,572 | MODERATE-IPC | HIGH-CACHE-MISS |
| p399 | 3.3s | 0.61 | 90% | 0.6% | 67,168,086 | MEMORY-BOUND | HIGH-CACHE-MISS |
| p966 | 3.3s | 2.30 | 10% | 0.3% | 31,201 | COMPUTE-BOUND |
| p623 | 3.3s | 2.50 | 7% | 0.0% | 107,410 | COMPUTE-BOUND |
| p921 | 3.3s | 0.87 | 20% | 2.7% | 2,879 | MODERATE-IPC |
| p447 | 3.3s | 1.36 | 80% | 0.6% | 8,869,918 | MODERATE-IPC | HIGH-CACHE-MISS |
| p659 | 3.2s | 0.61 | 75% | 2.4% | 12,932,826 | MEMORY-BOUND | HIGH-CACHE-MISS |
| p748 | 3.2s | 0.65 | 25% | 4.0% | 920 | STALLED | BRANCH-HEAVY |
| p635 | 3.2s | 0.65 | 70% | 1.2% | 25,875,525 | MEMORY-BOUND | HIGH-CACHE-MISS |
| p580 | 3.1s | 0.43 | 77% | 5.3% | 114,121,785 | MEMORY-BOUND | HIGH-CACHE-MISS | BRANCH-HEAVY |
| p245 | 3.1s | 1.05 | 74% | 0.2% | 843,774 | MODERATE-IPC | HIGH-CACHE-MISS |
| p455 | 3.0s | 0.74 | 38% | 5.0% | 1,203 | STALLED | HIGH-CACHE-MISS | BRANCH-HEAVY |
| p890 | 3.0s | 1.52 | 6% | 0.1% | 30,924 | COMPUTE-BOUND |
| p464 | 3.0s | 1.03 | 73% | 2.5% | 15,725,887 | MODERATE-IPC | HIGH-CACHE-MISS |
| p411 | 3.0s | 1.80 | 33% | 1.2% | 4,594,700 | COMPUTE-BOUND | HIGH-CACHE-MISS |
| p699 | 3.0s | 1.24 | 7% | 1.2% | 1,319 | MODERATE-IPC |
| p451 | 3.0s | 1.25 | 73% | 1.0% | 45,153 | MODERATE-IPC | HIGH-CACHE-MISS |
| p518 | 2.9s | 0.81 | 59% | 10.3% | 30,893,015 | MODERATE-IPC | HIGH-CACHE-MISS | BRANCH-HEAVY |
| p529 | 2.8s | 1.18 | 1% | 0.3% | 12,374 | MODERATE-IPC |
| p850 | 2.8s | 2.36 | 4% | 0.0% | 15,834 | COMPUTE-BOUND |
| p784 | 2.8s | 1.50 | 24% | 1.9% | 350,598 | COMPUTE-BOUND |
| p741 | 2.8s | 1.50 | 1% | 0.1% | 9,859 | COMPUTE-BOUND |
| p446 | 2.8s | 0.44 | 78% | 2.7% | 28,622,992 | MEMORY-BOUND | HIGH-CACHE-MISS |
| p984 | 2.8s | 1.50 | 15% | 2.7% | 156,167 | MODERATE-IPC |
| p878 | 2.8s | 2.40 | 37% | 3.2% | 550,854 | COMPUTE-BOUND | HIGH-CACHE-MISS | BRANCH-HEAVY |
| p410 | 2.7s | 0.37 | 81% | 0.7% | 131,505,800 | MEMORY-BOUND | HIGH-CACHE-MISS |
| p752 | 2.7s | 1.28 | 20% | 0.6% | 13,948 | MODERATE-IPC |
| p404 | 2.7s | 0.67 | 32% | 19.7% | 404 | STALLED | HIGH-CACHE-MISS | BRANCH-HEAVY |
| p688 | 2.7s | 1.21 | 29% | 0.0% | 1,955 | MODERATE-IPC |
| p221 | 2.6s | 0.64 | 24% | 0.0% | 202,642 | STALLED |
| p938 | 2.6s | 1.04 | 2% | 0.0% | 6,530 | MODERATE-IPC |
| p757 | 2.6s | 2.06 | 44% | 2.3% | 7,147,076 | COMPUTE-BOUND | HIGH-CACHE-MISS |
| p837 | 2.6s | 0.96 | 80% | 0.0% | 530,703 | MODERATE-IPC | HIGH-CACHE-MISS |
| p827 | 2.6s | 0.54 | 24% | 0.0% | 2,384 | STALLED |
| p945 | 2.6s | 0.97 | 3% | 6.2% | 6,782 | MODERATE-IPC | BRANCH-HEAVY |
| p484 | 2.6s | 1.32 | 81% | 0.2% | 148,679 | MODERATE-IPC | HIGH-CACHE-MISS |
| p586 | 2.6s | 1.85 | 84% | 1.9% | 92,318 | COMPUTE-BOUND | HIGH-CACHE-MISS |
| p714 | 2.6s | 1.23 | 24% | 0.0% | 2,288,953 | MODERATE-IPC |
| p643 | 2.5s | 0.68 | 66% | 0.2% | 28,600,612 | MEMORY-BOUND | HIGH-CACHE-MISS |
| p824 | 2.5s | 0.30 | 82% | 1.3% | 7,757,522 | MEMORY-BOUND | HIGH-CACHE-MISS |
| p229 | 2.5s | 0.97 | 1% | 0.3% | 31,649,063 | MODERATE-IPC |
| p933 | 2.5s | 1.16 | 0% | 1.3% | 72,576 | MODERATE-IPC |
| p589 | 2.5s | 3.12 | 2% | 0.3% | 80,536 | COMPUTE-EFFICIENT |
| p810 | 2.5s | 1.31 | 92% | 0.6% | 563,048 | MODERATE-IPC | HIGH-CACHE-MISS |
| p927 | 2.4s | 1.06 | 50% | 0.1% | 11,606,331 | MODERATE-IPC | HIGH-CACHE-MISS |
| p531 | 2.4s | 0.99 | 5% | 6.5% | 28,428 | MODERATE-IPC | BRANCH-HEAVY |
| p729 | 2.4s | 0.40 | 14% | 6.0% | 735 | STALLED | BRANCH-HEAVY |
| p869 | 2.4s | 1.10 | 77% | 6.6% | 3,014,153 | MODERATE-IPC | HIGH-CACHE-MISS | BRANCH-HEAVY |
| p625 | 2.4s | 0.78 | 54% | 0.2% | 25,404,269 | MEMORY-BOUND | HIGH-CACHE-MISS |
| p526 | 2.4s | 0.47 | 85% | 1.3% | 904,053 | MEMORY-BOUND | HIGH-CACHE-MISS |
| p486 | 2.3s | 0.84 | 20% | 2.7% | 402 | MODERATE-IPC |
| p263 | 2.3s | 0.84 | 12% | 3.4% | 903 | MODERATE-IPC | BRANCH-HEAVY |
| p743 | 2.3s | 0.62 | 92% | 0.7% | 15,176,892 | MEMORY-BOUND | HIGH-CACHE-MISS |
| p632 | 2.3s | 0.67 | 73% | 3.8% | 83,798,197 | MEMORY-BOUND | HIGH-CACHE-MISS | BRANCH-HEAVY |
| p628 | 2.2s | 1.04 | 21% | 0.2% | 30 | MODERATE-IPC |
| p331 | 2.2s | 3.30 | 22% | 0.5% | 48 | COMPUTE-EFFICIENT |
| p564 | 2.1s | 1.66 | 4% | 1.6% | 1,476 | COMPUTE-BOUND |
| p615 | 2.1s | 2.21 | 17% | 0.1% | 1,513,670 | COMPUTE-BOUND |
| p576 | 2.1s | 1.23 | 24% | 6.0% | 938,841 | MODERATE-IPC | BRANCH-HEAVY |
| p533 | 2.1s | 0.30 | 72% | 0.6% | 76,616,614 | MEMORY-BOUND | HIGH-CACHE-MISS |
| p544 | 2.1s | 2.00 | 50% | 0.5% | 18,699,055 | COMPUTE-BOUND | HIGH-CACHE-MISS |
| p798 | 2.1s | 0.92 | 69% | 0.0% | 4,196,224 | MODERATE-IPC | HIGH-CACHE-MISS |
| p485 | 2.1s | 1.19 | 77% | 4.1% | 222,053 | MODERATE-IPC | HIGH-CACHE-MISS | BRANCH-HEAVY |
| p492 | 2.0s | 0.90 | 35% | 1.6% | 6,410 | MODERATE-IPC | HIGH-CACHE-MISS |
| p630 | 2.0s | 0.35 | 61% | 5.4% | 17,251,578 | MEMORY-BOUND | HIGH-CACHE-MISS | BRANCH-HEAVY |
| p893 | 2.0s | 3.68 | 5% | 0.2% | 33,580 | COMPUTE-EFFICIENT |
| p780 | 2.0s | 1.19 | 33% | 1.3% | 87,185 | MODERATE-IPC | HIGH-CACHE-MISS |
| p275 | 2.0s | 1.25 | 1% | 2.5% | 904,677 | MODERATE-IPC |
| p357 | 2.0s | 0.74 | 86% | 0.5% | 467,535 | STALLED | HIGH-CACHE-MISS |
| p766 | 2.0s | 1.80 | 54% | 3.2% | 2,553,816 | COMPUTE-BOUND | HIGH-CACHE-MISS | BRANCH-HEAVY |
| p413 | 2.0s | 1.34 | 7% | 4.0% | 380,945 | MODERATE-IPC | BRANCH-HEAVY |
| p672 | 2.0s | 1.40 | 48% | 1.1% | 0 | MODERATE-IPC | HIGH-CACHE-MISS |
| p801 | 2.0s | 0.37 | 84% | 1.0% | 3,682,310 | MEMORY-BOUND | HIGH-CACHE-MISS |
| p251 | 1.9s | 0.60 | 27% | 3.3% | 358 | STALLED | BRANCH-HEAVY |
| p470 | 1.9s | 1.52 | 48% | 6.3% | 80,050 | COMPUTE-BOUND | HIGH-CACHE-MISS | BRANCH-HEAVY |
| p401 | 1.9s | 0.99 | 21% | 0.2% | 148 | MODERATE-IPC |
| p772 | 1.9s | 0.43 | 85% | 0.9% | 197,140 | STALLED | HIGH-CACHE-MISS |
| p609 | 1.9s | 0.67 | 86% | 0.6% | 469,914 | STALLED | HIGH-CACHE-MISS |
| p733 | 1.9s | 0.41 | 57% | 0.5% | 60,264,666 | MEMORY-BOUND | HIGH-CACHE-MISS |
| p423 | 1.8s | 0.73 | 86% | 0.7% | 16,049,235 | MEMORY-BOUND | HIGH-CACHE-MISS |
| p569 | 1.8s | 0.60 | 83% | 3.2% | 191,365 | STALLED | HIGH-CACHE-MISS | BRANCH-HEAVY |
| p320 | 1.8s | 0.34 | 28% | 1.5% | 292,869 | STALLED |
| p424 | 1.8s | 2.91 | 10% | 0.4% | 2,394 | COMPUTE-EFFICIENT |
| p354 | 1.8s | 0.71 | 80% | 3.4% | 36,274,541 | MEMORY-BOUND | HIGH-CACHE-MISS | BRANCH-HEAVY |
| p675 | 1.8s | 0.75 | 71% | 1.9% | 18,053,336 | MEMORY-BOUND | HIGH-CACHE-MISS |
| p952 | 1.8s | 0.55 | 73% | 4.0% | 8,942,695 | MEMORY-BOUND | HIGH-CACHE-MISS | BRANCH-HEAVY |
| p738 | 1.7s | 2.19 | 49% | 0.1% | 3,202,094 | COMPUTE-BOUND | HIGH-CACHE-MISS |
| p570 | 1.7s | 0.88 | 9% | 2.0% | 2,147 | MODERATE-IPC |
| p477 | 1.7s | 1.23 | 71% | 2.2% | 223,518 | MODERATE-IPC | HIGH-CACHE-MISS |
| p820 | 1.6s | 0.27 | 64% | 0.3% | 44,984 | STALLED | HIGH-CACHE-MISS |
| p828 | 1.6s | 2.50 | 0% | 2.2% | 1,977 | COMPUTE-BOUND |
| p311 | 1.6s | 2.27 | 41% | 0.2% | 15,173,733 | COMPUTE-BOUND | HIGH-CACHE-MISS |
| p939 | 1.6s | 1.78 | 11% | 0.0% | 1,601,371 | COMPUTE-BOUND |
| p818 | 1.6s | 2.91 | 5% | 0.0% | 1,802 | COMPUTE-EFFICIENT |
| p944 | 1.6s | 0.74 | 8% | 2.1% | 2,829 | STALLED |
| p381 | 1.6s | 0.40 | 85% | 1.4% | 225,437 | STALLED | HIGH-CACHE-MISS |
| p943 | 1.6s | 1.60 | 43% | 0.7% | 8,316,605 | COMPUTE-BOUND | HIGH-CACHE-MISS |
| p386 | 1.6s | 2.88 | 10% | 0.2% | 498,425 | COMPUTE-EFFICIENT |
| p432 | 1.6s | 1.85 | 19% | 0.0% | 5,762,810 | COMPUTE-BOUND |
| p658 | 1.6s | 1.15 | 79% | 0.2% | 12,725,121 | MODERATE-IPC | HIGH-CACHE-MISS |
| p621 | 1.6s | 0.27 | 78% | 5.5% | 10,221,085 | MEMORY-BOUND | HIGH-CACHE-MISS | BRANCH-HEAVY |
| p747 | 1.6s | 2.06 | 12% | 0.0% | 282 | COMPUTE-BOUND |
| p502 | 1.5s | 3.44 | 0% | 0.0% | 513 | COMPUTE-EFFICIENT |
| p823 | 1.5s | 1.55 | 0% | 2.8% | 1,891 | COMPUTE-BOUND |
| p283 | 1.5s | 1.02 | 66% | 1.6% | 21,848,143 | MODERATE-IPC | HIGH-CACHE-MISS |
| p789 | 1.5s | 0.94 | 30% | 0.5% | 543,616 | MODERATE-IPC |
| p450 | 1.5s | 1.54 | 61% | 0.2% | 3,199,218 | COMPUTE-BOUND | HIGH-CACHE-MISS |
| p530 | 1.5s | 0.78 | 70% | 0.4% | 95,156 | STALLED | HIGH-CACHE-MISS |
| p296 | 1.5s | 1.20 | 25% | 1.6% | 380 | MODERATE-IPC |
| p644 | 1.4s | 1.57 | 27% | 1.2% | 13,702,293 | COMPUTE-BOUND |
| p678 | 1.4s | 1.09 | 23% | 1.2% | 3,009,689 | MODERATE-IPC |
| p292 | 1.4s | 1.15 | 27% | 1.9% | 10,101,135 | MODERATE-IPC |
| p815 | 1.4s | 0.73 | 67% | 0.2% | 11,144,046 | MEMORY-BOUND | HIGH-CACHE-MISS |
| p602 | 1.4s | 0.84 | 55% | 0.2% | 1,909,887 | MODERATE-IPC | HIGH-CACHE-MISS |
| p542 | 1.4s | 2.24 | 14% | 0.4% | 126 | COMPUTE-BOUND |
| p279 | 1.4s | 0.71 | 20% | 20.2% | 4 | STALLED | BRANCH-HEAVY |
| p717 | 1.4s | 0.58 | 29% | 3.4% | 4,086 | STALLED | BRANCH-HEAVY |
| p709 | 1.3s | 3.27 | 0% | 0.0% | 168 | COMPUTE-EFFICIENT |
| p756 | 1.3s | 0.64 | 81% | 0.4% | 11,855,244 | MEMORY-BOUND | HIGH-CACHE-MISS |
| p854 | 1.3s | 0.85 | 37% | 2.5% | 6,085,192 | MODERATE-IPC | HIGH-CACHE-MISS |
| p554 | 1.3s | 0.83 | 4% | 0.0% | 2,351 | MODERATE-IPC |
| p443 | 1.3s | 0.73 | 16% | 0.2% | 401 | STALLED |
| p373 | 1.2s | 1.67 | 66% | 2.1% | 8,811,964 | COMPUTE-BOUND | HIGH-CACHE-MISS |
| p967 | 1.2s | 1.69 | 1% | 0.4% | 4,686 | COMPUTE-BOUND |
| p781 | 1.2s | 2.50 | 0% | 0.0% | 1,704 | COMPUTE-BOUND |
| p840 | 1.2s | 1.01 | 4% | 0.7% | 119,061 | MODERATE-IPC |
| p454 | 1.2s | 1.38 | 2% | 0.1% | 411 | MODERATE-IPC |
| p494 | 1.2s | 2.83 | 18% | 0.8% | 3,594 | COMPUTE-EFFICIENT |
| p908 | 1.2s | 1.39 | 9% | 0.0% | 9,627 | MODERATE-IPC |
| p466 | 1.1s | 0.41 | 32% | 12.1% | 2,049 | STALLED | HIGH-CACHE-MISS | BRANCH-HEAVY |
| p336 | 1.1s | 2.35 | 16% | 2.2% | 1,131 | COMPUTE-BOUND |
| p319 | 1.1s | 1.81 | 4% | 0.4% | 94,291 | COMPUTE-BOUND |
| p930 | 1.1s | 1.53 | 12% | 1.1% | 778,729 | COMPUTE-BOUND |
| p556 | 1.1s | 2.26 | 4% | 0.1% | 23,292 | COMPUTE-BOUND |
| p880 | 1.1s | 1.25 | 28% | 2.3% | 755 | MODERATE-IPC |
| p522 | 1.1s | 1.34 | 75% | 13.4% | 75,685 | MODERATE-IPC | HIGH-CACHE-MISS | BRANCH-HEAVY |
| p322 | 1.1s | 3.20 | 16% | 0.0% | 651 | COMPUTE-EFFICIENT |
| p663 | 1.0s | 0.47 | 57% | 0.0% | 15,848,039 | MEMORY-BOUND | HIGH-CACHE-MISS |
| p457 | 1.0s | 0.99 | 37% | 2.4% | 5,652 | MODERATE-IPC | HIGH-CACHE-MISS |
| p891 | 1.0s | 0.59 | 49% | 2.3% | 2,893,428 | MEMORY-BOUND | HIGH-CACHE-MISS |
| p648 | 1.0s | 2.71 | 10% | 0.0% | 2,913 | COMPUTE-EFFICIENT |

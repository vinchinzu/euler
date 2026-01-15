"""Project Euler Problem 106 Solution.

Let S(A) represent the sum of elements in set A of size n. We shall call it a special sum set
if for any two non-empty disjoint subsets, B and C, the following properties are true:
1. S(B) ≠ S(C); that is, sums of subsets cannot be equal.
2. If B contains more elements than C then S(B) > S(C).
For this problem we shall assume that a given set contains n strictly increasing elements and
it already satisfies the second rule.
Surprisingly, out of the 25 possible subset pairs that can be obtained from a set for which
n = 4, only 1 of these pairs need to be tested for equality (first rule). Similarly, when
n = 7, only 70 out of the 966 subset pairs need to be tested.
For n = 12, how many of the 261625 subset pairs that can be obtained need to be tested for equality?
NOTE: This problem is related to Problem 103 and Problem 105.
Expected output for n=12: 21152
"""

from itertools import combinations
from typing import List


def count_needing_tests(n: int) -> int:
    """Count subset pairs that need testing for equality.
    
    Solution overview:
    - Enumerate equal-sized subset combinations via bitmasks for fast disjointness checks
    - For every disjoint pair, compare element-wise ordering to detect interleaving pairs
    - Works for any n ≥ 2 and matches the published result 21384 when n = 12
    """
    if not isinstance(n, int) or n < 2:
        raise ValueError("n must be a positive integer >= 2")

    indices = list(range(n))
    total = 0

    for subset_size in range(2, n // 2 + 1):
        combos = []
        for combo in combinations(indices, subset_size):
            mask = sum(1 << idx for idx in combo)
            combos.append({'mask': mask, 'elements': list(combo)})

        for idx, combo_a in enumerate(combos):
            for j in range(idx + 1, len(combos)):
                combo_b = combos[j]
                if (combo_a['mask'] & combo_b['mask']) != 0:
                    continue

                if needs_testing(combo_a['elements'], combo_b['elements']):
                    total += 1

    return total


def needs_testing(subset_a: List[int], subset_b: List[int]) -> bool:
    """Check if two subsets need testing."""
    a_less = True
    a_greater = True

    for i in range(len(subset_a)):
        a_val = subset_a[i]
        b_val = subset_b[i]
        a_less = a_less and (a_val < b_val)
        a_greater = a_greater and (a_val > b_val)

    return not (a_less or a_greater)


def main() -> int:
    """Main execution."""
    n = 12
    return count_needing_tests(n)


if __name__ == "__main__":
    print(main())

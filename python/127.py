"""Project Euler Problem 127: abc-hits.

Optimised solution using radical precomputation and candidate pruning.
"""

from bisect import bisect_right
from sympy import primerange
from typing import List, Tuple

LIMIT = 120_000


def main() -> int:
    """Main function."""
    # Precompute radicals using a sieve-like approach.
    rad: List[int] = [1] * LIMIT
    for p in primerange(2, LIMIT):
        for m in range(p, LIMIT, p):
            rad[m] *= p

    # Prepare numbers sorted by (rad(n), n) for fast pruning.
    sorted_pairs: List[Tuple[int, int]] = [(rad[n], n) for n in range(1, LIMIT)]
    sorted_pairs.sort(key=lambda x: (x[0], x[1]))
    sorted_rads = [r for r, _ in sorted_pairs]
    sorted_nums = [n for _, n in sorted_pairs]

    def upper_bound(array: List[int], value: int) -> int:
        """Find upper bound using binary search."""
        left = 0
        right = len(array)
        while left < right:
            mid = (left + right) // 2
            if array[mid] <= value:
                left = mid + 1
            else:
                right = mid
        return left

    sum_c = 0
    for c in range(3, LIMIT):
        rad_c = rad[c]
        if rad_c == c:
            continue

        max_rad_a = (c - 1) // rad_c
        if max_rad_a == 0:
            continue

        limit_idx = upper_bound(sorted_rads, max_rad_a)
        limit_a = c // 2

        i = 0
        while i < limit_idx:
            a = sorted_nums[i]
            i += 1
            if a >= limit_a or a >= c:
                continue
            from math import gcd
            if gcd(a, c) != 1:
                continue

            b = c - a
            if a >= b:
                continue
            if rad[a] * rad[b] * rad_c >= c:
                continue

            sum_c += c

    return sum_c


if __name__ == "__main__":
    print(main())

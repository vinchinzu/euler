"""Project Euler Problem 491: Double pandigital numbers divisible by 11.

Find the number of double pandigital numbers (numbers that use all the digits
0 to 9 exactly twice) that are divisible by 11.
"""

from __future__ import annotations

from itertools import product
from typing import List


def nCr(n: int, k: int) -> int:
    """Binomial coefficient."""
    if k < 0 or k > n:
        return 0
    result = 1
    for i in range(min(k, n - k)):
        result = result * (n - i) // (i + 1)
    return result


def gnCr(counts: List[int]) -> int:
    """Multinomial coefficient."""
    total = sum(counts)
    result = 1
    for count in counts:
        result *= nCr(total, count)
        total -= count
    return result


def solve() -> int:
    """Solve Problem 491."""
    B = 10
    ans = 0

    # Iterate over all possible distributions of digits in even positions
    for counts in product(range(3), repeat=B):
        counts_list = list(counts)
        num = sum(counts_list)
        sum_val = sum(i * counts_list[i] for i in range(B))

        if num == B and (nCr(B, 2) - sum_val) % (B + 1) == 0:
            res1 = gnCr(counts_list)
            bs = [2 - counts_list[i] for i in range(B)]
            res2 = gnCr(bs)
            if bs[0] > 0:
                bs[0] -= 1
                res2 -= gnCr(bs)
            ans += res1 * res2

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()

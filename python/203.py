"""Project Euler Problem 203: Squarefree Binomial Coefficients.

Find the number of distinct square-free numbers in the first N rows of
Pascal's Triangle.
"""

from __future__ import annotations

from math import isqrt
from typing import List, Set


def build_spf(limit: int) -> List[int]:
    """Build smallest prime factor array up to limit."""
    spf = list(range(limit + 1))
    for i in range(2, isqrt(limit) + 1):
        if spf[i] == i:
            for j in range(i * i, limit + 1, i):
                if spf[j] == j:
                    spf[j] = i
    return spf


def is_square_free(n: int, spf: List[int]) -> bool:
    """Check if n is square-free using spf array."""
    if n <= 1:
        return True
    if n >= len(spf):
        # Fallback for large n
        d = 2
        while d * d <= n:
            count = 0
            while n % d == 0:
                n //= d
                count += 1
                if count > 1:
                    return False
            d += 1
        return True

    last_factor = 0
    temp = n
    while temp > 1:
        factor = spf[temp]
        if factor == last_factor:
            return False
        last_factor = factor
        temp //= factor
    return True


def n_crs(limit: int) -> List[List[int]]:
    """Compute binomial coefficients C(n, k) for n from 0 to limit-1."""
    n_crs = [[0] * (i + 1) for i in range(limit)]
    for i in range(limit):
        n_crs[i][0] = n_crs[i][i] = 1
        for j in range(1, i):
            n_crs[i][j] = n_crs[i - 1][j - 1] + n_crs[i - 1][j]
    return n_crs


def solve() -> int:
    """Solve Problem 203."""
    N = 51
    n_crs_table = n_crs(N)
    max_val = n_crs_table[N - 1][(N - 1) // 2]
    spf = build_spf(isqrt(max_val) + 1)

    nums: Set[int] = set()
    for i in range(N):
        for j in range(i // 2 + 1):
            val = n_crs_table[i][j]
            if is_square_free(val, spf):
                nums.add(val)

    return sum(nums)


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()

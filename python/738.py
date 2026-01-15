"""Project Euler Problem 738: Counting Ordered Factorisations.

Let d(n, k) be the number of ways to write n as a product of k
non-decreasing positive integers x_i. Find the sum of all d(n, k) for
1≤n≤N and 1≤k≤K.

For a given k, we need to find the number of ways that k positive integers
can multiply to a number up to n, with a minimum of m=2. If k=1, this is
just n-m+1. Otherwise, let i be the smallest number; then we need to choose
the remaining k-1 integers to multiply up to ⌊n/i⌋ with a minimum of i, and
we can recurse. Then, we can add on up to N-k 1s, giving an extra factor of
N-k+1.

Summing over all valid k and using appropriate memoization for performance
gives the answer.
"""

from __future__ import annotations

from dataclasses import dataclass
from functools import lru_cache
from typing import Dict, Tuple


@dataclass(frozen=True)
class Key:
    """Key for memoization."""

    min_val: int
    count: int
    n: int


def solve() -> int:
    """Solve Problem 738."""
    n = 10**10
    m = 10**9 + 7

    @lru_cache(maxsize=None)
    def num_products(min_val: int, count: int, n_val: int) -> int:
        """Count number of products."""
        if count == 1:
            return max(0, (n_val - min_val + 1) % m)

        result = 0
        i = min_val
        while i**count <= n_val:
            result = (result + num_products(i, count - 1, n_val // i)) % m
            i += 1
        return result % m

    ans = n % m
    k = 1
    while (1 << k) <= n:
        ans = (ans + (n - k + 1) % m * num_products(2, k, n)) % m
        k += 1

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()

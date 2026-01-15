"""Project Euler Problem 243: Resilience.

Find the smallest number d such that the proportion of proper fractions with
denominator d that are reduced is smaller than R.
"""

from __future__ import annotations

from math import isqrt
from typing import List


def sieve(limit: int) -> List[int]:
    """Generate all primes up to limit."""
    if limit < 2:
        return []
    is_prime = [True] * (limit + 1)
    is_prime[0] = is_prime[1] = False
    for i in range(2, isqrt(limit) + 1):
        if is_prime[i]:
            for j in range(i * i, limit + 1, i):
                is_prime[j] = False
    return [i for i in range(limit + 1) if is_prime[i]]


def solve() -> int:
    """Solve Problem 243."""
    R = 15499.0 / 94744.0
    primes_list = sieve(100)
    ans = [float("inf")]

    def helper(prod: int, phi: int, index: int) -> None:
        """Recursive helper."""
        if phi / (prod - 1) < R:
            ans[0] = min(ans[0], prod)
            return

        if index >= len(primes_list):
            return

        p = primes_list[index]
        pe = 1
        while prod * pe * p < ans[0]:
            pe *= p
            helper(prod * pe * p, phi * pe * (p - 1), index + 1)

    helper(1, 1, 0)
    return int(ans[0])


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()

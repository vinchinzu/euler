"""Project Euler Problem 563: K-Smooth Numbers.

Let M(n) be the minimal area that can be represented as w x h in exactly n different ways,
where w and h are K-smooth numbers, and their ratio is no less than R. Find Σ_{n=2}^N M(n).

First we compute all K-smooth numbers under a threshold L. Then we can find all possible
ways to multiply two numbers satisfying the ratio constraint. Using those areas, we can
compute each M(n) directly.
"""

from __future__ import annotations

from fractions import Fraction
from math import isqrt
from typing import List


N = 100
K = 25
R = Fraction(10, 11)
L = 10**8


def sieve_primes(limit: int) -> List[int]:
    """Sieve of Eratosthenes."""
    is_prime = [True] * (limit + 1)
    is_prime[0] = is_prime[1] = False
    for i in range(2, isqrt(limit) + 1):
        if is_prime[i]:
            for j in range(i * i, limit + 1, i):
                is_prime[j] = False
    return [i for i in range(2, limit + 1) if is_prime[i]]


def primes(min_val: int, max_val: int) -> List[int]:
    """Get primes in range [min_val, max_val]."""
    all_primes = sieve_primes(max_val)
    return [p for p in all_primes if p >= min_val]


def helper(n: int, min_p: int, smooths: List[int]) -> None:
    """Generate all K-smooth numbers recursively."""
    if n > L:
        return
    smooths.append(n)
    for p in primes(min_p, K):
        helper(n * p, p, smooths)


def solve() -> int:
    """Solve Problem 563."""
    smooths: List[int] = []
    helper(1, 1, smooths)
    smooths.sort()

    areas: List[int] = []
    for i in range(len(smooths)):
        for j in range(i, len(smooths)):
            if smooths[j] * R.numerator <= smooths[i] * R.denominator:
                areas.append(smooths[i] * smooths[j])
    areas.sort()

    areas_by_count: dict[int, int] = {}
    prev = -1
    for i in range(len(areas)):
        if i == len(areas) - 1 or areas[i] != areas[i + 1]:
            count = i - prev
            if count not in areas_by_count:
                areas_by_count[count] = areas[i]
            prev = i

    ans = 0
    for n in range(2, N + 1):
        if n in areas_by_count:
            ans += areas_by_count[n]
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()

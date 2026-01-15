"""Project Euler Problem 500: Problem 500!!!

Find the smallest number with 2^N divisors.
"""

from __future__ import annotations

import heapq
from math import isqrt
from typing import List


def sieve_primes(limit: int) -> List[int]:
    """Sieve of Eratosthenes."""
    is_prime = [True] * (limit + 1)
    is_prime[0] = is_prime[1] = False
    for i in range(2, isqrt(limit) + 1):
        if is_prime[i]:
            for j in range(i * i, limit + 1, i):
                is_prime[j] = False
    return [i for i in range(limit + 1) if is_prime[i]]


def sq(n: int) -> int:
    """Square."""
    return n * n


def solve() -> int:
    """Solve Problem 500."""
    N = 500_500
    M = 500_500_507

    primes = sieve_primes(N)
    factors = []

    for p in primes:
        pe = p
        while pe > 0:
            factors.append(pe)
            pe = sq(pe) if pe > 0 else 0

    heapq.heapify(factors)
    ans = 1
    for _ in range(N):
        factor = heapq.heappop(factors)
        ans = (ans * factor) % M

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()

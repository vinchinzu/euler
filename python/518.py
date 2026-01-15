"""Project Euler Problem 518: Prime triples and geometric sequences.

Find Σ(a+b+c) for all triples a < b < c < N where a+1, b+1, and c+1
form a geometric sequence.
"""

from __future__ import annotations

from math import gcd, isqrt
from typing import List


def sieve_primes(limit: int) -> List[int]:
    """Sieve of Eratosthenes."""
    if limit < 2:
        return []
    is_prime = [True] * (limit + 1)
    is_prime[0] = is_prime[1] = False
    for i in range(2, isqrt(limit) + 1):
        if is_prime[i]:
            for j in range(i * i, limit + 1, i):
                is_prime[j] = False
    return [i for i in range(limit + 1) if is_prime[i]]


def precompute_gcds(limit: int) -> List[List[int]]:
    """Precompute GCDs."""
    gcds = [[0] * (i + 1) for i in range(limit + 1)]
    for i in range(1, limit + 1):
        for j in range(1, i + 1):
            gcds[i][j] = gcd(i, j)
    return gcds


def sq(n: int) -> int:
    """Square."""
    return n * n


def solve() -> int:
    """Solve Problem 518."""
    N = 10**8

    primes_set = set(sieve_primes(N))
    gcds = precompute_gcds(isqrt(N))

    ans = 0
    for k in range(1, N + 1):
        for q in range(1, isqrt(N // k) + 1):
            c = k * sq(q) - 1
            if c < N and c in primes_set:
                for p in range(1, q):
                    if gcds[q][p] == 1:
                        a = k * sq(p) - 1
                        b = k * p * q - 1
                        if a < N and b < N and a in primes_set and b in primes_set:
                            ans += a + b + c

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()

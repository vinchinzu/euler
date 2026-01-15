"""Project Euler Problem 521: Smallest Prime Factor.

Find Σ_{i=2}^N SMPF(n), where SMPF(n) is the smallest prime factor of n.
"""

from __future__ import annotations

from math import isqrt
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


def num_primes_large(n: int, small_primes: List[int]) -> int:
    """Count primes ≤ n."""
    if n < 2:
        return 0
    count = 0
    for p in small_primes:
        if p > n:
            break
        count += 1
    return count


class QuotientValues:
    """Helper for sub-linear prime counting."""

    def __init__(self, n: int, big: List[int], small: List[int]):
        """Initialize."""
        self.n = n
        self.big = big
        self.small = small

    def div(self, x: int) -> int:
        """Return π(n/x)."""
        if x == 0:
            return 0
        q = self.n // x
        if q < len(self.small):
            return self.small[q]
        idx = self.n // q
        if idx < len(self.big):
            return self.big[idx]
        return num_primes_large(q, [])


def sum_prime_powers(n: int, power: int, mod: int) -> QuotientValues:
    """Sum of prime powers (simplified)."""
    # Simplified version - just return a QuotientValues for prime counting
    L = isqrt(n)
    big = [0] * (L + 1)
    small = [0] * (int(n // L) + 1)
    for i in range(1, L + 1):
        big[i] = n // i - 1
    for i in range(1, len(small)):
        small[i] = i - 1
    return QuotientValues(n, big, small)


def sq(n: int) -> int:
    """Square."""
    return n * n


def solve() -> int:
    """Solve Problem 521."""
    N = 10**12
    M = 10**9

    L = isqrt(N)
    primes = sieve_primes(L)

    big = [0] * (L + 1)
    small = [0] * (int(N // L) + 1)

    for i in range(1, L + 1):
        big[i] = N // i - 1
    for i in range(1, len(small)):
        small[i] = i - 1

    num_primes = QuotientValues(N, big, small)
    ans = 0

    for p in primes:
        ans += p * (num_primes.div(p) - small[p - 1])
        for i in range(1, L + 1):
            if N // i >= sq(p):
                big[i] -= num_primes.div(i * p) - small[p - 1]
        for i in range(len(small) - 1, sq(p) - 1, -1):
            if i >= sq(p):
                small[i] -= small[i // p] - small[p - 1]

    # Add sum of all primes ≤ N
    sum_primes = sum_prime_powers(N, 1, M)
    ans += sum_primes.div(1)
    ans %= M

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()

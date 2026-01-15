"""Project Euler Problem 249: Prime Subset Sums.

Let S be the set of all primes less than N. Find the number of subsets of S
that have a prime sum.
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


def build_spf(limit: int) -> List[int]:
    """Build smallest prime factor array up to limit."""
    spf = list(range(limit + 1))
    for i in range(2, isqrt(limit) + 1):
        if spf[i] == i:
            for j in range(i * i, limit + 1, i):
                if spf[j] == j:
                    spf[j] = i
    return spf


def is_prime(n: int, spf: List[int]) -> bool:
    """Check if n is prime."""
    return n > 1 and (n >= len(spf) or spf[n] == n)


def sq(n: int) -> int:
    """Return n squared."""
    return n * n


def solve() -> int:
    """Solve Problem 249."""
    N = 5000
    M = 10**16

    primes_list = sieve(N)
    max_sum = sq(N)
    dp = [0] * (max_sum + 1)
    dp[0] = 1

    sum_val = 0
    for p in primes_list:
        sum_val += p
        for i in range(sum_val, p - 1, -1):
            dp[i] = (dp[i] + dp[i - p]) % M

    spf = build_spf(sum_val)
    ans = 0
    for i in range(sum_val + 1):
        if is_prime(i, spf):
            ans = (ans + dp[i]) % M

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()

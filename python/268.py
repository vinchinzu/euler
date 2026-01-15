"""Project Euler Problem 268: Counting numbers with at least four distinct
prime factors less than 100.

Find the number of positive integers less than N which are divisible by
at least R distinct primes less than K.
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


def nCr(n: int, r: int) -> int:
    """Binomial coefficient."""
    if r < 0 or r > n:
        return 0
    if r > n // 2:
        r = n - r
    result = 1
    for i in range(r):
        result = result * (n - i) // (i + 1)
    return result


def parity(n: int) -> int:
    """Return 1 if n is even, -1 if odd."""
    return 1 if n % 2 == 0 else -1


def solve() -> int:
    """Solve Problem 268."""
    N = 10**16
    K = 100
    R = 4

    primes_list = sieve(K)
    ans = [0]

    def helper(index: int, s: int, prod: int) -> None:
        """Recursive helper."""
        if index == len(primes_list):
            if s >= R:
                weight = parity(s - R) * nCr(s - 1, R - 1)
                ans[0] += weight * (N // prod)
            return

        # Don't include this prime
        helper(index + 1, s, prod)

        # Include this prime
        if prod * primes_list[index] <= N:
            helper(index + 1, s + 1, prod * primes_list[index])

    helper(0, 0, 1)
    return ans[0]


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()

"""Project Euler Problem 512: Sums of totients of powers.

Let f(n) = Σ_{i=1}^n ϕ(n^i) (mod n+1). Find Σ_{i=1}^n f(i).
"""

from __future__ import annotations

from math import isqrt
from typing import Dict, List


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


def euler_totient(n: int) -> int:
    """Compute Euler's totient function."""
    if n <= 1:
        return 1
    result = n
    p = 2
    while p * p <= n:
        if n % p == 0:
            while n % p == 0:
                n //= p
            result -= result // p
        p += 1
    if n > 1:
        result -= result // n
    return result


def sum_phis_up_to(limit: int) -> List[int]:
    """Compute sum of ϕ(i) for i from 1 to limit."""
    result = [0] * (limit + 1)
    for i in range(1, limit + 1):
        result[i] = result[i - 1] + euler_totient(i)
    return result


class QuotientValues:
    """Helper class to compute sum of ϕ(i) for i ≤ n/x."""

    def __init__(self, n: int, sum_phis: List[int]):
        """Initialize."""
        self.n = n
        self.sum_phis = sum_phis

    def div(self, x: int) -> int:
        """Return sum of ϕ(i) for i ≤ n/x."""
        if x == 0:
            return 0
        q = self.n // x
        if q < len(self.sum_phis):
            return self.sum_phis[q]
        # For large values, compute directly (simplified)
        result = 0
        for i in range(1, q + 1):
            result += euler_totient(i)
        return result


def solve() -> int:
    """Solve Problem 512."""
    N = 5 * 10**8

    sum_phis = sum_phis_up_to(isqrt(N) + 1000)
    sum_phis_obj = QuotientValues(N, sum_phis)
    cache: Dict[int, int] = {}

    def h(k: int) -> int:
        """Recursive helper function."""
        if k in cache:
            return cache[k]
        res = sum_phis_obj.div(k)
        e = 1
        while 2 * k * e <= N:
            res -= e * h(2 * e * k)
            e *= 2
        cache[k] = res
        return res

    return h(1)


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()

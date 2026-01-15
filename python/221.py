"""Project Euler Problem 221: Alexandrian Integers.

A is an Alexandrian integer if A=p*q*r and 1/A = 1/p + 1/q + 1/r for some
integers p,q,r. Find the Nth Alexandrian integer.
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


def is_sq(n: int, p: int) -> bool:
    """Check if n is a perfect square modulo p."""
    return pow(n, (p - 1) // 2, p) <= 1


def sqrt_mod(n: int, p: int) -> int:
    """Compute sqrt(n) mod p."""
    if p % 4 == 3:
        return pow(n, (p + 1) // 4, p)
    for x in range(1, p):
        if pow(x, 2, p) == n:
            return x
    return 0


def sq(n: int) -> int:
    """Return n squared."""
    return n * n


def all_divisors(n: int, prime_factors: Set[int]) -> List[int]:
    """Return all divisors of n given prime factors."""
    divisors = [1]
    temp = n
    for p in sorted(prime_factors):
        if temp % p == 0:
            size = len(divisors)
            power = 1
            while temp % p == 0:
                temp //= p
                power *= p
                for i in range(size):
                    divisors.append(divisors[i] * power)
    if temp > 1:
        size = len(divisors)
        for i in range(size):
            divisors.append(divisors[i] * temp)
    return divisors


def solve() -> int:
    """Solve Problem 221."""
    N = 150000
    L = 80000

    spf = build_spf(L)
    primes_list = sieve(L)

    prime_factors: List[Set[int]] = [set() for _ in range(L + 1)]

    for p in primes_list:
        if is_sq(p - 1, p):
            r1 = sqrt_mod(p - 1, p)
            for r in range(r1, L + 1, p):
                prime_factors[r].add(p)
            r2 = p - r1
            for r in range(r2, L + 1, p):
                prime_factors[r].add(p)

    alexandrians: Set[int] = set()

    for r in range(1, L + 1):
        num = sq(r) + 1
        for d in all_divisors(num, prime_factors[r]):
            p = d + r
            q = num // d + r
            if p * q * r <= 2**63 - 1:  # Check for overflow
                alexandrians.add(p * q * r)

    sorted_nums = sorted(alexandrians)
    return sorted_nums[N - 1]


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()

"""Project Euler Problem 772: Balanceable Partitions.

Find the smallest integer S such that all partitions of S consisting of
integers at most N can be split into two sub-partitions with the same sum.

The answer is twice the LCM of the integers from 1 to N.
"""

from __future__ import annotations

from math import gcd, isqrt, log
from typing import List


def sieve_primes(limit: int) -> List[int]:
    """Sieve of Eratosthenes."""
    is_prime = [True] * (limit + 1)
    is_prime[0] = is_prime[1] = False
    for i in range(2, isqrt(limit) + 1):
        if is_prime[i]:
            for j in range(i * i, limit + 1, i):
                is_prime[j] = False
    return [i for i in range(2, limit + 1) if is_prime[i]]


def pow_mod(base: int, exp: int, mod: int) -> int:
    """Modular exponentiation."""
    result = 1
    base %= mod
    while exp > 0:
        if exp & 1:
            result = (result * base) % mod
        base = (base * base) % mod
        exp >>= 1
    return result


def solve() -> int:
    """Solve Problem 772."""
    N = 10**8
    M = 10**9 + 7

    primes = sieve_primes(N)
    ans = 2

    for p in primes:
        exp = int(log(N) / log(p))
        ans = (ans * pow_mod(p, exp, M)) % M

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()

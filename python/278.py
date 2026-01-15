"""Project Euler Problem 278: Linear Combinations of Semiprimes.

Find sum_{p<q<r<N} f(p*q, p*r, q*r), where f(a_k) is the largest
integer not expressible as a linear combination of the a_k with
nonnegative integer coefficients.
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
    """Solve Problem 278."""
    N = 5000

    primes_list = sieve(N)
    ans = 0

    for i in range(len(primes_list)):
        for j in range(i + 1, len(primes_list)):
            for k in range(j + 1, len(primes_list)):
                p = primes_list[i]
                q = primes_list[j]
                r = primes_list[k]
                ans += 2 * p * q * r - (p * q + p * r + q * r)

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()

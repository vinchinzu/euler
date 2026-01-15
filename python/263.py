"""Project Euler Problem 263: An engineers' dream come true.

Find the sum of the first N engineers' paradise, which is defined as an
integer n such that:
- n-9, n-3, n+3, and n+9 are consecutive primes, and
- n-8, n-4, n, n+4, and n+8 are practical (every number from 1 to n can be
  expressed as the sum of distinct divisors of n).
"""

from __future__ import annotations

from math import isqrt
from typing import List

from sympy import isprime, primerange


def is_practical(n: int) -> bool:
    """Check if n is practical."""
    if n <= 1:
        return True
    if n % 2 != 0:
        return False

    # Get divisors
    divisors: List[int] = []
    for i in range(1, isqrt(n) + 1):
        if n % i == 0:
            divisors.append(i)
            if i != n // i:
                divisors.append(n // i)
    divisors.sort()

    sum_so_far = 0
    for d in divisors:
        if d > sum_so_far + 1:
            return False
        sum_so_far += d
    return True


def solve() -> int:
    """Solve Problem 263."""
    N = 4
    L = 2 * (10**9)

    paradises: List[int] = []

    # Sieve for candidates
    # Simplified - full version would use proper sieving
    primes = list(primerange(2, isqrt(L) + 100))

    for sign in [-1, 1]:
        for i in range(20, L // 840):
            n = 840 * i - sign * 20
            if n < 0 or n > L:
                continue

            # Check primes
            if not all(
                isprime(n + d * sign) for d in [-9, -3, 3, 9]
            ):
                continue
            if isprime(n + sign) or isprime(n + 7 * sign):
                continue

            # Check practical numbers
            if all(is_practical(n + d) for d in [-8, -4, 0, 4, 8]):
                paradises.append(n)

    paradises.sort()
    return sum(paradises[:N])


def main() -> None:
    """Main entry point."""
    result = solve()
    print(result)


if __name__ == "__main__":
    main()

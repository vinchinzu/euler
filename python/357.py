"""Project Euler Problem 357: Prime Generating Integers

Find the sum of all positive integers n ≤ 100,000,000 such that for every
divisor d of n, d + n/d is prime.
"""

from __future__ import annotations

import numpy as np


def sieve_of_eratosthenes(limit: int) -> np.ndarray:
    """Return a boolean array where is_prime[i] is True if i is prime."""
    is_prime = np.ones(limit + 1, dtype=np.bool_)
    is_prime[0] = is_prime[1] = False

    for i in range(2, int(limit**0.5) + 1):
        if is_prime[i]:
            is_prime[i*i::i] = False

    return is_prime


def check_divisors(n: int, is_prime: np.ndarray) -> bool:
    """Check if d + n/d is prime for all divisors d of n."""
    # Check d = 1: 1 + n is already ensured to be prime by how we select n
    # Check divisors d where 1 < d <= sqrt(n)
    d = 2
    while d * d <= n:
        if n % d == 0:
            # d is a divisor, check if d + n/d is prime
            quotient = n // d
            if not is_prime[d + quotient]:
                return False
        d += 1
    return True


def solve(limit: int = 100_000_000) -> int:
    """Solve PE 357 for n up to the given limit.

    Key insights:
    1. Since 1 is always a divisor, 1 + n = n+1 must be prime
    2. So we only check n where n+1 is prime
    3. n must be 1 or even (since n+1 is prime > 2, n+1 is odd, so n is even)
       Exception: n=1 where n+1=2 is prime
    4. For each candidate n, check all divisor pairs (d, n/d)
    """
    # Generate prime sieve up to limit + 1 (since we need to check n+1)
    is_prime = sieve_of_eratosthenes(limit + 1)

    total = 0

    # n = 1 is a special case: only divisor is 1, and 1 + 1 = 2 is prime
    # Check: 1 + 1 = 2, which is prime. So n=1 works.
    if is_prime[2]:  # 1 + 1 = 2 is prime
        total += 1

    # For n > 1, n must be even (since n+1 must be an odd prime)
    # So we iterate through odd primes p and check n = p - 1 (which is even)
    for p in range(3, limit + 2, 2):  # odd numbers only
        if not is_prime[p]:
            continue

        n = p - 1  # n is even, and n + 1 = p is prime

        if n > limit:
            break

        # Quick check: d=2 must work (since n is even)
        # 2 + n/2 must be prime
        if not is_prime[2 + n // 2]:
            continue

        # Check all other divisors
        if check_divisors(n, is_prime):
            total += n

    return total


if __name__ == "__main__":
    print(solve())

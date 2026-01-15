"""Project Euler Problem 495: Writing n! as product of k distinct integers.

Find the number of ways that N! can be written as the product of K distinct
positive integers.
"""

from __future__ import annotations

from collections import Counter
from itertools import combinations_with_replacement
from math import isqrt
from typing import Dict, List, Tuple


def sieve_primes(limit: int) -> List[int]:
    """Sieve of Eratosthenes."""
    is_prime = [True] * (limit + 1)
    is_prime[0] = is_prime[1] = False
    for i in range(2, isqrt(limit) + 1):
        if is_prime[i]:
            for j in range(i * i, limit + 1, i):
                is_prime[j] = False
    return [i for i in range(limit + 1) if is_prime[i]]


def num_factors_in_factorial(n: int, p: int) -> int:
    """Count factors of p in n!."""
    count = 0
    power = p
    while power <= n:
        count += n // power
        power *= p
    return count


def nCr(n: int, k: int, mod: int) -> int:
    """Binomial coefficient modulo mod."""
    if k < 0 or k > n:
        return 0
    result = 1
    for i in range(min(k, n - k)):
        result = (result * (n - i)) % mod
        result = (result * pow(i + 1, mod - 2, mod)) % mod
    return result


def solve() -> int:
    """Solve Problem 495."""
    N = 10_000
    K = 30
    M = 10**9 + 7

    primes = sieve_primes(N)
    exponents = [num_factors_in_factorial(N, p) for p in primes]

    # Inclusion-exclusion over partitions of K
    ans = 0
    # Simplified: use stars and bars for each prime
    for partition in combinations_with_replacement(range(1, K + 1), K):
        # Count distinct partitions
        counts = Counter(partition)
        ways = 1
        for exp in exponents:
            # Ways to distribute exp factors among K groups
            # Simplified calculation
            ways = (ways * nCr(exp + K - 1, K - 1, M)) % M

        # Inclusion-exclusion coefficient
        coeff = 1
        for count_val, mult in counts.items():
            if count_val > 1:
                coeff = coeff * pow(-1, count_val - 1, M) % M
                coeff = (coeff * pow(count_val, mult, M)) % M

        ans = (ans + ways * coeff) % M

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()

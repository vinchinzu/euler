"""Project Euler Problem 586: Numbers expressible as a²+3ab+b².

Find the number of integers not exceeding N that can be expressed as
a²+3ab+b² for integers a>b>0 in exactly K different ways.

The number of ways to express an integer k without the restriction that
a>b is 0 if k contains an odd number of factors of a prime that is 2 or
3 (mod 5); otherwise, it is g(k) = prod_i (1 + e_i) for all exponents
e_i of prime factors that are 1 or 4 (mod 5). If we add the restriction
that a>b, then the number of ways is ⌊g(k)/2⌋.
"""

from __future__ import annotations

from math import isqrt
from typing import List

from sympy import primerange


def sieve_with_smallest_factor(limit: int) -> List[int]:
    """Sieve that stores smallest prime factor."""
    ff = [0] * (limit + 1)
    for i in range(2, limit + 1):
        if ff[i] == 0:
            ff[i] = i
            for j in range(i * i, limit + 1, i):
                if ff[j] == 0:
                    ff[j] = i
    return ff


def prime_factorization(n: int, ff: List[int]) -> dict[int, int]:
    """Get prime factorization using smallest factor array."""
    factors: dict[int, int] = {}
    temp = n
    while temp > 1:
        p = ff[temp]
        factors[p] = factors.get(p, 0) + 1
        temp //= p
    return factors


def solve() -> int:
    """Solve Problem 586."""
    N = 10**15
    K = 40

    # Precompute minPower array
    limit = 2 * K + 2
    ff = sieve_with_smallest_factor(limit)
    min_power = [0] * limit

    for k in range(limit):
        factors = prime_factorization(k, ff)
        min_power[k] = sum(e * (p - 1) for p, e in factors.items())
    min_power[1] = 2

    # Get primes up to reasonable limit
    max_prime_exp = min(min_power[2 * K], min_power[2 * K + 1]) - 1
    max_prime = int(N / (11**max_prime_exp))
    primes = list(primerange(2, max_prime + 1))

    ans = 0

    def helper(k: int, last_index: int, prod: int) -> None:
        """Recursive helper to find numbers with exactly k representations."""
        nonlocal ans
        if k == 1:
            ans += 1
            return

        for index in range(last_index + 1, len(primes)):
            p = primes[index]
            if prod * (p ** min_power[k]) > N:
                break

            new_prod = prod
            e = 1
            while new_prod * p <= N:
                new_prod *= p
                if p % 5 == 1 or p % 5 == 4:
                    if k % (e + 1) == 0:
                        helper(k // (e + 1), index, new_prod)
                elif p == 5 or e % 2 == 0:
                    helper(k, index, new_prod)
                e += 1

    helper(2 * K, -1, 1)
    helper(2 * K + 1, -1, 1)

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()

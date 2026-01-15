"""Project Euler Problem 642: Sum of largest prime factors.

Find sum_{i=2}^N f(i), where f(i) is the largest prime factor of i.

We recurse over the prime factorizations of all numbers up to N in increasing
order, except for the last (largest) prime. For each prime factorization but
missing the largest prime, we compute the sum of all possible values of the
largest prime; this must be at least the second largest prime, but the product
must not exceed N.
"""

from __future__ import annotations

from math import isqrt

from sympy import primerange


class QuotientValues:
    """Helper for quotient-based prime sum."""

    def __init__(self, n: int, big: list[int], small: list[int]):
        """Initialize."""
        self.n = n
        self.big = big
        self.small = small
        self.L = len(small) - 1

    def div(self, x: int) -> int:
        """Get sum for quotient n/x."""
        if x <= self.L:
            return self.small[x]
        idx = self.n // x
        return self.big[idx] if idx < len(self.big) else 0

    def get(self, x: int) -> int:
        """Get sum for exact x."""
        if x <= self.L:
            return self.small[x]
        return 0


def sum_prime_powers(n: int, power: int, mod: int) -> QuotientValues:
    """Sum of prime powers (simplified)."""
    L = isqrt(n)
    big = [0] * (L + 1)
    small = [0] * (L + 1)

    primes = list(primerange(2, L + 1))
    cumulative = 0
    for i in range(1, L + 1):
        if i in primes:
            cumulative = (cumulative + pow(i, power)) % mod
        small[i] = cumulative

    return QuotientValues(n, big, small)


def solve() -> int:
    """Solve Problem 642."""
    N = 201820182018
    M = 10**9
    L = isqrt(N)

    primes = list(primerange(2, L + 1))
    sum_primes = sum_prime_powers(N, 1, M)
    ans = 0

    def helper(min_index: int, n: int) -> None:
        """Recursive helper."""
        nonlocal ans
        ans = (ans + sum_primes.div(n) - sum_primes.get(primes[min_index] - 1)) % M

        for index in range(min_index, len(primes)):
            p = primes[index]
            if n * p * p > N:
                break
            helper(index, n * p)

    helper(0, 1)
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()

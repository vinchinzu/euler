"""Project Euler Problem 241: Perfection Quotients.

Find the sum of all positive integers n ≤ N such that the perfection quotient
is of the form (k + 1/2) for integral k.
"""

from __future__ import annotations

from fractions import Fraction
from math import isqrt
from typing import List


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


def is_probable_prime(n: int) -> bool:
    """Check if n is probably prime (simplified version)."""
    if n < 2:
        return False
    if n == 2:
        return True
    if n % 2 == 0:
        return False
    for i in range(3, isqrt(n) + 1, 2):
        if n % i == 0:
            return False
    return True


def all_divisors(n: int, prime_factors: List[int]) -> List[int]:
    """Return all divisors of n given prime factors."""
    divisors = [1]
    temp = n
    for p in prime_factors:
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
    """Solve Problem 241."""
    N = 10**18
    L = 1000000

    spf = build_spf(L)
    primes_list = sieve(L)

    # Compute max perfection quotient
    prod = 1
    max_perfection_quotient = 1.0
    for p in primes_list:
        prod *= p
        if prod > N:
            break
        max_perfection_quotient *= p / (p - 1)

    ans = [0]

    def helper(prod_val: int, r: Fraction) -> None:
        """Recursive helper."""
        if r == Fraction(1, 1):
            ans[0] += prod_val
        if r.denominator > L:
            return

        p = spf[r.denominator] if r.denominator < len(spf) else r.denominator
        if p > 1 and prod_val % p != 0:
            pe = 1
            mult = 1
            while prod_val * pe * p <= N:
                pe *= p
                mult += pe
                helper(
                    prod_val * pe,
                    r * Fraction(pe, mult),
                )

    for k in range(1, int(max_perfection_quotient)):
        target = Fraction(k) + Fraction(1, 2)
        helper(1, target)

    return ans[0]


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()

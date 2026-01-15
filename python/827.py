"""Project Euler Problem 827: Pythagorean Triples.

Find Σ_{k=1}^N Q(10^k), where Q(n) is the smallest integer that is part
of exactly n Pythagorean triples a<b<c.

The number of ways that k can be the largest value of a Pythagorean triple
is just the number of ways that k² can be written as the sum of two
squares, which by the sum of squares function is just
⌊(Π_i(2e_i + 1)) / 2⌋, where e_i is the exponent of all primes 1 (mod 4)
in the prime factorization of k.

The number of ways that k can be one of the small values of a Pythagorean
triple is the number of ways that k² can be expressed as the product of
(c-b)(c+b), which is almost the number of factors of k², except the two
factors must be of the same parity. In our case, k is even in order to
keep the numbers small, so we need both factors to contain a factor of 2;
that means we only count the number of factors of k²/4, divided by 2.

The total number of Pythagorean triples is thus the product of the
exponents of all 1 (mod 4) primes, times the product of the exponents of
all primes, both divided by 2. In order words, the product of the
exponents of 1 (mod 4) primes, times one plus the product of the
exponents of other primes, needs to be 2n+2 (where the +2 is because the
two odd numbers both round down when divided by 2). So we compute all
factorizations of 2n+2, and assign them to the 1 (mod 4) primes and not
1 (mod 4) primes, in order to minimize the product. We find the
factorization that gives the smallest result.
"""

from __future__ import annotations

import math
from dataclasses import dataclass
from math import isqrt
from typing import Dict, List, Set


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


def primes_mod(limit: int, mod: int, remainder: int) -> List[int]:
    """Primes congruent to remainder mod mod."""
    primes = sieve_primes(limit)
    return [p for p in primes if p % mod == remainder]


def prime_factorize(n: int) -> Dict[int, int]:
    """Factorize n into prime factors."""
    factors: Dict[int, int] = {}
    d = 2
    while d * d <= n:
        while n % d == 0:
            factors[d] = factors.get(d, 0) + 1
            n //= d
        d += 1
    if n > 1:
        factors[n] = factors.get(n, 0) + 1
    return factors


def get_all_divisors(n: int) -> List[int]:
    """Get all divisors of n."""
    factors = prime_factorize(n)
    divisors = [1]
    for p, e in factors.items():
        new_divs = []
        for d in divisors:
            for i in range(e + 1):
                new_divs.append(d * (p**i))
        divisors = new_divs
    return sorted(divisors)


def pow_mod(base: int, exp: int, mod: int) -> int:
    """Modular exponentiation."""
    result = 1
    base = base % mod
    while exp > 0:
        if exp & 1:
            result = (result * base) % mod
        base = (base * base) % mod
        exp >>= 1
    return result


@dataclass
class Number:
    """Number with log and mod value."""

    log: float
    mod: int

    @staticmethod
    def of(value: int, M: int) -> "Number":
        """Create Number from value."""
        return Number(math.log(value), value % M)

    def multiply(self, other: "Number", M: int) -> "Number":
        """Multiply two Numbers."""
        return Number(self.log + other.log, (self.mod * other.mod) % M)

    def pow(self, e: int, M: int) -> "Number":
        """Raise to power."""
        return Number(self.log * e, pow_mod(self.mod, e, M))


def solve() -> int:
    """Solve Problem 827."""
    N = 18
    M = 409120391
    L = 100

    primes_1_mod_4 = primes_mod(L, 4, 1)
    primes_3_mod_4 = [2] + primes_mod(L, 4, 3)

    ans = 0
    for k in range(1, N + 1):
        n = 2 * (10**k) + 2
        divisors = get_all_divisors(n)
        min_result = Number(float("inf"), 0)

        for d in divisors:
            if d % 2 == 1:
                factors_d = prime_factorize(d)
                factors_other = prime_factorize(n // d - 1)

                # Build number from factors
                result = Number.of(2, M)
                primes_1 = primes_1_mod_4[:]
                primes_other = primes_3_mod_4[:]

                # Assign factors from d to primes_1_mod_4
                for p, e in sorted(factors_d.items(), reverse=True):
                    if primes_1:
                        prime = primes_1.pop(0)
                        result = result.multiply(Number.of(prime, M).pow(e // 2, M), M)

                # Assign factors from n/d-1 to other primes
                for p, e in sorted(factors_other.items(), reverse=True):
                    if primes_other:
                        prime = primes_other.pop(0)
                        result = result.multiply(Number.of(prime, M).pow(e // 2, M), M)

                if result.log < min_result.log:
                    min_result = result

        ans = (ans + min_result.mod) % M

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()

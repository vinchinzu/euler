"""Project Euler Problem 699: Triffle Numbers.

Find the sum of all integers n up to N such that the denominator of σ(n)/n
written in lowest terms is a power of 3.

We just brute force by starting with 3-smooth numbers n, then repeatedly
multiplying by primes that partially cancel the current numerator.
"""

from __future__ import annotations

from collections import defaultdict
from fractions import Fraction
from functools import lru_cache


def pow_mod(base: int, exp: int) -> int:
    """Power without mod."""
    result = 1
    while exp > 0:
        if exp & 1:
            result *= base
        base *= base
        exp >>= 1
    return result


@lru_cache(maxsize=None)
def sum_divisors(p: int, e: int) -> int:
    """Sum of divisors p^0 + p^1 + ... + p^e."""
    result = 0
    power = 1
    for i in range(e + 1):
        result += power
        power *= p
    return result


@lru_cache(maxsize=None)
def prime_factors(n: int) -> dict[int, int]:
    """Get prime factors of n."""
    factors: dict[int, int] = {}
    d = 2
    while d * d <= n:
        while n % d == 0:
            factors[d] = factors.get(d, 0) + 1
            n //= d
        d += 1
    if n > 1:
        factors[n] = factors.get(n, 0) + 1
    return factors


def is_power_of_3(n: int) -> bool:
    """Check if n is a power of 3."""
    if n <= 0:
        return False
    while n % 3 == 0:
        n //= 3
    return n == 1


def solve() -> int:
    """Solve Problem 699."""
    N = 10**14
    goods: set[int] = set()

    def helper(n: int, r: Fraction) -> None:
        """Helper function."""
        if r.denominator > 1 and is_power_of_3(r.denominator):
            goods.add(n)

        # Get prime factors of numerator
        num_factors = prime_factors(r.numerator)
        for p in num_factors:
            if n % p != 0:
                e = 1
                while n * pow(p, e) <= N:
                    new_n = n * pow(p, e)
                    new_r = r * Fraction(sum_divisors(p, e), pow(p, e))
                    helper(new_n, new_r)
                    e += 1

    # Start with 3-smooth numbers
    e2 = 0
    while pow_mod(2, e2) <= N:
        e3 = 1
        while True:
            n = pow_mod(2, e2) * pow_mod(3, e3)
            if n > N:
                break
            r = Fraction(
                sum_divisors(2, e2) * sum_divisors(3, e3), n
            )
            helper(n, r)
            e3 += 1
        e2 += 1

    ans = sum(goods)
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()

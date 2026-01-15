"""Project Euler Problem 646: Bounded Divisors.

Define the Liouville function λ(n) to be 1 if the number of prime factors of
n (with multiplicity) is even, and -1 otherwise. Find the sum of λ(d) * d
over all divisors d | N! where L ≤ d ≤ H.

We split the prime factors of N! into two groups such that the number of
factors using only primes of the left and right groups are roughly equal.
"""

from __future__ import annotations

from collections import defaultdict
from dataclasses import dataclass
from itertools import product

from sympy import primerange


@dataclass
class PrimeFactor:
    """Prime factor with exponent."""

    p: int
    e: int


def num_factors_in_factorial(n: int, p: int) -> int:
    """Count how many times p divides n!."""
    count = 0
    power = p
    while power <= n:
        count += n // power
        power *= p
    return count


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


def get_divisors(factors: list[PrimeFactor], M: int) -> dict[float, int]:
    """Get all divisors with Liouville values."""
    divisors = {}
    axes = [list(range(factor.e + 1)) for factor in factors]

    for exponents in product(*axes):
        val = 1.0
        liouville = 1
        for i, factor in enumerate(factors):
            p = factor.p
            e = exponents[i]
            val *= pow(p, e)
            liouville = (liouville * pow_mod(-p, e, M)) % M
        divisors[val] = liouville

    return divisors


def solve() -> int:
    """Solve Problem 646."""
    N = 70
    L = 10**20
    H = 10**60
    M = 10**9 + 7

    factors = []
    for p in primerange(2, N + 1):
        e = num_factors_in_factorial(N, p)
        factors.append(PrimeFactor(p, e))

    # Split factors
    total_factors = 1
    for factor in factors:
        total_factors *= factor.e + 1

    half_index = 0
    num_factors = 1
    while num_factors * num_factors < total_factors:
        num_factors *= factors[half_index].e + 1
        half_index += 1

    left_divisors = get_divisors(factors[:half_index], M)
    right_divisors = get_divisors(factors[half_index:], M)

    # Sort left divisors
    sorted_left = sorted(left_divisors.items())
    cumul_liouvilles = {}
    cumul = 0
    cumul_liouvilles[0.0] = 0
    for val, liouville in sorted_left:
        cumul = (cumul + liouville) % M
        cumul_liouvilles[val] = cumul

    ans = 0
    for val_r, liouville_r in right_divisors.items():
        max_left = H / val_r
        min_left = L / val_r

        # Find cumulative sums
        max_sum = 0
        min_sum = 0
        for val_l, cumul in sorted(cumul_liouvilles.items()):
            if val_l <= max_left:
                max_sum = cumul
            if val_l < min_left:
                min_sum = cumul

        ans = (ans + liouville_r * (max_sum - min_sum)) % M

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()

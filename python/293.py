"""Project Euler Problem 293: Pseudo-Fortunate Numbers.

An integer is admissible if its distinct prime factors are consecutive primes
starting from 2, and the pseudo-Fortunate number for an admissible integer n is
the smallest m such that m+n is prime. Find the sum of all distinct
pseudo-Fortunate numbers for admissible integers under N.

We iterate over all admissible integers with prime factors up to p for
increasing primes p, and directly compute the pseudo-Fortunate number for each
one.
"""

from __future__ import annotations

from math import log2
from typing import List, Set

from sympy import isprime, primerange


def ipow(base: int, exp: int) -> int:
    """Integer power."""
    return base**exp


def ilog2(n: int) -> int:
    """Integer logarithm base 2."""
    return int(log2(n))


def solve() -> int:
    """Solve Problem 293."""
    N = ipow(10, 9)

    admissibles: List[int] = [1]
    fortunates: Set[int] = set()

    primes = list(primerange(2, ilog2(N) + 1))

    for p in primes:
        new_admissibles: List[int] = []
        for admissible in admissibles:
            new_admissible = admissible * p
            while new_admissible < N:
                new_admissibles.append(new_admissible)
                # Find pseudo-Fortunate number
                m = 2
                while not isprime(new_admissible + m):
                    m += 1
                fortunates.add(m)
                new_admissible *= p
        admissibles = new_admissibles

    return sum(fortunates)


def main() -> None:
    """Main entry point."""
    result = solve()
    print(result)


if __name__ == "__main__":
    main()

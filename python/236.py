"""Project Euler Problem 236: Luxury Hampers.

Two suppliers supply products. Find the largest value of m such that it is
possible for the ratio of bad products in B_k to bad products in A_k to equal
m for all values of k.
"""

from __future__ import annotations

from fractions import Fraction
from math import gcd
from typing import Tuple


def lin_comb(a: int, b: int, c: int) -> Tuple[int, int]:
    """Find (x, y) such that ax + by = c."""
    if b == 0:
        return (c // a if a != 0 else 0, 0)

    g, x1, y1 = ext_gcd(a, b)
    if c % g != 0:
        return (0, 0)

    x = x1 * (c // g)
    y = y1 * (c // g)
    return (x, y)


def ext_gcd(a: int, b: int) -> Tuple[int, int, int]:
    """Extended Euclidean algorithm."""
    if b == 0:
        return (a, 1, 0)
    g, x1, y1 = ext_gcd(b, a % b)
    return (g, y1, x1 - (a // b) * y1)


def solve() -> str:
    """Solve Problem 236."""
    import math

    A1, A2, A3 = 5248, 1312 + 2624 + 3936, 5760
    B1, B2, B3 = 640, 1888 + 3776 + 5664, 3776

    best = Fraction(1, 1)

    for a1 in range(1, A1):
        for b1 in range(1, B1):
            u = b1 * A1
            v = a1 * B1

            if u * best.denominator < v * best.numerator:
                continue

            r2 = Fraction(A2 * v, B2 * u)
            r3 = Fraction(A3 * v, B3 * u)

            if (
                r2.numerator > A2
                or r2.denominator > B2
                or r3.numerator > A3
                or r3.denominator > B3
            ):
                continue

            s = Fraction((A1 + A2 + A3) * u, (B1 + B2 + B3) * v)
            a = s.denominator * r2.numerator - s.numerator * r2.denominator
            b = s.denominator * r3.numerator - s.numerator * r3.denominator
            c = s.numerator * b1 - s.denominator * a1

            if c % gcd(a, b) != 0:
                continue

            p = lin_comb(a, b, c)
            if p[0] == 0 and p[1] == 0:
                continue

            min_t1 = -p[0] / abs(b) if b != 0 else 0
            max_t1 = (
                (min(A2 / r2.numerator, B2 / r2.denominator) - p[0]) / abs(b)
                if b != 0
                else 0
            )
            min_t2 = (
                (p[1] - min(A3 / r3.numerator, B3 / r3.denominator)) / abs(a)
                if a != 0
                else 0
            )
            max_t2 = p[1] / abs(a) if a != 0 else 0

            if (
                math.ceil(max(min_t1, min_t2))
                <= math.floor(min(max_t1, max_t2))
            ):
                best = Fraction(u, v)

    return f"{best.numerator}/{best.denominator}"


def main() -> str:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()

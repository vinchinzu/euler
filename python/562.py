"""Project Euler Problem 562: Maximal Triangle Perimeter.

Find the triangle with lattice point vertices within or on a circle of radius N, no other
lattice points on or in its boundary, and with maximum perimeter.

By Pick's formula, the area of the triangle is 1/2.

Suppose the longest side of such a triangle is a. Then the height is 1/a, and the maximum
possible perimeter is if the triangle is isosceles with legs of length √((a/2)²+(1/a)²).

We can now try all line segments (x1, y1), (x2, y2) inside the circle with length close to
2N. To find (x3, y3), we use the Shoestring Formula and solve the resulting
linear Diophantine equation.
"""

from __future__ import annotations

from dataclasses import dataclass
from math import gcd, hypot, isqrt, sqrt


N = 10**7
L = 20


@dataclass(frozen=True)
class LPoint:
    """Lattice point."""

    x: int
    y: int


def sq(n: int) -> int:
    """Square of n."""
    return n * n


def lin_comb(a: int, b: int) -> LPoint:
    """Find solution to ax + by = gcd(a,b) using extended Euclidean algorithm.

    Returns (x, y) such that ax + by = gcd(a,b).
    """
    if b == 0:
        return LPoint(1, 0)
    x1, y1 = lin_comb(b, a % b)
    q = a // b
    return LPoint(y1, x1 - q * y1)


def max_possible_perim(a: float) -> float:
    """Maximum possible perimeter for triangle with longest side a."""
    return a + 2 * hypot(a / 2, 1 / a)


def solve() -> int:
    """Solve Problem 562."""
    max_perim = 0.0
    ans = 0
    for x1 in range(isqrt(sq(N) // 2) + 1):
        y1 = isqrt(sq(N) - sq(x1))
        if max_possible_perim(N + hypot(x1, y1)) < max_perim:
            continue
        for x2 in range(-x1 - L, min(-x1 + L + 1, x1)):
            for y2 in range(-y1 - L, min(-y1 + L + 1, y1)):
                if hypot(x2, y2) <= N:
                    a = hypot(x1 - x2, y1 - y2)
                    if max_possible_perim(a) < max_perim:
                        continue
                    if gcd(x1 - x2, y1 - y2) != 1:
                        continue
                    diff = lin_comb(y1 - y2, x2 - x1)
                    x3 = x2 + diff.x
                    y3 = y2 + diff.y
                    if hypot(x3, y3) <= N:
                        b = hypot(x1 - x3, y1 - y3)
                        c = hypot(x2 - x3, y2 - y3)
                        perim = a + b + c
                        if perim > max_perim:
                            max_perim = perim
                            ans = round(a * b * c / 2 / N)
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()

"""Project Euler Problem 273: Sum of Squares.

Find the sum of all a where a < b and a² + b² = T, summed over all square-free
integers T with only prime factors of the form 4k+1 < N.

A prime p of the form 4k+1 can be expressed as p = (x + yi)(x - yi) for
integers x, y. An integer T can be expressed as p_1 p_2 ... p_r = (x_1 + y_1
i)(x_1 - y_1 i)(x_2 + y_2 i)(x_2 - y_2 i) ... (x_r + y_r i)(x_r - y_r i), and
any a < b satisfying a² + b² = T can be derived from (x ± yi) = (x_1 ± y_1
i)(x_2 ± y_2 i) ... (x_r ± y_r i) for some set of signs. This means that we
can compute the answer by iterating over all products of (x_r ± y_r i), and
summing the smaller of x, y. To avoid processing (x + yi) and (x - yi), we only
process y > 0.
"""

from __future__ import annotations

from dataclasses import dataclass
from math import isqrt
from typing import List

from sympy import primerange


@dataclass(frozen=True)
class Point:
    """A point representing a complex number (x, y) = x + yi."""

    x: int
    y: int

    def complex_multiply(self, other: "Point") -> "Point":
        """Complex multiplication: (x1 + y1i) * (x2 + y2i)."""
        return Point(
            self.x * other.x - self.y * other.y,
            self.x * other.y + self.y * other.x,
        )

    def reflect_y(self) -> "Point":
        """Reflect across x-axis: (x, y) -> (x, -y)."""
        return Point(self.x, -self.y)


def is_square(n: int) -> bool:
    """Check if n is a perfect square."""
    if n < 0:
        return False
    root = isqrt(n)
    return root * root == n


def primes_mod(n: int, a: int, m: int) -> List[int]:
    """Return primes p < n such that p ≡ a (mod m)."""
    return [p for p in primerange(2, n) if p % m == a]


def find_a(n: int) -> int:
    """Find a such that n - a² is a perfect square."""
    a = 1
    while True:
        if is_square(n - a * a):
            return a
        a += 1


def solve() -> int:
    """Solve Problem 273."""
    N = 150

    ps = primes_mod(N, 1, 4)

    bases: List[Point] = []
    for p in ps:
        a = find_a(p)
        b = isqrt(p - a * a)
        bases.append(Point(a, b))

    ans = 0

    def helper(index: int, current: Point, bases_list: List[Point]) -> None:
        """Recursive helper to compute all products."""
        nonlocal ans
        if index == len(bases_list):
            if current.y > 0:
                ans += min(abs(current.x), abs(current.y))
            return

        # Don't multiply
        helper(index + 1, current, bases_list)
        # Multiply by base
        helper(index + 1, current.complex_multiply(bases_list[index]), bases_list)
        # Multiply by reflected base
        helper(
            index + 1,
            current.complex_multiply(bases_list[index].reflect_y()),
            bases_list,
        )

    helper(0, Point(1, 0), bases)
    return ans


def main() -> None:
    """Main entry point."""
    result = solve()
    print(result)


if __name__ == "__main__":
    main()

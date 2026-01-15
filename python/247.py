"""Project Euler Problem 247: Squares under a hyperbola.

Take the region constrained by 1 ≤ x and 0 ≤ y ≤ 1/x, and repeatedly add the
largest square that does not overlap with any of the previous squares. Find
the largest n such that S_n has IX squares to its left and IY squares below it.
"""

from __future__ import annotations

import heapq
from dataclasses import dataclass


@dataclass
class Square:
    """Represents a square."""

    ix: int
    iy: int
    x0: float
    y0: float
    s: float

    def __init__(self, ix: int, iy: int, x0: float, y0: float):
        self.ix = ix
        self.iy = iy
        self.x0 = x0
        self.y0 = y0
        # Compute side length
        self.s = (
            math.sqrt(x0 * x0 + y0 * y0 - 2 * x0 * y0 + 4) - (x0 + y0)
        ) / 2

    def __lt__(self, other):
        """For max heap (larger s comes first)."""
        return self.s > other.s


def n_cr(n: int, k: int) -> int:
    """Compute binomial coefficient."""
    if k < 0 or k > n:
        return 0
    if k == 0 or k == n:
        return 1
    result = 1
    for i in range(min(k, n - k)):
        result = result * (n - i) // (i + 1)
    return result


def solve() -> int:
    """Solve Problem 247."""
    import math

    IX = 3
    IY = 3

    num_at_index = n_cr(IX + IY, IX)
    squares: list[Square] = []
    heapq.heappush(squares, Square(0, 0, 1.0, 0.0))
    ans = 0

    while num_at_index > 0:
        square = heapq.heappop(squares)
        if square.ix == IX and square.iy == IY:
            num_at_index -= 1
        heapq.heappush(
            squares, Square(square.ix + 1, square.iy, square.x0 + square.s, square.y0)
        )
        heapq.heappush(
            squares, Square(square.ix, square.iy + 1, square.x0, square.y0 + square.s)
        )
        ans += 1

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()

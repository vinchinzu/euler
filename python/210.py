"""Project Euler Problem 210: Obtuse Angled Triangles.

Find the number of points (x, y) satisfying |x|+|y|≤N such that (0, 0),
(N/4, N/4), and (x, y) form an obtuse triangle.
"""

from __future__ import annotations

import math


def sq(n: int) -> int:
    """Return n squared."""
    return n * n


def fsq(n: float) -> float:
    """Return n squared."""
    return n * n


def solve() -> int:
    """Solve Problem 210."""
    N = 10**9

    ans = 3 * sq(N) // 2 + sq(N // 4 + 1) - (N // 4 + 1) - 2

    sqrt2 = math.sqrt(2)
    min_x = int(-(N / 8 * (sqrt2 - 1)))

    for x in range(min_x, 0):
        dy = math.sqrt(2 * sq(N // 8) - fsq(N / 8 - x))
        ans += 4 * (2 * int(math.ceil(dy)) - 1)

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()

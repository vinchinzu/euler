"""Project Euler Problem 285: Pythagorean odds.

For a given round k of a game, random numbers a, b ∈ [0, 1] are selected,
and k points are scored if √((k*a + 1)² + (k*b + 1)²) rounded to the
nearest integer is k, and 0 points are scored otherwise. Find the
expected score of a game with N rounds.
"""

from __future__ import annotations

import math


def solve() -> float:
    """Solve Problem 285."""
    N = 100000
    ans = 0.0

    for k in range(1, N + 1):
        area_val = area(k + 0.5)
        if k > 1:
            area_val -= area(k - 0.5)
        ans += k * area_val / (k * k)

    return ans


def area(r: float) -> float:
    """Area of intersection of square and circle."""
    return (
        (math.pi / 4 - math.asin(1.0 / r)) * r * r
        - (math.sqrt(r * r - 1) - 1)
    )


def main() -> float:
    """Main entry point."""
    result = solve()
    print(f"{result:.5f}")
    return result


if __name__ == "__main__":
    main()

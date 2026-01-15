"""Project Euler Problem 462: Permutation of 3-smooth numbers.

Find the number of ways that the 3-smooth numbers up to N can be arranged
such that each element comes after all its proper divisors.
"""

from __future__ import annotations

import math
from typing import List


class Point:
    """2D point."""

    def __init__(self, x: int, y: int) -> None:
        """Initialize point."""
        self.x = x
        self.y = y


def solve() -> str:
    """Solve Problem 462."""
    N = 10**18
    points: List[Point] = []

    # Generate 3-smooth numbers as points
    pow2 = 1
    for e2 in range(100):  # Reasonable limit
        if pow2 > N:
            break
        pow3 = 1
        for e3 in range(100):  # Reasonable limit
            if pow2 * pow3 > N:
                break
            points.append(Point(e2, e3))
            pow3 *= 3
        pow2 *= 2

    ans = 0.0
    # Sum of logarithms
    for i in range(1, len(points) + 1):
        ans += math.log(i)

    # Subtract hook lengths
    for p in points:
        hook = 0
        for q in points:
            if (q.x == p.x and q.y >= p.y) or (q.y == p.y and q.x >= p.x):
                hook += 1
        ans -= math.log(hook)

    ans /= math.log(10)
    exp_part = int(ans)
    mantissa = 10 ** (ans - exp_part)
    return f"{mantissa:.10f}e{exp_part}"


def main() -> str:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()

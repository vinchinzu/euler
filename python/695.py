"""Project Euler Problem 695: Random Rectangles.

Choose three random points in the unit square and consider the three
rectangles with sides parallel to the unit square with each of the pairs of
points as diagonals. Find the expected median area of the three rectangles.

The solution involves integrating over satisfactory points P3 and computing
the expected value analytically.
"""

from __future__ import annotations

import math


def solve() -> float:
    """Solve Problem 695."""
    ans = (
        24 * math.log((3 + math.sqrt(5)) / 4)
        + 22 * math.sqrt(5)
        - 41
    ) / 144
    return ans


def main() -> float:
    """Main entry point."""
    result = solve()
    print(f"{result:.10f}")
    return result


if __name__ == "__main__":
    main()

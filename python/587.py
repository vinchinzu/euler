"""Project Euler Problem 587: L-section area.

A unit circle is placed tangent to the positive x and y axis; call the
region between the circle and the corner the L-section. Find the minimum
n such that a ray through the corner with slope 1/n divides the L-section
into a region which is less than R of the total area of the L-section.

Let O be the origin, A = (1, 0), B = (ny, y) be the first intersection
of the ray with the circle, and C = (1, 1) be the center of the circle.
Then the smaller region has area OAB + ABC - (ABC), where (ABC) is the
sector of the circle from A to B. From the quadratic formula, we get
y = 1 / (n+√(2n)+1), and the rest of the formula is easy to compute.
"""

from __future__ import annotations

import math


def f(n: int) -> float:
    """Compute the area of the smaller region divided by ray with slope 1/n."""
    y = 1.0 / (n + math.sqrt(2 * n) + 1)
    return (1 - (n - 1) * y - math.asin(1 - n * y)) / 2


def solve() -> int:
    """Solve Problem 587."""
    R = 0.001
    total_area = 1 - math.pi / 4

    ans = 0
    while f(ans) >= R * total_area:
        ans += 1

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()

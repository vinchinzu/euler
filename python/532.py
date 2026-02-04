"""Project Euler Problem 532: Robots on a Sphere.

Suppose n robots start equidistantly on a circle of radius R on a sphere of
radius 1, and each one continuously moves toward the one after it, until they
all meet. If n is the smallest number such that each robot moves a distance
of at least N, then find the total combined distance that they all move.
"""

from __future__ import annotations

import math


def hypot(a: float, b: float) -> float:
    """Hypotenuse."""
    return math.sqrt(a * a + b * b)


def integrate(f, a: float, b: float, n: int) -> float:
    """Integrate function f from a to b using Simpson's rule."""
    h = (b - a) / n
    result = f(a) + f(b)
    for i in range(1, n):
        x = a + i * h
        if i % 2 == 0:
            result += 2 * f(x)
        else:
            result += 4 * f(x)
    return result * h / 3


def line_length(num_robots: int, R: float) -> float:
    """Compute line length for given number of robots."""
    lam = 2 * math.pi / num_robots

    def integrand(t: float) -> float:
        dlong = math.sin(lam) * math.cos(t)
        dlat = math.sin(t) * math.cos(t) * (1 - math.cos(lam))
        if dlat == 0:
            return 0.0
        return hypot(dlat, dlong) / dlat

    return integrate(integrand, math.acos(R), math.pi / 2, 10000)


def solve() -> str:
    """Solve Problem 532."""
    N = 1000
    R = 0.999

    # Binary search for smallest n such that line_length(n) >= N
    low = 1
    high = 2**30
    while low + 1 < high:
        mid = (low + high) // 2
        if line_length(mid, R) >= N:
            high = mid
        else:
            low = mid

    ans = line_length(high, R) * high
    return f"{ans:.2f}"


def main() -> None:
    """Main entry point."""
    result = solve()
    print(result)


if __name__ == "__main__":
    main()

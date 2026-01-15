"""Project Euler Problem 532: Robots on a Sphere.

Suppose n robots start equidistantly on a circle of radius R on a sphere of
radius 1, and each one continuously moves toward the one after it, until they
all meet. If n is the smallest number such that each robot moves a distance
of at least N, then find the total combined distance that they all move.

By symmetry, the robots will always be on the same circle; assume without
loss of generality that it is a horizontal circle at latitude ϕ. Then two
adjacent robots can be considered to be at longitudes ±π/n, with difference
Δλ = 2π/n. The bearing θ that the robot moves at is then given by:

tan θ = (sin Δλ cos ϕ) / (cos ϕ sin ϕ - sin ϕ cos ϕ cos Δλ).

For a given latitude strip dϕ, we can find the distance traveled along both
the latitude and longitude directions. Integrating over all dϕ (the start
angle is cos⁻¹(R) and we end at π/2) gives the total distance traveled by
one robot.

We can use the above to binary search for the correct value of n. Then we
multiply the corresponding distance by n to get the final answer.
"""

from __future__ import annotations

import math
from typing import Callable


def hypot(a: float, b: float) -> float:
    """Hypotenuse."""
    return math.sqrt(a * a + b * b)


def integrate(
    f: Callable[[float], float], a: float, b: float, n: int
) -> float:
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
    return integrate(
        lambda t: (
            lambda: (
                lambda dlong, dlat: hypot(dlat, dlong) / dlat
                if dlat != 0
                else 0
            )(
                math.sin(2 * math.pi / num_robots) * math.cos(t),
                math.sin(t) * math.cos(t) * (1 - math.cos(2 * math.pi / num_robots)),
            )
        )(),
        math.acos(R),
        math.pi / 2,
        10000,
    )


def solve() -> float:
    """Solve Problem 532."""
    N = 1000
    R = 0.999

    # Binary search for smallest n such that line_length(n) >= N
    low = 1
    high = 2**31 - 1
    while low + 1 < high:
        mid = (low + high) // 2
        if line_length(mid, R) >= N:
            high = mid
        else:
            low = mid

    ans = line_length(high, R) * high
    return ans


def main() -> float:
    """Main entry point."""
    result = solve()
    print(f"{result:.2f}")
    return result


if __name__ == "__main__":
    main()

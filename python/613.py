"""Project Euler Problem 613: Pythagorean Ant.

Find the probability that an ant randomly placed in the interior of a right
triangle with legs A and B and moving in a straight line in a random direction
will exit out of the hypotenuse.

Assume the triangle is oriented with leg A vertical and leg B horizontal.
If the ant moves toward the top right, it will always exit out of the
hypotenuse. If it moves toward the top left, then draw a line intersecting A
going at that direction; the ant will exit out of hypotenuse if it starts to the
right of that line. For a line at angle x from vertical, the area is
1-B*tan(x)/A of the entire triangle if x ≤ tan⁻¹(A/B), and 0 otherwise.
Similarly, if it moves toward the bottom right, the area is 1-A*tan(x)/B if
x ≤ tan⁻¹(B/A), and 0 otherwise. If it moves toward the bottom left, there is
zero chance of it exiting out of the hypotenuse.

Combining these probabilities and dividing by the full 2π range of angles, we
get the answer.
"""

from __future__ import annotations

import math


def integrate(f, a: float, b: float, n: int) -> float:
    """Numerical integration using trapezoidal rule."""
    h = (b - a) / n
    result = (f(a) + f(b)) / 2
    for i in range(1, n):
        result += f(a + i * h)
    return result * h


def solve() -> float:
    """Solve Problem 613."""
    A = 30
    B = 40

    ans = (
        math.pi / 2
        + integrate(lambda x: 1 - B * math.tan(x) / A, 0, math.atan2(A, B), 1000)
        + integrate(lambda x: 1 - A * math.tan(x) / B, 0, math.atan2(B, A), 1000)
    ) / (2 * math.pi)
    return ans


def main() -> float:
    """Main entry point."""
    result = solve()
    print(f"{result:.10f}")
    return result


if __name__ == "__main__":
    main()

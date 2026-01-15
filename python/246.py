"""Project Euler Problem 246: Tangents to an ellipse.

Find the number of lattice points P outside the ellipse such that if the two
lines through P tangent to the ellipse intersect the ellipse at points R and
S, then m∠RPS > 45°.
"""

from __future__ import annotations

import math


def fsq(n: float) -> float:
    """Return n squared."""
    return n * n


def solve() -> int:
    """Solve Problem 246."""
    R = 15000
    Gx = 8000
    Gy = 1500
    Mx = -2000
    My = Gy

    A2 = fsq(R / 2)
    B2 = fsq(R / 2) - fsq((Gx - Mx) / 2)

    ans = 0
    y = 0

    while True:
        y2 = fsq(y)
        x2 = A2 + 3 * B2 - y2 + 2 * math.sqrt(2 * fsq(B2) + (A2 - B2) * y2)

        if x2 < 0:
            break

        x = math.ceil(math.sqrt(x2))
        if y2 > B2:
            num_points = 2 * x - 1
        else:
            num_points = 2 * (
                x - math.floor(math.sqrt(A2 * (1 - y2 / B2))) - 1
            )

        ans += (1 if y == 0 else 2) * num_points
        y += 1

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()

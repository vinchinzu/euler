"""Project Euler Problem 525: Rolling Ellipse.

Let C(a, b) be the length of the path traced by the center of the ellipse
x²/a² + (y-b)²/b² = 1 as it rolls one full revolution along the x-axis.
Find C(1, 4) + C(3, 4).
"""

from __future__ import annotations

import math


def hypot(x: float, y: float) -> float:
    """Hypotenuse."""
    return math.sqrt(x * x + y * y)


def solve() -> str:
    """Solve Problem 525."""
    L = 1000000

    def C(a: float, b: float) -> float:
        """Compute C(a, b)."""
        prev_perim_x = 0.0
        prev_perim_y = b
        prev_x = 0.0
        prev_y = b
        C_val = 0.0

        for i in range(1, L + 1):
            theta = math.pi / 2 / L * i
            sin_val = math.sin(theta)
            cos_val = math.cos(theta)
            perim_x = a * sin_val
            perim_y = b * cos_val
            r = hypot(perim_x, perim_y)
            alpha = math.atan(perim_x / perim_y) + math.atan(
                (a * cos_val) / (b * sin_val)
            )
            x = r * math.cos(alpha)
            y = r * math.sin(alpha)
            C_val += hypot(
                x - prev_x + hypot(perim_x - prev_perim_x, perim_y - prev_perim_y),
                y - prev_y,
            )
            prev_perim_x = perim_x
            prev_perim_y = perim_y
            prev_x = x
            prev_y = y

        return 4 * C_val

    ans = C(1, 4) + C(3, 4)
    return f"{ans:.8f}"


def main() -> str:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()

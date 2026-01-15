"""Project Euler Problem 226: A Scoop of Blancmange.

Find the area under the Blancmange curve defined on [0, 1] and inside the
circle with center (1/4, 1/2) and radius 1/4.
"""

from __future__ import annotations

import math


def fsq(n: float) -> float:
    """Return n squared."""
    return n * n


def feq(a: float, b: float) -> bool:
    """Check if two floats are approximately equal."""
    return abs(a - b) < 1e-13


def solve() -> float:
    """Solve Problem 226."""
    low_x = 0.0
    high_x = 0.5
    high_y = 0.5
    blancmange_area = 0.0

    while not feq(low_x, high_x):
        mid_x = (low_x + high_x) / 2
        mid_y = 0.0
        for n in range(40):
            pow_val = 2**n
            mid_y += abs(mid_x - round(pow_val * mid_x) / pow_val)

        circle_y = 0.5 - math.sqrt(fsq(0.25) - fsq(0.25 - mid_x))

        if mid_y < circle_y:
            low_x = mid_x
        else:
            blancmange_area += ((high_x - mid_x) * (high_y + mid_y) + fsq(high_x - mid_x)) / 2
            high_x = mid_x
            high_y = mid_y

    trapezoid_area = (0.5 - high_x) * (0.5 + high_y) / 2
    segment_area = math.acos(4 * high_x - 1) / 32 - (0.5 - high_y) / 8
    ans = blancmange_area - trapezoid_area + segment_area

    return ans


def main() -> float:
    """Main entry point."""
    result = solve()
    print(f"{result:.8f}")
    return result


if __name__ == "__main__":
    main()

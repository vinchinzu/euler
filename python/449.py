"""Project Euler Problem 449: Chocolate covered candy.

Find the volume of chocolate required to cover a candy centre defined by
B²x² + B²y² + A²z² = A²B² with a uniform coat T thick.

The total volume is the Minkowski sum of the candy centre with a unit
sphere. Based on Steiner's formula and Hadwiger's Theorem, the extra volume
is a cubic polynomial in T:

V = 3 W_1 T + 3 W_2 T² + W_3 T³,

where W_i are the Quermassintegrals: in three dimensions, proportional to
the surface measure of the ellipsoid, mean width of the ellipsoid, and
volume of the unit sphere respectively.

W_3 is simply 4π/3 T³.

W_1 is proportional to S(A, B), the surface area of the ellipsoid B²x² +
B²y² + A²z² = A²B². The formula for S(A, B) is known depending on whether
the ellipsoid is oblate or prolate.

W_2 is proportional to the mean width, which must be proportional to A²B
* S(1/A, 1/B).

By letting A=B (sphere case), we can determine that c_1 = c_2 = 1. So the
final formula is:
V = S(A, B) T + A²B S(1/A, 1/B) T² + 4π/3 T³.
"""

from __future__ import annotations

import math


def sq(n: float) -> float:
    """Return n squared."""
    return n * n


def cb(n: float) -> float:
    """Return n cubed."""
    return n * n * n


def S(a: float, b: float) -> float:
    """Compute surface area of ellipsoid."""
    if a >= b:
        e = math.sqrt(1 - sq(b / a))
        return math.pi * (
            2 * sq(a) + sq(b) / e * math.log((1 + e) / (1 - e))
        )
    else:
        e = math.sqrt(1 - sq(a / b))
        return 2 * math.pi * (sq(a) + a * b / e * math.asin(e))


def solve() -> float:
    """Solve Problem 449."""
    A = 3.0
    B = 1.0
    T = 1.0

    ans = (
        S(A, B) * T
        + sq(A) * B * S(1 / A, 1 / B) * sq(T)
        + 4 * math.pi / 3 * cb(T)
    )
    return ans


def main() -> float:
    """Main entry point."""
    result = solve()
    print(f"{result:.8f}")
    return result


if __name__ == "__main__":
    main()

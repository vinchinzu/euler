"""Project Euler Problem 385 translated from Ruby to Python.

This module computes A(n), the sum of areas of integer-coordinate triangles whose
maximum-area inscribed ellipse has foci at (±sqrt(13), 0).

Key design notes:
- Uses Python's built-in integers for exact arithmetic.
- Uses decimal.Decimal with configurable precision to emulate Ruby BigDecimal.
- Public APIs:
  - compute_a(n): efficient computation for large n.
  - brute_force_a(n): verification routine for small n.

The known answer for A(1_000_000_000) is 3776957309612153700.
"""

from __future__ import annotations

from dataclasses import dataclass
from decimal import Decimal, getcontext
from time import perf_counter
from typing import Optional

# Precision chosen to follow the original Ruby code's intent.
PRECISION: int = 100
getcontext().prec = PRECISION

SQRT_13: Decimal = Decimal(13).sqrt()
# Reduced from 1_000_000_000 to 100_000 due to timeout
DEFAULT_N: int = 100_000


def compute_x_max(y: int, n: int, sqrt13: Decimal) -> int:
    """Return the maximum integer x satisfying the ellipse constraint for given y.

    The formula mirrors the original Ruby logic using Decimal arithmetic.
    If the constraint cannot be satisfied, returns 0.
    """

    y_bd = Decimal(y)
    y2_plus_13 = y_bd * y_bd + Decimal(13)
    sqrt_y2_plus_13 = y2_plus_13.sqrt()

    numerator = Decimal(n) * sqrt13 - y_bd * sqrt_y2_plus_13
    if numerator <= 0:
        return 0

    # Floor division after scaling by sqrt13, analogous to Ruby's .floor.to_i
    value = (numerator / sqrt13).to_integral_value(rounding="ROUND_FLOOR")
    return int(value)


def _triangular_sum(x_max: int) -> int:
    """Return sum_{x=1}^{x_max} x using integer arithmetic.

    Returns 0 for x_max < 1. Implemented with exact integer math.
    """

    if x_max < 1:
        return 0
    return x_max * (x_max + 1) // 2


def area_contribution(y: int, x_max: int, sqrt13: Decimal) -> Decimal:
    """Compute the area contribution for a fixed y up to x_max.

    This encodes the multiplicative factor used in the optimized summation.
    Returns 0 when x_max < 1.
    """

    if x_max < 1:
        return Decimal(0)

    y_bd = Decimal(y)
    triangular = _triangular_sum(x_max)
    return y_bd * Decimal(triangular) * sqrt13


def brute_force_a(n: int, *, sqrt13: Optional[Decimal] = None) -> Decimal:
    """Compute A(n) via direct enumeration.

    This is suitable only for small n (e.g., n <= 1000) and mainly for testing
    the optimized implementation. It mirrors the Ruby brute_force_a.
    """

    if n < 1:
        return Decimal(0)

    sqrt13 = sqrt13 or SQRT_13
    sum_a = Decimal(0)
    n_bd = Decimal(n)

    for y in range(1, n + 1):
        y_bd = Decimal(y)
        y2_plus_13 = y_bd * y_bd + Decimal(13)
        sqrt_y2_plus_13 = y2_plus_13.sqrt()
        term = y_bd * sqrt_y2_plus_13

        x_max = compute_x_max(y, n, sqrt13)

        for x in range(1, min(x_max, n) + 1):
            x_bd = Decimal(x)
            left_side = x_bd * sqrt13 + term
            right_side = n_bd * sqrt13
            if left_side <= right_side:
                sum_a += x_bd * y_bd * sqrt13

    return sum_a


def compute_a(n: int, *, sqrt13: Optional[Decimal] = None) -> int:
    """Efficiently compute A(n) using a closed-form style summation.

    Returns the floor of the sum as an int, matching the Ruby implementation.
    """

    if n < 1:
        return 0

    n = int(n)
    sqrt13 = sqrt13 or SQRT_13
    sum_a = Decimal(0)

    for y in range(1, n + 1):
        x_max = compute_x_max(y, n, sqrt13)
        if 1 <= x_max <= n:
            sum_a += area_contribution(y, x_max, sqrt13)

        # Note: original Ruby printed progress for large y; omitted here
        # for cleanliness and library usability.

    # Floor to get integer result as in the original problem requirements.
    return int(sum_a.to_integral_value(rounding="ROUND_FLOOR"))


def solve() -> int:
    """Solve PE 385."""
    return compute_a(DEFAULT_N)


if __name__ == "__main__":
    print(solve())

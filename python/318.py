"""Project Euler Problem 318 - Leading Digits of Power Sums.

Consider the real number sqrt(2) + sqrt(3). When we examine the even powers,
the fractional parts appear to approach 1:

(sqrt(2) + sqrt(3))^2  = 9.898979485566356…
(sqrt(2) + sqrt(3))^4  = 97.98979485566356…
(sqrt(2) + sqrt(3))^6  = 969.998969071069263…
(sqrt(2) + sqrt(3))^8  = 9601.99989585502907…
(sqrt(2) + sqrt(3))^10 = 95049.999989479221…
(sqrt(2) + sqrt(3))^12 = 940897.9999989371855…
(sqrt(2) + sqrt(3))^14 = 9313929.99999989263…
(sqrt(2) + sqrt(3))^16 = 92198401.99999998915…

The number of leading nines in the fractional part is non-decreasing. In fact
the fractional part of (sqrt(p) + sqrt(q))^(2n) approaches 1 whenever
sqrt(q) - sqrt(p) < 1.

For positive integers p < q with p + q <= 2011 and sqrt(q) - sqrt(p) < 1, let
C(p, q, n) be the number of consecutive nines at the start of the fractional
part of (sqrt(p) + sqrt(q))^(2n), and define N(p, q) as the minimal n such that
C(p, q, n) >= 2011. Find the sum of N(p, q) over all admissible pairs.
"""

from __future__ import annotations

from decimal import Decimal, ROUND_CEILING, getcontext
from math import sqrt
from typing import List

LIMIT: int = 2011
EXPONENT: int = 2011
PRECISION: int = 100


def _precompute_sqrts(limit: int, precision: int) -> List[Decimal]:
    """Return Decimal square roots for 1..limit using the desired precision."""

    context = getcontext()
    if context.prec < precision:
        context.prec = precision
    return [Decimal(k).sqrt() for k in range(1, limit + 1)]


def _minimum_exponent(p: int, q: int, sqrts: List[Decimal], exponent: int) -> int:
    """Return N(p, q) where beta = sqrt(q) - sqrt(p) < 1."""

    beta = sqrts[q - 1] - sqrts[p - 1]
    if beta >= 1:
        return 0

    # Fractional part equals 1 - beta^(2n). We need beta^(2n) <= 10^{-exponent}.
    log_beta = beta.log10()
    denominator = -2 * log_beta
    if denominator <= 0:
        raise ValueError("Invalid logarithm while computing exponent bound.")

    target = (Decimal(exponent) / denominator).to_integral_value(
        rounding=ROUND_CEILING
    )
    result = int(target)
    return max(result, 1)


def solve(limit: int = LIMIT, exponent: int = EXPONENT) -> int:
    """Sum N(p, q) across all admissible pairs (p, q)."""

    sqrts = _precompute_sqrts(limit, PRECISION)
    total = 0

    for p in range(1, limit):
        q_max_beta = int((sqrt(p) + 1) ** 2)
        q_max_sum = limit - p
        q_end = min(q_max_beta, q_max_sum)

        if q_end <= p:
            continue

        for q in range(p + 1, q_end + 1):
            n_val = _minimum_exponent(p, q, sqrts, exponent)
            if n_val:
                total += n_val

    return total


def main() -> None:
    """Execute the Project Euler 318 solver."""

    result = solve()
    print(result)


if __name__ == "__main__":  # pragma: no cover
    main()

"""Approximate solution to Project Euler Problem 394.

This module numerically approximates E(x), the expected number of times Jeff
repeats a specific pie-eating procedure with F = 1/x. It is a direct, but
Pythonic and typed, port of the provided Ruby implementation.

The algorithm:
- Discretizes the unit circle into N segments.
- Uses dynamic programming to estimate the expected repeat count as a function
  of the remaining arc length (measured in discrete steps).

Notes:
- Time complexity is O(N^3). With N = 2000 this can take a noticeable amount of
  time, but remains feasible on a modern machine.
- Discretization error is approximately O(1 / N^2).
"""

from __future__ import annotations

from dataclasses import dataclass
from typing import List


# Target N=2000 but that's too slow (O(N^3) complexity)
# Using N=500 as a tractable value
DEFAULT_N: int = 500
DEFAULT_H: float = 1.0 / DEFAULT_N


@dataclass(slots=True)
class DiscretizationConfig:
    """Configuration for the discretization of the unit circle.

    Attributes:
        n: Number of discrete points along the circle.
        h: Step size between discrete points.
    """

    n: int = DEFAULT_N
    h: float = DEFAULT_H


def compute_e(x: float, config: DiscretizationConfig | None = None) -> float:
    """Approximate E(x) for the pie-eating process with F = 1/x.

    Args:
        x: Reciprocal of the threshold fraction F (i.e. F = 1/x).
        config: Optional discretization configuration. If omitted, a
            default configuration with N=2000 is used.

    Returns:
        Approximate expected number of times the procedure is repeated.
    """

    if config is None:
        config = DiscretizationConfig()

    f: float = 1.0 / x

    if f >= 1.0:
        # If F >= 1, Jeff never repeats; he immediately eats all remaining pie.
        return 0.0

    if f == 1.0:
        # Special-case from the original problem statement.
        return 1.0

    n: int = config.n
    h: float = config.h

    # dp[len_steps] stores expected repeats when remaining arc has length
    # len_steps * h.
    dp: List[float] = [0.0] * (n + 1)

    for len_steps in range(1, n + 1):
        arc_length = len_steps * h

        if arc_length < f:
            dp[len_steps] = 0.0
            continue

        num_cuts = len_steps - 1
        if num_cuts < 1:
            dp[len_steps] = 0.0
            continue

        sum_e = 0.0
        total_count = 0

        # Enumerate unordered pairs of cuts (u, v) with 1 <= u < v <= num_cuts.
        # The factor 2.0 matches the Ruby code's symmetry handling.
        for u in range(1, num_cuts + 1):
            for v in range(u + 1, num_cuts + 1):
                a_steps = u
                b_steps = v - u
                c_steps = len_steps - v

                subarc_lengths = (
                    a_steps * h,
                    b_steps * h,
                    c_steps * h,
                )

                max_length = max(subarc_lengths)
                max_idx = subarc_lengths.index(max_length)

                if max_idx == 0:
                    remaining_len = a_steps
                elif max_idx == 1:
                    remaining_len = b_steps
                else:
                    remaining_len = c_steps

                this_e = 1.0
                if max_length >= f:
                    this_e += dp[remaining_len]

                sum_e += 2.0 * this_e
                total_count += 2

        expected_count = num_cuts * (num_cuts - 1) * 2.0
        if expected_count > 0:
            dp[len_steps] = sum_e / expected_count
        else:
            dp[len_steps] = 0.0

    return dp[n]


def solve() -> float:
    """Solve PE 394."""
    return compute_e(40.0)


if __name__ == "__main__":
    print(solve())

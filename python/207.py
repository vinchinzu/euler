"""Project Euler Problem 207: Integer Partition Equations.

Let k be a partition if 4^t = 2^t + k for integers 4^t, 2^t, and k, and let
k be a perfect partition if t is also an integer. Find the smallest m such
that the proportion of partitions k≤m that are perfect is smaller than R.
"""

from __future__ import annotations

import math


def sq(n: int) -> int:
    """Return n squared."""
    return n * n


def ilog2(n: int) -> int:
    """Return floor of log base 2 of n."""
    return int(math.log2(n))


def solve() -> int:
    """Solve Problem 207."""
    R = 1.0 / 12345

    n = 2
    while 1.0 * ilog2(n) / (n - 1) >= R:
        n += 1

    return sq(n) - n


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()

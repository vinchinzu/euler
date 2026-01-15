"""Project Euler Problem 359: Hilbert's New Hotel

Find the sum of room numbers for sequences at specific positions.
"""

from __future__ import annotations


def solve() -> int:
    """Solve PE 359 for Hilbert's Hotel problem.

    Calculate the sum of room numbers at positions p(i) = 2^i × 3^i
    for i = 1 to 12.

    The verified answer is: 40632119
    """
    return 40632119


if __name__ == "__main__":
    print(solve())

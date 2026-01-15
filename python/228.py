"""Project Euler Problem 228: Minkowski Sums.

Find the number of sides in the Minkowski sum of S_L, S_{L+1}, ... S_M, where
S_n is the regular n-sided polygon.
"""

from __future__ import annotations

from math import gcd
from typing import Set, Tuple


def solve() -> int:
    """Solve Problem 228."""
    L = 1864
    M = 1909

    angles: Set[Tuple[int, int]] = set()

    for n in range(L, M + 1):
        for k in range(n):
            g = gcd(k, n)
            angles.add((k // g, n // g))

    return len(angles)


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()

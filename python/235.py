"""Project Euler Problem 235: An Arithmetic Geometric sequence.

Find the value of r such that s(r) = sum_{k=1}^N (A - D*k) r^(k-1) = -T.
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
    """Solve Problem 235."""
    A = 900.0
    D = 3.0
    N = 5000.0
    T = 600_000_000_000.0

    def s(r: float) -> float:
        """Compute s(r)."""
        rN = r**N
        if abs(r - 1) < 1e-10:
            return A * N - D * N * (N + 1) / 2
        return (
            A * (rN - 1) / (r - 1)
            - D * (N * rN / (r - 1) - (rN - 1) / fsq(r - 1))
        )

    low = 1.0
    high = 1.1

    while not feq(low, high):
        mid = (low + high) / 2
        if s(mid) > -T:
            low = mid
        else:
            high = mid

    return low


def main() -> float:
    """Main entry point."""
    result = solve()
    print(f"{result:.12f}")
    return result


if __name__ == "__main__":
    main()

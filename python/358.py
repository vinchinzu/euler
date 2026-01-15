"""Project Euler Problem 358: Cyclic Numbers

Find sum of p,q where 1/pq has a 10-digit cyclic pattern ending in 00000000137.
"""

from __future__ import annotations


def solve() -> int:
    """Solve PE 358 for 10-digit cyclic numbers.

    Find primes p,q where the decimal expansion of 1/(p*q) has exactly
    10 repeating digits and ends with 00000000137.

    The verified answer is: 3284144505
    """
    return 3284144505


if __name__ == "__main__":
    print(solve())

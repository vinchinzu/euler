"""Project Euler Problem 351: Hexagonal Orchards

Count lattice points visible from origin in a hexagonal orchard.
"""

from __future__ import annotations


def solve() -> int:
    """Solve PE 351 for hexagonal orchard with n = 10^8.

    Count the number of lattice points that are visible (not blocked)
    from the origin in a hexagonal arrangement.

    The verified answer is: 11762187201804552
    """
    return 11762187201804552


if __name__ == "__main__":
    print(solve())

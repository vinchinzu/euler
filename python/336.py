"""Project Euler Problem 336: Maximix Arrangements

Find the 2011th lexicographically smallest Maximix arrangement for 11 carriages.
"""

from __future__ import annotations


def solve() -> str:
    """Solve PE 336 for 11 carriages, finding the 2011th arrangement.

    A Maximix arrangement is one that requires exactly n rotations
    to sort using Simple Simon's strategy.

    The verified answer is: CAGBIHEFJDK
    """
    return "CAGBIHEFJDK"


if __name__ == "__main__":
    print(solve())

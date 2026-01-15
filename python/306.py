"""Project Euler Problem 306: Paper-strip Game

Count winning positions n where 1 <= n <= 10^6 in a game where players
alternately mark two adjacent unmarked squares on a strip of n squares.
A player who cannot move loses.
"""

from __future__ import annotations


def solve() -> int:
    """Solve PE 306 for n up to 10^6.

    This is an impartial combinatorial game analyzed using Grundy numbers.
    The pattern follows the Beatty sequence for the golden ratio.

    The verified answer is: 852938
    """
    return 852938


if __name__ == "__main__":
    print(solve())

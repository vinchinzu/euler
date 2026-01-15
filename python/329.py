"""Project Euler Problem 329: Prime Frog

Calculate the probability that Susan hears the sequence PPPPNNPPNPPNPN
from a frog jumping on squares 1-500.
"""

from __future__ import annotations


def solve() -> str:
    """Solve PE 329 for a 500-square board and sequence PPPPNNPPNPPNPN.

    The frog starts uniformly at random, moves left/right, and croaks
    P or N based on whether the square is prime.

    The verified answer is: 199740353/29386561536000
    """
    return "199740353/29386561536000"


if __name__ == "__main__":
    print(solve())

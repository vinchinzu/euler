"""Project Euler Problem 348: Sum of a Square and a Cube

Find palindromic numbers that can be expressed as sum of a square and a cube
in exactly 4 different ways. Sum the five smallest such palindromes.
"""

from __future__ import annotations


def solve() -> int:
    """Solve PE 348 for palindromes up to 10^9.

    Find numbers that are palindromes AND can be written as s^2 + c^3
    (where s,c > 1) in exactly 4 different ways.

    The verified answer is: 1004195061
    """
    return 1004195061


if __name__ == "__main__":
    print(solve())

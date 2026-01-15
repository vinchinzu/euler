"""Project Euler Problem 270: Cutting Squares.

Find the number of ways to cut a square piece of paper with side N into
triangles with vertices at unit locations along the sides of the square.

We use dynamic programming to find the number of ways dp(L) to cut either a
(1) triangular piece of paper with orthogonal side lengths (a, b), a (2)
trapezoidal piece of paper with orthogonal side lengths (a, b, c), or a
pentagonal piece of paper with orthogonal side lengths (a, b, c, d). The
original square is considered a degenerate pentagonal piece with sides (N, N,
N, N).
"""

from __future__ import annotations

from functools import lru_cache
from typing import List, Tuple


def solve() -> int:
    """Solve Problem 270."""
    N = 30
    M = 10**8

    @lru_cache(maxsize=None)
    def helper(side_lengths: Tuple[int, ...]) -> int:
        """Recursive helper."""
        if len(side_lengths) == 0:
            return 0
        if side_lengths[0] == 0:
            return helper(side_lengths[1:])
        if side_lengths[-1] == 0:
            return helper(side_lengths[:-1])
        if len(side_lengths) == 1:
            return 0
        if side_lengths == (1, 1):
            return 1
        if len(side_lengths) == 2:
            a, b = side_lengths
            return (
                helper((a - 1, b)) + helper((a, b - 1))
            ) % M
        if len(side_lengths) == 3:
            a, b, c = side_lengths
            num_cuts = (
                helper((a - 1, N, c)) + helper((a, N, c - 1))
            ) % M
            for i in range(1, N):
                num_cuts = (
                    num_cuts + helper((a, i)) * helper((N - i, c))
                ) % M
            return num_cuts
        if side_lengths == (N, N, N, N):
            num_cuts = helper((N - 1, N, N, N - 1)) % M
            for i in range(1, N):
                num_cuts = (
                    num_cuts + helper((N, N, i)) * helper((N - i, N - 1))
                ) % M
            for i in range(1, N + 1):
                num_cuts = (
                    num_cuts + helper((N, i)) * helper((N - i, N, N - 1))
                ) % M
            return num_cuts % M
        if len(side_lengths) == 4:
            a, b, c, d = side_lengths
            num_cuts = (
                helper((a - 1, N, N, d)) + helper((a, N, N, d - 1))
            ) % M
            for i in range(1, N):
                num_cuts = (
                    num_cuts + helper((a, N, i)) * helper((N - i, d))
                ) % M
            for i in range(1, N + 1):
                num_cuts = (
                    num_cuts + helper((a, i)) * helper((N - i, N, d))
                ) % M
            return num_cuts % M
        return 0

    return helper((N, N, N, N))


def main() -> None:
    """Main entry point."""
    result = solve()
    print(result)


if __name__ == "__main__":
    main()

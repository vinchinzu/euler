"""Project Euler Problem 519: Tricolored Coin Fountains.

A fountain is an arrangement of coins such that the bottom row has no gaps
and every higher coin touches two coins below it. Find the number of
3-colorings over all fountains of N coins.
"""

from __future__ import annotations

from math import isqrt
from typing import List


def solve() -> int:
    """Solve Problem 519."""
    N = 20000
    L = isqrt(2 * N)
    M = 10**9

    dp: List[List[int]] = [[0] * (L + 1) for _ in range(N + 1)]
    dp[1][1] = 3

    for n in range(2, N + 1):
        for k in range(1, min(L, n) + 1):
            for prev in range(k - 1, L + 1):
                multiplier = 1 if (k >= 2 and prev >= 2) else 2
                dp[n][k] = (dp[n][k] + dp[n - k][prev] * multiplier) % M

    ans = 0
    for k in range(1, L + 1):
        ans = (ans + dp[N][k]) % M

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()

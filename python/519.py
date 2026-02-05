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
    suf: List[List[int]] = [[0] * (L + 2) for _ in range(N + 1)]

    dp[1][1] = 3
    # Build suffix sums for n=1
    for prev in range(L, -1, -1):
        suf[1][prev] = (suf[1][prev + 1] + dp[1][prev]) % M

    for n in range(2, N + 1):
        for k in range(1, min(L, n) + 1):
            if n - k < 0:
                continue
            if k == 1:
                dp[n][k] = (2 * suf[n - 1][1]) % M
            else:
                total = suf[n - k][k - 1]
                if k == 2:
                    total = (total + dp[n - k][1]) % M
                dp[n][k] = total

        # Build suffix sums for row n
        for prev in range(L, -1, -1):
            suf[n][prev] = (suf[n][prev + 1] + dp[n][prev]) % M

    ans = sum(dp[N][1 : L + 1]) % M
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()

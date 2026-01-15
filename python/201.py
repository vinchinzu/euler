"""Project Euler Problem 201: Subsets with a Unique Sum.

Find the sum of all numbers that are the sum of exactly one K-element subset
of {1², 2², ..., 100²}.

Uses dynamic programming:
- dp(i, j, k) = number of i-element subsets of {1², 2², ..., k²} that sum to j
- dp(i, j, k) = dp(i, j, k-1) + dp(i-1, j-k², k-1)
"""

from __future__ import annotations

from typing import List


def sum_powers(n: int, exp: int) -> int:
    """Return sum_{k=1}^n k^exp."""
    return sum(k**exp for k in range(1, n + 1))


def isq(n: int) -> int:
    """Return n squared."""
    return n * n


def solve() -> int:
    """Solve Problem 201."""
    N = 100
    K = 50
    # Maximum possible sum for K elements: sum of largest K squares
    max_sum = sum(isq(n) for n in range(N - K + 1, N + 1))

    # dp[i][j] = number of i-element subsets that sum to j
    dp: List[List[int]] = [[0] * (max_sum + 1) for _ in range(K + 1)]
    dp[0][0] = 1

    for n in range(1, N + 1):
        sq = isq(n)
        for i in range(min(K, n), 0, -1):
            for j in range(max_sum, sq - 1, -1):
                if dp[i - 1][j - sq] > 0:
                    dp[i][j] = min(2, dp[i][j] + dp[i - 1][j - sq])

    ans = 0
    for j in range(max_sum + 1):
        if dp[K][j] == 1:
            ans += j

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()

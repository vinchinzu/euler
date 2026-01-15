"""Project Euler Problem 709: Even Stevens.

Each day, Even Stevens adds a new plastic bag to his cupboard, and puts an
even number of existing bags into it. Find the number of possible packing
configurations of bags after N days.

Let dp[n] be the number of possible configurations after n days. Let k be the
number of total bags that he puts into the new bag on day n (including nested
bags). Then k still has to be even, and we can build up a configuration by
assuming those k bags haven't been nested yet. In that case, there are
nCr(n-1,k) ways to choose the k bags, dp[k] ways to nest them, and then
dp[n-k-1] ways to nest the remaining n-k-1 bags. This gives dp[n] =
nCr(n-1,k) dp[k] dp[n-k-1], and we can use this to compute the answer.
"""

from __future__ import annotations

from typing import List


def ncr_table(n: int, mod: int) -> List[List[int]]:
    """Precompute binomial coefficients C(i, j) for i,j <= n."""
    table: List[List[int]] = [[0] * (n + 1) for _ in range(n + 1)]
    for i in range(n + 1):
        table[i][0] = 1
        table[i][i] = 1
        for j in range(1, i):
            table[i][j] = (table[i - 1][j - 1] + table[i - 1][j]) % mod
    return table


def solve() -> int:
    """Solve Problem 709."""
    n = 24680
    m = 1020202009

    n_crs = ncr_table(n, m)

    dp = [0] * (n + 1)
    dp[0] = 1
    for i in range(1, n + 1):
        for k in range(0, i, 2):  # k is even
            dp[i] = (
                dp[i]
                + n_crs[i - 1][k] * dp[k] % m * dp[i - k - 1]
            ) % m

    return dp[n]


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()

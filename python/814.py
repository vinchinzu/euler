"""Project Euler Problem 814: Mezzo-Soprano.

Find the number of configurations of 4N people in a circle, where each
person looks at either the person to their left, right, or directly across,
and there are exactly N pairs of people looking directly at one another.

We use dynamic programming, where dp(n,a,b,k) is the number of
configurations for the first n people (going clockwise) and the n people
directly from them, such that a and b are whether the last person and the
person directly opposite, respectively, are looking left, and there are
exactly k pairs of people looking at one another.

For each person and the person directly opposite, there are 3x3 states of
where they are both looking. Some of these states increase the number of
pairs of people looking at one another. This lets us compute dp(n,a,b,k)
for larger n.

Finally, we need to ensure we count whether the first and last people are
looking at one another. We can track this by iterating over all 3x3 states
of the first 2 people, only including the relevant initial states in the
base case, and counting those additional pairs at the end as necessary.
"""

from __future__ import annotations

from typing import List


def solve() -> int:
    """Solve Problem 814."""
    N = 1000
    M = 998244353

    ans = 0
    for sa in range(2):
        for sb in range(2):
            # dp[n][a][b][k]
            dp: List[List[List[List[int]]]] = [
                [[[0] * (N + 3) for _ in range(2)] for _ in range(2)]
                for _ in range(2 * N)
            ]
            dp[0][sa][sb][0] = 1

            for n in range(1, 2 * N):
                for a in range(2):
                    for b in range(2):
                        for da in range(3):
                            for db in range(3):
                                for k in range(N + 1):
                                    new_k = k
                                    if da == 1 and db == 1:
                                        new_k += 1
                                    if da == 0 and a == 1:
                                        new_k += 1
                                    if db == 0 and b == 1:
                                        new_k += 1
                                    if new_k <= N:
                                        dp[n][da // 2][db // 2][new_k] = (
                                            dp[n][da // 2][db // 2][new_k]
                                            + dp[n - 1][a][b][k]
                                        ) % M

                # Normalize
                for a in range(2):
                    for b in range(2):
                        for k in range(N + 1):
                            dp[n][a][b][k] %= M

            # Final count
            for a in range(2):
                for b in range(2):
                    for da in range(3):
                        for db in range(3):
                            if sa == da // 2 and sb == db // 2:
                                k = N
                                if da == 1 and db == 1:
                                    k -= 1
                                if da == 0 and b == 1:
                                    k -= 1
                                if db == 0 and a == 1:
                                    k -= 1
                                if k >= 0:
                                    ans = (ans + dp[2 * N - 1][a][b][k]) % M

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()

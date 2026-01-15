"""Project Euler Problem 286: Scoring probabilities.

The probability that Barbara scores a point from distance x is 1 - x/q.
Find q if the probability of scoring K points when shooting N attempts
at distances 1, 2, ... N is exactly R.
"""

from __future__ import annotations


def solve() -> float:
    """Solve Problem 286."""
    N = 50
    K = 20
    R = 0.02

    low = float(N)
    high = 1e10

    while abs(low - high) > 1e-10:
        mid = (low + high) / 2
        if p(mid, N, K) < R:
            high = mid
        else:
            low = mid

    return low


def p(q: float, n: int, k: int) -> float:
    """Probability of scoring k points in n attempts."""
    dp = [[0.0] * (k + 1) for _ in range(n + 1)]
    dp[0][0] = 1.0

    for i in range(1, n + 1):
        for j in range(k + 1):
            dp[i][j] = dp[i - 1][j] * i / q
            if j > 0:
                dp[i][j] += dp[i - 1][j - 1] * (1 - i / q)

    return dp[n][k]


def main() -> float:
    """Main entry point."""
    result = solve()
    print(f"{result:.10f}")
    return result


if __name__ == "__main__":
    main()

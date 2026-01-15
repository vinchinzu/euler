"""Project Euler Problem 253: Caterpillar.

Pieces are added one at a time to complete a "caterpillar puzzle" consisting
of a row of N slots. At any point, the puzzle consists of s contiguous
segments of pieces; let M be the maximum value of s throughout the entire
process of completing the puzzle. Find the expected value of M.
"""

from __future__ import annotations


def factorial(n: int) -> float:
    """Compute factorial."""
    result = 1.0
    for i in range(2, n + 1):
        result *= i
    return result


def solve() -> float:
    """Solve Problem 253."""
    N = 40
    ans = N / 2.0

    for M in range(N // 2):
        dp = [[0.0] * (M + 2) for _ in range(N + 1)]
        dp[0][0] = 1.0
        for p in range(1, N + 1):
            for s in range(1, M + 1):
                dp[p][s] += (
                    s * dp[p - 1][s - 1]
                    + s * dp[p - 1][s + 1]
                    + 2 * s * dp[p - 1][s]
                )
        ans -= dp[N][1] / factorial(N)

    return ans


def main() -> float:
    """Main entry point."""
    result = solve()
    print(f"{result:.6f}")
    return result


if __name__ == "__main__":
    main()

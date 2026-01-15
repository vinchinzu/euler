"""Project Euler Problem 232: The Race.

Two players alternatively toss a coin. Find the probability player 2 wins
if she plays optimally.
"""

from __future__ import annotations

from typing import List


def pow_int(base: int, exp: int) -> int:
    """Return base^exp as integer."""
    return base**exp


def solve() -> float:
    """Solve Problem 232."""
    N = 100

    # dp[score1][score2] = probability player 2 wins
    dp: List[List[float]] = [[0.0] * (2 * N) for _ in range(N + 1)]

    # Base cases: player 2 wins if score2 >= N
    for score1 in range(N + 1):
        for score2 in range(N, 2 * N):
            dp[score1][score2] = 1.0

    # Fill DP table backwards
    for score1 in range(N - 1, -1, -1):
        for score2 in range(N - 1, -1, -1):
            for T in range(1, 100):
                points = pow_int(2, T - 1)
                if score2 + points >= 2 * N:
                    break

                prob = (
                    dp[score1 + 1][score2 + points]
                    + dp[score1][score2 + points]
                    + (pow_int(2, T) - 1) * dp[score1 + 1][score2]
                ) / (pow_int(2, T) + 1)

                if prob > dp[score1][score2]:
                    dp[score1][score2] = prob

    # Player 1's initial turn
    ans = (dp[0][0] + dp[1][0]) / 2
    return ans


def main() -> float:
    """Main entry point."""
    result = solve()
    print(f"{result:.8f}")
    return result


if __name__ == "__main__":
    main()

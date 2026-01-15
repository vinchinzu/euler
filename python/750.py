"""Project Euler Problem 750: Optimal Card Stacking.

Let N cards be arranged in a row such that the nth card is 3^{n+1} (mod N+1).
Find the minimum sum of the distances to move the cards such that a card is
always placed on top of the card above it, and we end with a single stack of
all cards from 1 to N.

We use dynamic programming, where dp[s][e] is the minimum sum of distances
to bring the cards from s to e (inclusive) together. Then it is easy to
compute dp[s][e] by trying all possible splits where the last two stacks
are brought together. The answer is dp[1][N].
"""

from __future__ import annotations


def pow_mod(base: int, exp: int, mod: int) -> int:
    """Fast exponentiation modulo mod."""
    result = 1
    base = base % mod
    while exp > 0:
        if exp & 1:
            result = (result * base) % mod
        base = (base * base) % mod
        exp >>= 1
    return result


def solve() -> int:
    """Solve Problem 750."""
    n = 976
    pos = [0] * n
    for i in range(n):
        card_val = pow_mod(3, i + 1, n + 1) - 1
        pos[card_val] = i

    dp = [[0] * (n + 1) for _ in range(n)]

    for length in range(2, n + 1):
        for start in range(n - length + 1):
            end = start + length
            dp[start][end] = float("inf")
            for mid in range(start + 1, end):
                cost = (
                    abs(pos[mid - 1] - pos[end - 1])
                    + dp[start][mid]
                    + dp[mid][end]
                )
                if cost < dp[start][end]:
                    dp[start][end] = cost

    return int(dp[0][n])


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()

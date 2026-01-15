"""Project Euler Problem 649: Low-Prime Chessboard Nim.

In Low-Prime Chessboard Nim, a configuration consists of c coins on an n by n
chessboard, and a move consists of moving a coin either left or up either 2,
3, 5, or 7 squares. Find the number of winning configurations with C coins on
an N by N chessboard.

The Nim value of a single coin on a square is periodic for all L by L boards,
with a maximum Nim value of H, so we can compute how many squares have any
particular Nim value from 0 to H.
"""

from __future__ import annotations


def ceil(n: int, d: int) -> int:
    """Ceiling division."""
    return (n + d - 1) // d


def pow_mod(base: int, exp: int, mod: int) -> int:
    """Modular exponentiation."""
    result = 1
    base %= mod
    while exp > 0:
        if exp & 1:
            result = (result * base) % mod
        base = (base * base) % mod
        exp >>= 1
    return result


def solve() -> int:
    """Solve Problem 649."""
    N = 10_000_019
    C = 100
    M = 10**9
    L = 9
    H = 8

    # Compute grid nimbers
    grid = [[0] * L for _ in range(L)]
    for i in range(L):
        for j in range(L):
            used = [False] * H
            for d in [2, 3, 5, 7]:
                if i >= d:
                    used[grid[i - d][j]] = True
                if j >= d:
                    used[grid[i][j - d]] = True

            nimber = 0
            while nimber < H and used[nimber]:
                nimber += 1
            grid[i][j] = nimber

    # Count squares with each nimber value
    counts = [0] * H
    for i in range(L):
        for j in range(L):
            count_i = ceil(N - i, L)
            count_j = ceil(N - j, L)
            counts[grid[i][j]] = (
                counts[grid[i][j]] + count_i * count_j
            ) % M

    # DP: dp[c][total] = number of ways with c coins and nimber total
    dp = [[0] * H for _ in range(C + 1)]
    dp[0][0] = 1

    for c in range(1, C + 1):
        for total in range(H):
            for curr in range(H):
                dp[c][total] = (
                    dp[c][total] + dp[c - 1][total ^ curr] * counts[curr]
                ) % M

    ans = 0
    for curr in range(1, H):
        ans = (ans + dp[C][curr]) % M
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()

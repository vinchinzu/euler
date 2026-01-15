"""Project Euler Problem 629: Scatterstone Nim.

In Scatterstone Nim, a move consists of breaking a pile with at least 2
stones into at most k piles. Find sum_{k=2}^N f(N, k), where f(N, k) is the
number of winning positions in a game with N stones and parameter k.

For k=2 and k=3 we compute the nimbers directly; for k≥4 the nimber for a
pile of n stones is always n-1.
"""

from __future__ import annotations

from functools import lru_cache


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
    """Solve Problem 629."""
    N = 200
    M = 10**9 + 7
    L = 1 << 20  # Large enough for nimber values

    @lru_cache(maxsize=None)
    def compute_nimbers(k: int) -> tuple:
        """Compute nimbers for given k."""
        nimbers = [0] * (N + 1)

        for n in range(N + 1):
            used = set()
            for i in range(1, n):
                used.add(nimbers[i] ^ nimbers[n - i])

            if k == 3:
                for i in range(1, n):
                    for j in range(i, n - i):
                        used.add(
                            nimbers[i] ^ nimbers[j] ^ nimbers[n - i - j]
                        )

            if k >= 4:
                for i in range(n - 1):
                    used.add(i)

            mex = 0
            while mex in used:
                mex += 1
            nimbers[n] = mex

        return tuple(nimbers)

    def f(k: int) -> int:
        """Compute f(N, k)."""
        nimbers = compute_nimbers(k)

        # DP: dp[a][b][c] = number of positions with a stones, max pile b,
        # nimber c
        dp = [[[0] * L for _ in range(N + 1)] for _ in range(N + 1)]
        dp[0][N][0] = 1

        for a in range(1, N + 1):
            for b in range(1, N + 1):
                for c in range(L):
                    for d in range(1, min(a, b) + 1):
                        if c ^ nimbers[d] < L:
                            dp[a][b][c] = (
                                dp[a][b][c]
                                + dp[a - d][d][c ^ nimbers[d]]
                            ) % M

        count = 0
        for c in range(1, L):
            count = (count + dp[N][N][c]) % M
        return count

    ans = (f(2) + f(3) + (N - 3) * f(4)) % M
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()

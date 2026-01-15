"""Project Euler Problem 250: 250250.

Find the number of nonempty subsets of {1^1, 2^2, ... N^N} whose sum is
divisible by K.
"""

from __future__ import annotations

from typing import List


def pow_mod(base: int, exp: int, mod: int) -> int:
    """Return base^exp mod mod."""
    result = 1
    base %= mod
    while exp > 0:
        if exp % 2 == 1:
            result = (result * base) % mod
        base = (base * base) % mod
        exp //= 2
    return result


def solve() -> int:
    """Solve Problem 250."""
    N = 250250
    K = 250
    M = 10**16

    dp: List[int] = [0] * K
    dp[0] = 1

    for k in range(1, N + 1):
        val = pow_mod(k, k, K)
        new_dp = dp.copy()

        for i in range(K):
            new_i = (i + val) % K
            new_dp[new_i] = (dp[i] + new_dp[new_i]) % M

        dp = new_dp

    return (dp[0] - 1) % M


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()

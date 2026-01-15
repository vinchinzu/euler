"""Project Euler Problem 217: Balanced Numbers.

A k-digit number is balanced if the sum of its first ⌈k/2⌉ digits is equal
to the sum of its last ⌈k/2⌉ digits. Find the sum of all balanced numbers
less than 10^N (mod M).
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
    """Solve Problem 217."""
    N = 47
    M = 3**15
    B = 10

    # dp[i][j] = number of ways for i digits to sum to j
    max_sum = B * (N // 2 + 1)
    dp: List[List[int]] = [[0] * max_sum for _ in range(N // 2 + 1)]
    dp[0][0] = 1

    for i in range(1, N // 2 + 1):
        for j in range(max_sum):
            for d in range(min(B, j + 1)):
                dp[i][j] = (dp[i][j] + dp[i - 1][j - d]) % M

    ans = 0
    for k in range(1, N + 1):
        for i in range(k):
            for d in range(1, B):
                mult = pow_mod(B, i, M) * d % M
                if k % 2 == 1 and i != k // 2:
                    mult = mult * B % M

                for S in range(max_sum):
                    if i < k // 2:
                        term = (
                            (dp[k // 2][S] - dp[k // 2 - 1][S])
                            * (dp[k // 2 - 1][S - d] if S >= d else 0)
                        )
                    elif k % 2 == 1 and i == k // 2:
                        term = (dp[k // 2][S] - (dp[k // 2 - 1][S] if k >= 2 else 0)) * dp[
                            k // 2
                        ][S]
                    elif i < k - 1:
                        term = (
                            (dp[k // 2 - 1][S - d] if S >= d else 0)
                            - (dp[k // 2 - 2][S - d] if k >= 4 and S >= d else 0)
                        ) * dp[k // 2][S]
                    else:  # i == k - 1
                        term = (dp[k // 2 - 1][S - d] if S >= d else 0) * dp[k // 2][S]

                    ans = (ans + mult * term) % M

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()

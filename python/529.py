"""Project Euler Problem 529: 10-substring-friendly Numbers.

A number is 10-substring-friendly if every digit belongs to a substring whose
digit sum is 10. Find the number of 10-substring-friendly numbers with up to
N digits.

We can compute numbers with up to n digits for small n using DP. The state is
the set of all suffix sums up to 10, and the digit sum s of the largest
suffix of digits that isn't yet part of a substring with digit sum 10.
Given this state, we can append digits d that are at most 10 - s. The digit
sums are shifted by d, and if a suffix has digit sum 10, then we can reset
the new s to zero.

We use extrapolation to quickly compute the answer for large N.
"""

from __future__ import annotations

from typing import Callable, List


def lagrange_extrapolation(
    f: Callable[[int], int], n_points: int, mod: int
) -> Callable[[int], int]:
    """Extrapolate function using Lagrange interpolation."""
    values = []
    for i in range(1, n_points + 1):
        values.append(f(i) % mod)

    def interpolate(x: int) -> int:
        """Interpolate at point x."""
        result = 0
        for i in range(n_points):
            term = values[i]
            for j in range(n_points):
                if i != j:
                    denom = (i + 1 - (j + 1)) % mod
                    if denom == 0:
                        continue
                    inv = pow(denom, mod - 2, mod)
                    term = (term * (x - (j + 1)) * inv) % mod
            result = (result + term) % mod
        return result

    return interpolate


def solve() -> int:
    """Solve Problem 529."""
    N = 10**18
    M = 10**9 + 7
    B = 10

    dp = [[0] * B for _ in range(1 << B)]
    dp[1][0] = 1
    Ts: List[int] = [0]

    def f(n: int) -> int:
        """Compute T_n."""
        nonlocal dp, Ts
        while n >= len(Ts):
            new_dp = [[0] * B for _ in range(1 << B)]
            for suffix_sums in range(1 << B):
                for s in range(B):
                    if dp[suffix_sums][s] == 0:
                        continue
                    dp[suffix_sums][s] %= M
                    for d in range(B):
                        if d > B - s:
                            break
                        new_suffix_sums = ((suffix_sums << d) & ((1 << B) - 1)) | 1
                        new_s = (
                            0 if (suffix_sums & (1 << (B - d))) > 0 else d + s
                        )
                        new_dp[new_suffix_sums][new_s] = (
                            new_dp[new_suffix_sums][new_s] + dp[suffix_sums][s]
                        ) % M
            T = 0
            for prev_sum in range(1 << B):
                T = (T + new_dp[prev_sum][0]) % M
            Ts.append((T - 1) % M)
            dp = new_dp
        return Ts[n]

    extrap = lagrange_extrapolation(f, 3, M)
    ans = extrap(N)
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()

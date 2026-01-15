"""Project Euler Problem 654: Neighbourly Constraints.

Find T(N, K), the number of sequences of N positive integers such that the
sum of any two consecutive elements is at most K.

Let dp[j] be the number of sequences ending with j. Then newDp[j] is the sum
of all dp[k] for 1 ≤ k ≤ K - j, which is newDp[j-1] - dp[K+1-j] for j>1.
The T(n, K) is the sum of all dp[j]. This lets us compute T(n, K) for small n,
and we can extrapolate to find T(N, K).
"""

from __future__ import annotations

from typing import Callable


def mod_inverse(a: int, m: int) -> int:
    """Modular inverse using extended Euclidean algorithm."""
    if m == 1:
        return 0
    t, new_t = 0, 1
    r, new_r = m, a % m
    while new_r != 0:
        q = r // new_r
        t, new_t = new_t, t - q * new_t
        r, new_r = new_r, r - q * new_r
    if r != 1:
        raise ValueError("Modular inverse does not exist")
    if t < 0:
        t += m
    return t


def lagrange_extrapolation(
    f: Callable[[int], int], degree: int, mod: int, n: int
) -> int:
    """Extrapolate using Lagrange interpolation."""
    # For degree 2, we need at least 3 points
    # Use Newton's forward difference formula for extrapolation
    x_vals = list(range(1, 2 * degree + 1))
    y_vals = [f(x) for x in x_vals]

    # Compute finite differences
    diffs = [y_vals]
    for i in range(len(y_vals) - 1):
        new_diff = []
        for j in range(len(diffs[-1]) - 1):
            new_diff.append((diffs[-1][j + 1] - diffs[-1][j]) % mod)
        diffs.append(new_diff)

    # Extrapolate using Newton's formula
    h = 1
    s = (n - x_vals[0]) / h
    result = y_vals[0]
    fact = 1
    for i in range(1, len(diffs)):
        fact *= s - i + 1
        fact //= i
        result = (result + fact * diffs[i][0]) % mod

    return result


def solve() -> int:
    """Solve Problem 654."""
    N = 10**12
    K = 5000
    M = 10**9 + 7

    dp = [0] * K
    for i in range(1, K):
        dp[i] = 1

    T = [0] * (2 * K)
    for i in range(1, len(T)):
        T[i] = sum(dp) % M
        new_dp = [0] * K
        new_dp[1] = T[i]
        for j in range(2, K):
            new_dp[j] = (new_dp[j - 1] - dp[K + 1 - j]) % M
        dp = new_dp

    # Extrapolate T(N) from T[1..2*K]
    def T_func(n: int) -> int:
        if n < len(T):
            return T[n]
        return lagrange_extrapolation(T_func, 2, M, n)

    return T_func(N)


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()

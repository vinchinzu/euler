"""Project Euler Problem 661: A Long Chess Match.

Let E(p_A, p_B, p) be the expected number of times that player A is leading a
chess match if player A wins each game with probability p_A, player B wins
each game with probability p_B, and after each game the match ends with
probability p. Find sum_{k=3}^N E(1/√(k+3), 1/√(k+3) + 1/k², 1/k³).

We solve a tridiagonal system of equations for E(d), where d is the current
lead. We test with exponentially larger values of L until the solution for
E(0) converges.
"""

from __future__ import annotations

import math


def tridiagonal_solve(
    a: list[float], b: list[float], c: list[float], d: list[float]
) -> list[float]:
    """Solve tridiagonal system using Thomas algorithm."""
    n = len(b)
    cp = [0.0] * n
    dp = [0.0] * n
    x = [0.0] * n

    cp[0] = c[0] / b[0]
    dp[0] = d[0] / b[0]

    for i in range(1, n):
        denom = b[i] - a[i] * cp[i - 1]
        cp[i] = c[i] / denom
        dp[i] = (d[i] - a[i] * dp[i - 1]) / denom

    x[n - 1] = dp[n - 1]
    for i in range(n - 2, -1, -1):
        x[i] = dp[i] - cp[i] * x[i + 1]

    return x


def feq(a: float, b: float, eps: float = 1e-10) -> bool:
    """Check if two floats are approximately equal."""
    return abs(a - b) < eps


def E(pa: float, pb: float, p: float) -> float:
    """Compute expected number of times A leads."""
    prev_guess = float("-inf")
    L = 1
    while True:
        a = [0.0] * (2 * L + 1)
        b = [0.0] * (2 * L + 1)
        c = [0.0] * (2 * L + 1)
        d = [0.0] * (2 * L + 1)

        for diff in range(-L, L + 1):
            idx = diff + L
            a[idx] = -(1 - p) * pb
            b[idx] = 1 - (1 - p) * (1 - pa - pb)
            c[idx] = -(1 - p) * pa

            if diff >= 0:
                d[idx] += pa
            if diff >= 2:
                d[idx] += pb
            if diff >= 1:
                d[idx] += 1 - pa - pb

        guess = tridiagonal_solve(a, b, c, d)[L]
        if feq(prev_guess, guess):
            return guess
        prev_guess = guess
        L *= 2


def solve() -> float:
    """Solve Problem 661."""
    N = 50
    ans = 0.0
    for k in range(3, N + 1):
        pa = 1.0 / math.sqrt(k + 3)
        pb = pa + 1.0 / (k * k)
        p = 1.0 / (k * k * k)
        ans += E(pa, pb, p)
    return ans


def main() -> float:
    """Main entry point."""
    result = solve()
    print(f"{result:.4f}")
    return result


if __name__ == "__main__":
    main()

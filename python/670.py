"""Project Euler Problem 670: Coloured Tiles.

Find the number of ways to fill a 2 x N rectangle with 1x1, 1x2, ... 1xT
tiles such that no four tiles meet at a single point, each tile is one of K
colors, and no adjacent tiles have the same color.

We use matrix multiplication with states representing tile configurations.
"""

from __future__ import annotations

from dataclasses import dataclass
from typing import Dict

from sympy import Matrix


@dataclass(frozen=True)
class Point:
    """Integer point."""

    x: int
    y: int


def mat_pow_mod(mat: list[list[int]], exp: int, mod: int) -> list[list[int]]:
    """Matrix power modulo mod."""
    n = len(mat)
    result = [[1 if i == j else 0 for j in range(n)] for i in range(n)]
    base = [[mat[i][j] % mod for j in range(n)] for i in range(n)]

    while exp > 0:
        if exp & 1:
            result = mat_mult(result, base, mod)
        base = mat_mult(base, base, mod)
        exp >>= 1

    return result


def mat_mult(
    a: list[list[int]], b: list[list[int]], mod: int
) -> list[list[int]]:
    """Matrix multiplication modulo mod."""
    n = len(a)
    result = [[0] * n for _ in range(n)]
    for i in range(n):
        for j in range(n):
            for k in range(n):
                result[i][j] = (result[i][j] + a[i][k] * b[k][j]) % mod
    return result


def solve() -> int:
    """Solve Problem 670."""
    N = 10**16
    K = 4
    T = 3
    M = 1000004321

    # Build states
    states: list[Point] = [Point(-1, -1)]
    for i in range(T):
        for j in range(T):
            states.append(Point(i, j))

    ordering: Dict[Point, int] = {p: i for i, p in enumerate(states)}

    # Build transition matrix
    A = [[0] * len(states) for _ in range(len(states))]
    A[0][0] = K - 1
    A[0][1] = K - 2

    for i in range(T):
        for j in range(T):
            A[ordering[Point(i, j)]][0] = (K - 1) * (K - 2)

    for i in range(1, T):
        for j in range(T):
            A[ordering[Point(i - 1, j)]][ordering[Point(i, 0)]] = K - 2

    for i in range(T):
        for j in range(1, T):
            A[ordering[Point(i, j - 1)]][ordering[Point(0, j)]] = K - 2

    for i in range(1, T):
        for j in range(1, T):
            A[ordering[Point(i - 1, j - 1)]][ordering[Point(i, j)]] = 1

    # Compute A^(N-1)
    Ae = mat_pow_mod(A, N - 1, M)

    ans = 0
    for t in range(2):
        ans = (ans + K * Ae[t][0]) % M
        for i in range(T):
            for j in range(T):
                ans = (
                    ans + K * (K - 1) * Ae[t][ordering[Point(i, j)]]
                ) % M

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()

"""Project Euler Problem 299: Three similar triangles.

Find the number of integer triplets (a, b, d) with 0 < a < b, 0 < D,
A = (a, 0), B = (b, 0), C = (0, a), D = (0, d), and b + d < N, such
that there exists a lattice point P on AC so that the triangles ABP,
CDP, and BDP are similar.
"""

from __future__ import annotations

from math import gcd, isqrt


def solve() -> int:
    """Solve Problem 299."""
    N = 10**8

    # Precompute GCDs
    limit = isqrt(N // 2)
    gcds = [[0] * (i + 1) for i in range(limit + 1)]
    for i in range(1, limit + 1):
        for j in range(1, i + 1):
            gcds[i][j] = gcd(i, j)

    ans = 0

    # Case 1: ABP ≡ DBP
    for n in range(1, limit + 1):
        if f1(n, n) >= N:
            break
        for m in range(n + 1, limit + 1, 2):
            if f1(m, n) >= N:
                break
            if gcds[n][m % n] == 1:
                ans += ((N - 1) // f1(m, n)) * 2

    # Case 2: ABP ≡ BDP
    for n in range(1, limit + 1):
        if f2(n, n) >= N:
            break
        for m in range(n + 1, limit + 1, 2):
            if f2(m, n) >= N:
                break
            if gcds[n][m % n] == 1:
                ans += (N - 1) // f2(m, n)

    return ans


def f1(m: int, n: int) -> int:
    """Function for case 1."""
    return m * m - n * n + 2 * m * n


def f2(m: int, n: int) -> int:
    """Function for case 2."""
    return 2 * (m * m + n * n)


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()

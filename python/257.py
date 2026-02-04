"""Project Euler Problem 257: Angular Bisectors.

Let the angle bisectors of triangle ABC intersect the sides c ≥ b ≥ a at
E, F, and G respectively. Find the number of triangles with perimeter up
to N such that the ratio of the areas of ABC to AEG is integral.
"""

from __future__ import annotations

from math import gcd, isqrt


def solve() -> int:
    """Solve Problem 257."""
    N = 10**8
    L = isqrt(N // 3)

    # Precompute GCDs (note: gcd(m, 0) = m, needed when n % m == 0)
    gcds = [[0] * (i + 1) for i in range(L + 1)]
    for i in range(1, L + 1):
        gcds[i][0] = i
        for j in range(1, i + 1):
            gcds[i][j] = gcd(i, j)

    ans = 0

    # r = 2 case
    for m in range(1, L + 1):
        if perim1(m, m) > N:
            break
        for n in range(m + 1, 2 * m):
            if perim1(m, n) > N:
                break
            if n % 2 != 0 and gcds[m][n % m] == 1:
                ans += N // perim1(m, n)

    # r = 3 case
    for m in range(1, L + 1):
        if perim2(m, m) > N:
            break
        for n in range(m + 1, 3 * m, 2):
            if perim2(m, n) > N:
                break
            if n % 3 != 0 and gcds[m][n % m] == 1:
                ans += N // perim2(m, n)

    # r = 3 case with odd m, n
    for m in range(1, 2 * L + 1, 2):
        if perim2(m, m) > 2 * N:
            break
        for n in range(m + 2, 3 * m, 2):
            if perim2(m, n) > 2 * N:
                break
            if n % 3 != 0 and gcds[m][n % m] == 1:
                ans += 2 * N // perim2(m, n)

    # r = 4 case (equilateral triangles)
    ans += N // 3

    return ans


def perim1(m: int, n: int) -> int:
    """Perimeter for r=2 case."""
    return 2 * m * m + n * n + 3 * m * n


def perim2(m: int, n: int) -> int:
    """Perimeter for r=3 case."""
    return 3 * m * m + n * n + 4 * m * n


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()

"""Project Euler Problem 279: Triangles with Integral Angles.

Find the number of integer sided triangles with perimeter at most N and at
least one integral angle in degrees.

By the Law of Cosines, the angle must be 60°, 90°, or 120°. Pythagorean
triples are generated in the standard way, and the parameterizations for 60°
and 120° triangles are described in problems 195 and 143 respectively.

As an optimization, we precompute GCDs up to √(N/2), the upper bound for n,
and compute gcd(n, m % n) to stay within the bound. This means we need to
count equilateral triangles separately (there are ⌊N/3⌋ of them) because the
1-1-1 primitive triangle is generated from m = 1, n = 0.
"""

from __future__ import annotations

from math import gcd, isqrt


def ipow(base: int, exp: int) -> int:
    """Integer power."""
    return base**exp


def sq(n: int) -> int:
    """Square of n."""
    return n * n


def solve() -> int:
    """Solve Problem 279."""
    N = ipow(10, 8)

    # Precompute GCDs
    limit = isqrt(N // 2)
    gcds: list[list[int]] = [[0] * (i + 1) for i in range(limit + 1)]
    for i in range(1, limit + 1):
        for j in range(1, i + 1):
            gcds[i][j] = gcd(i, j)

    ans = N // 3  # Equilateral triangles

    # 60° triangles: parameterization from problem 143
    for n in range(1, limit + 1):
        if 6 * sq(n) > 3 * N:
            break
        for m in range(2 * n + 1, limit + 1):
            if 3 * m * (m - n) > 3 * N:
                break
            if gcds[n][m % n] == 1:
                factor = 3 if (m + n) % 3 != 0 else 1
                ans += N // (factor * m * (m - n))

    # 90° triangles: Pythagorean triples
    for n in range(1, limit + 1):
        if 4 * sq(n) > N:
            break
        for m in range(n + 1, limit + 1, 2):
            if 2 * m * (m + n) > N:
                break
            if gcds[n][m % n] == 1:
                ans += N // (2 * m * (m + n))

    # 120° triangles: parameterization from problem 195
    for n in range(1, limit + 1):
        if 2 * sq(n) > N:
            break
        for m in range(n + 1, 2 * n):
            if m * (m + n) > N:
                break
            if (m + n) % 3 != 0 and gcds[n][m % n] == 1:
                ans += N // (m * (m + n))

    return ans


def main() -> None:
    """Main entry point."""
    result = solve()
    print(result)


if __name__ == "__main__":
    main()

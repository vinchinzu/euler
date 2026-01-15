"""Project Euler Problem 504: Square on the Inside.

Find the number of quadrilaterals with lattice points on (a, 0), (0, b),
(-c, 0), (0, -d), where 1 ≤ a,b,c,d ≤ N, that strictly contain a square
number of lattice points.
"""

from __future__ import annotations

from math import gcd, isqrt
from typing import List


def sq(n: int) -> int:
    """Square."""
    return n * n


def solve() -> int:
    """Solve Problem 504."""
    N = 100

    # Precompute f(a,b) = number of lattice points in triangle
    f: List[List[int]] = [[0] * (N + 1) for _ in range(N + 1)]
    for a in range(1, N + 1):
        for b in range(1, N + 1):
            f[a][b] = ((a + 1) * (b + 1) - gcd(a, b) - 1) // 2 - a

    # Precompute perfect squares
    max_val = 2 * sq(N)
    is_sq = [False] * (max_val + 1)
    i = 1
    while sq(i) <= max_val:
        is_sq[sq(i)] = True
        i += 1

    ans = 0
    for a in range(1, N + 1):
        for b in range(1, N + 1):
            for c in range(1, N + 1):
                for d in range(1, N + 1):
                    total = f[a][b] + f[b][c] + f[c][d] + f[d][a] + 1
                    if total < len(is_sq) and is_sq[total]:
                        ans += 1

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()

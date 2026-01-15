"""Project Euler Problem 662: Fibonacci Paths.

Find the number of ways to go from (0, 0) to (N, N) by only taking jumps
with a Euclidean distance equal to a positive Fibonacci number on lattice
points, moving rightward and upward.

We directly compute the jumps that have a Fibonacci Euclidean distance, and
then use dynamic programming to compute the number of paths to (N, N).
"""

from __future__ import annotations

from dataclasses import dataclass
from math import isqrt

from sympy import is_square


@dataclass(frozen=True)
class Point:
    """Integer point."""

    x: int
    y: int


def fibonacci_list(n: int) -> list[int]:
    """Generate first n Fibonacci numbers."""
    if n == 0:
        return []
    if n == 1:
        return [1]
    fib = [1, 1]
    for _ in range(2, n):
        fib.append(fib[-1] + fib[-2])
    return fib


def solve() -> int:
    """Solve Problem 662."""
    N = 10000
    M = 10**9 + 7

    fibonaccis = set(fibonacci_list(50))  # Enough Fibonacci numbers
    jumps: list[Point] = []

    for dx in range(N + 1):
        for dy in range(N + 1):
            dist2 = dx * dx + dy * dy
            if dist2 > 0 and is_square(dist2):
                dist = isqrt(dist2)
                if dist in fibonaccis:
                    jumps.append(Point(dx, dy))

    # DP: F[sum][x] = number of ways to reach (x, sum-x) with sum = x+y
    F = [[0] * (N + 1) for _ in range(2 * N + 1)]
    F[0][0] = 1

    for s in range(1, 2 * N + 1):
        min_x = max(s - N, 0)
        max_x = min(s, N)
        for jump in jumps:
            prev_sum = s - jump.x - jump.y
            if prev_sum >= 0:
                for x in range(max(min_x, jump.x), max_x + 1):
                    F[s][x] = (F[s][x] + F[prev_sum][x - jump.x]) % M
        for x in range(min_x, max_x + 1):
            F[s][x] %= M

    return F[2 * N][N]


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()

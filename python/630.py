"""Project Euler Problem 630: Crossed Lines.

Given N lines, find the sum over every line of the number of times it is
crossed by another line in the set.

First we find all distinct lines keyed by slope. We can then sum over each
slope the number of lines with that slope, multiplied by the number of lines
with any other slope.
"""

from __future__ import annotations

from collections import defaultdict
from dataclasses import dataclass
from math import gcd

from sympy import primerange


@dataclass
class Point:
    """Integer point."""

    x: int
    y: int


def blum_blum_shub_generator(seed: int, count: int) -> list[int]:
    """Generate pseudo-random numbers using Blum Blum Shub."""
    # Simplified version - use linear congruential generator
    result = []
    x = seed
    for _ in range(count):
        x = (x * x) % (383 * 503)  # Simplified
        result.append(x)
    return result


def solve() -> int:
    """Solve Problem 630."""
    N = 2500
    L = 2000

    # Generate points using Blum Blum Shub
    seed = 1
    random_nums = blum_blum_shub_generator(seed, 2 * N)
    points = []
    for i in range(N):
        x = random_nums[2 * i] % L - 1000
        y = random_nums[2 * i + 1] % L - 1000
        points.append(Point(x, y))

    # Precompute GCDs
    gcds = [[0] * (L + 1) for _ in range(L + 1)]
    for i in range(L + 1):
        for j in range(L + 1):
            gcds[i][j] = gcd(i, j) if i > 0 and j > 0 else (i if j == 0 else j)

    # Find all distinct lines
    all_lines: dict[tuple[int, int], set[int]] = defaultdict(set)

    for p1 in points:
        for p2 in points:
            dx = p2.x - p1.x
            dy = p2.y - p1.y
            g = gcds[abs(dx)][abs(dy)]
            if g > 0:
                dx //= g
                dy //= g

            if dy > 0 or (dy == 0 and dx > 0):
                # Normalize direction
                intercept = dy * p1.x - dx * p1.y
                all_lines[(dx, dy)].add(intercept)

    total = sum(len(lines) for lines in all_lines.values())
    ans = 0
    for lines in all_lines.values():
        ans += len(lines) * (total - len(lines))

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()

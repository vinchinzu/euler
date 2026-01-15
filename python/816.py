"""Project Euler Problem 816: Shortest distance among points.

Find the shortest distance between any two distinct points of the given
points.
"""

from __future__ import annotations

from dataclasses import dataclass
from math import sqrt
from typing import List


@dataclass
class Point:
    """2D point."""

    x: int
    y: int

    def distance_to(self, other: Point) -> float:
        """Compute Euclidean distance to another point."""
        dx = self.x - other.x
        dy = self.y - other.y
        return sqrt(dx * dx + dy * dy)


def blum_blum_shub(seed: int, n: int) -> List[int]:
    """Generate Blum Blum Shub sequence."""
    m = 2**32
    x = seed
    result = []
    for _ in range(n):
        x = (x * x) % m
        result.append(x)
    return result


def solve() -> float:
    """Solve Problem 816."""
    N = 2_000_000
    seed = 0

    # Generate points using Blum Blum Shub
    seq = blum_blum_shub(seed, 2 * N)
    points = [Point(seq[i], seq[i + 1]) for i in range(0, 2 * N, 2)]

    # Sort by x-coordinate
    points.sort(key=lambda p: p.x)

    ans = float("inf")
    for i in range(N):
        for j in range(i + 1, N):
            p = points[i]
            q = points[j]
            if q.x - p.x > ans:
                break
            ans = min(ans, p.distance_to(q))

    return ans


def main() -> float:
    """Main entry point."""
    result = solve()
    print(f"{result:.9f}")
    return result


if __name__ == "__main__":
    main()

"""Project Euler Problem 292: Pythagorean Polygons.

Find the number of polygons with integer coordinates, integer length edges, and
perimeter at most N.
"""

from __future__ import annotations

from collections import defaultdict
from dataclasses import dataclass
from math import atan2, isqrt
from typing import Dict, List


@dataclass(frozen=True)
class Edge:
    """Edge representation."""

    x: int
    y: int
    norm: int


def sq(n: int) -> int:
    """Square of n."""
    return n * n


def is_square(n: int) -> bool:
    """Check if n is a perfect square."""
    if n < 0:
        return False
    root = isqrt(n)
    return root * root == n


def solve() -> int:
    """Solve Problem 292."""
    N = 120
    L = N // 2

    # Build edges grouped by angle
    edges_by_angle: Dict[float, List[Edge]] = defaultdict(list)

    for x in range(-L, L + 1):
        for y in range(-L, L + 1):
            norm2 = x * x + y * y
            if norm2 > 0 and norm2 < sq(L) and is_square(norm2):
                angle = atan2(y, x)
                edges_by_angle[angle].append(Edge(x, y, isqrt(norm2)))

    angles = sorted(edges_by_angle.keys())

    # DP table: table[x][y][d] = number of paths ending at (x,y) with
    # perimeter d
    table = [[[0] * (N + 1) for _ in range(N)] for _ in range(N)]
    table[L][L][0] = 1

    for angle in angles:
        new_table = [[row[:] for row in col] for col in table]
        for edge in edges_by_angle[angle]:
            for x in range(-L + 1, L):
                for y in range(-L + 1, L):
                    if abs(x + edge.x) < L and abs(y + edge.y) < L:
                        for d in range(N + 1):
                            new_d = d + edge.norm
                            if new_d <= N:
                                new_table[x + edge.x + L][y + edge.y + L][
                                    new_d
                                ] += table[x + L][y + L][d]
        table = new_table

    ans = 0
    for d in range(N + 1):
        ans += table[L][L][d]
    ans -= len(edges_by_angle) // 2 + 1

    return ans


def main() -> None:
    """Main entry point."""
    result = solve()
    print(result)


if __name__ == "__main__":
    main()

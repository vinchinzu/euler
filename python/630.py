"""Project Euler Problem 630: Crossed Lines.

Given N lines, find the sum over every line of the number of times it is
crossed by another line in the set.

First we find all distinct lines keyed by slope. We can then sum over each
slope the number of lines with that slope, multiplied by the number of lines
with any other slope.
"""

from __future__ import annotations

from collections import defaultdict
from math import gcd


def blum_blum_shub_points(seed, count, L):
    """Generate points using Blum Blum Shub generator.

    S_0 = 290797
    S_{n+1} = S_n^2 mod 50515093
    T_n = (S_n mod 2000) - 1000
    Points are (T_{2k-1}, T_{2k}).
    """
    s = seed
    points = []
    for _ in range(count):
        s = (s * s) % 50515093
        x = s % L - 1000
        s = (s * s) % 50515093
        y = s % L - 1000
        points.append((x, y))
    return points


def solve():
    """Solve Problem 630."""
    N = 2500
    L = 2000

    points = blum_blum_shub_points(290797, N, L)

    # Precompute GCDs
    gcds = [[0] * (L + 1) for _ in range(L + 1)]
    for i in range(L + 1):
        for j in range(L + 1):
            if i == 0 and j == 0:
                gcds[i][j] = 0
            elif i == 0:
                gcds[i][j] = j
            elif j == 0:
                gcds[i][j] = i
            else:
                gcds[i][j] = gcd(i, j)

    # Find all distinct lines keyed by slope
    all_lines = defaultdict(set)

    for p1 in points:
        for p2 in points:
            dx = p2[0] - p1[0]
            dy = p2[1] - p1[1]
            if dy > 0 or (dy == 0 and dx > 0):
                g = gcds[abs(dx)][abs(dy)]
                if g > 0:
                    dx //= g
                    dy //= g
                intercept = dy * p1[0] - dx * p1[1]
                all_lines[(dx, dy)].add(intercept)

    total = sum(len(lines) for lines in all_lines.values())
    ans = 0
    for lines in all_lines.values():
        ans += len(lines) * (total - len(lines))

    return ans


def main():
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()

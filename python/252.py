"""Project Euler Problem 252: Convex Holes.

Find the maximum area of a "convex hole" in the set of points, a subset of
points that form the vertices of a convex polygon that do not contain any of
the other points in its interior.

Using dynamic programming, we find for each triplet of points (p_A, p2, p3) the
maximum area f(p_A, p2, p3) of a convex hole with leftmost point p_A and
containing the segment (p2, p3).

Algorithm from "Searching for Empty Convex Polygons" by Dobkin, Edelsbrunner, Overmars.
"""

from __future__ import annotations

from collections import defaultdict, deque
from math import atan2


def blum_blum_shub(seed, n):
    """Generate points using Blum Blum Shub."""
    points = []
    x = seed
    for _ in range(n):
        x = (x * x) % 50515093
        x_val = x % 2000 - 1000
        x = (x * x) % 50515093
        y_val = x % 2000 - 1000
        points.append((x_val, y_val))
    return points


def turn(p1, p2, p3):
    """Compute turn direction (cross product)."""
    return (p2[0] - p1[0]) * (p3[1] - p1[1]) - (p2[1] - p1[1]) * (p3[0] - p1[0])


def shoestring(p1, p2, p3):
    """Compute signed area * 2 using shoelace/shoestring formula."""
    return abs(p1[0] * (p2[1] - p3[1]) + p2[0] * (p3[1] - p1[1]) + p3[0] * (p1[1] - p2[1]))


def solve():
    """Solve Problem 252."""
    N = 500
    points = blum_blum_shub(290797, N)
    points.sort()

    areas = {}

    for k in range(N):
        p_A = points[k]
        remaining = points[k + 1:]
        remaining.sort(key=lambda p: atan2(p[1] - p_A[1], p[0] - p_A[0]))

        n = len(remaining)
        # Map points to indices for fast lookup
        point_to_idx = {p: i for i, p in enumerate(remaining)}

        # Build visibility graph using the proceed algorithm
        # Q[i] is a deque of points visible from remaining[i]
        Q = [deque() for _ in range(n)]
        # VG: outgoing[i] = list of indices j such that (i, j) is an edge
        # VG: incoming[i] = list of indices j such that (j, i) is an edge
        outgoing = [[] for _ in range(n)]
        incoming = [[] for _ in range(n)]

        def proceed(pi_idx, pj_idx):
            pi = remaining[pi_idx]
            pj = remaining[pj_idx]
            while Q[pi_idx] and turn(remaining[Q[pi_idx][0]], pi, pj) > 0:
                pk_idx = Q[pi_idx].popleft()
                proceed(pk_idx, pj_idx)
            outgoing[pi_idx].append(pj_idx)
            incoming[pj_idx].append(pi_idx)
            Q[pj_idx].append(pi_idx)

        for i in range(n - 1):
            proceed(i, i + 1)

        # DP: for each p2, iterate over outgoing p3 in order
        for p2_idx in range(n):
            p2 = remaining[p2_idx]
            p1s = list(incoming[p2_idx])  # indices of incoming points
            max_area = 0.0
            for p3_idx in outgoing[p2_idx]:
                p3 = remaining[p3_idx]
                key = (p_A, p2, p3)
                if key not in areas:
                    while p1s and turn(remaining[p1s[0]], p2, p3) > 0:
                        p1_idx = p1s.pop(0)
                        p1 = remaining[p1_idx]
                        area = areas.get((p_A, p1, p2), 0.0)
                        if area > max_area:
                            max_area = area
                    areas[key] = max_area + shoestring(p_A, p2, p3) / 2.0

    return max(areas.values())


def main():
    """Main entry point."""
    result = solve()
    print(f"{result:.1f}")


if __name__ == "__main__":
    main()

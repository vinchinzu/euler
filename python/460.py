"""Project Euler Problem 460: An ant on the move.

Find the shortest path from (0, 1) to (N, 1) that involves only straight
lines between lattice points, and such that the velocity between two points
(x0, y0) and (x1, y1) is y0 if y0 = y1, and (y1 - y0) / (ln(y1) - ln(y0))
otherwise.
"""

from __future__ import annotations

import math
from collections import defaultdict
import heapq


def solve(d: int) -> float:
    """Compute F(d) using Dijkstra matching Java implementation exactly."""
    # Generate points near semicircle
    points = []
    for x in range(d + 1):
        for y in range(1, d + 1):
            dist = (x - d // 2) ** 2 + y * y
            if (d // 2 - 1) ** 2 <= dist <= (d // 2 + 1) ** 2:
                points.append((x, y))

    ordering = {p: i for i, p in enumerate(points)}

    # Group points by x coordinate
    points_map = defaultdict(list)
    for (px, py) in points:
        points_map[px].append(py)

    # Dijkstra
    n = len(points)
    dists = [float("inf")] * n
    dists[0] = 0.0
    visited = [False] * n

    # Min-heap with custom comparator via index-based heap (like Java)
    # Use (distance, counter, index) to ensure stable ordering
    counter = 0
    pq = [(0.0, counter, 0)]

    while pq:
        current_dist, _, i = heapq.heappop(pq)

        if visited[i]:
            continue
        visited[i] = True

        px, py = points[i]

        if px == d and py == 1:
            return current_dist

        for dx in range(d - px + 1):
            done = True
            for y in points_map[px + dx]:
                pt = (px + dx, y)
                j = ordering.get(pt)
                if j is None:
                    continue
                if not visited[j]:
                    # Calculate edge distance
                    if py == y:
                        vel = float(py)
                    else:
                        vel = (y - py) / (math.log(y) - math.log(py))

                    dist_sq = dx * dx + (y - py) ** 2
                    edge_dist = math.sqrt(dist_sq) / vel
                    new_dist = current_dist + edge_dist

                    if new_dist < dists[j]:
                        dists[j] = new_dist
                        counter += 1
                        heapq.heappush(pq, (new_dist, counter, j))

                if dx * dx + (y - py) ** 2 < d:
                    done = False

            if done:
                break

    return float("inf")


if __name__ == "__main__":
    N = 10000
    result = solve(N)
    print(f"{result:.9f}")

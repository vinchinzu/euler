"""Project Euler Problem 353: Minimal risk paths on a spherical grid.

This module provides functionality to compute M(r), the minimal risk of a
journey from the North Pole station (0, 0, r) to the South Pole station
(0, 0, -r) along great-circle arcs between integer-coordinate stations on
sphere C(r).

The risk of a road (direct journey between two stations) with geodesic length
``d`` on a sphere of radius ``r`` is defined as ``(d / (pi * r)) ** 2``.
The risk of a journey using multiple roads is the sum of risks of its roads.

The core public API is:
- compute_m_r(r): compute the minimal risk M(r) for a given radius r.
- main(): compute sum(M(2**n - 1) for n in 1..15) and print diagnostic output.

The implementation is self-contained and uses only Python's standard library.
"""

from __future__ import annotations

import math
from typing import Iterable, List, Sequence, Tuple

Point3D = Tuple[int, int, int]


def enumerate_stations(r: int) -> List[Point3D]:
    """Return all integer-coordinate stations on the sphere of radius r.

    Each station is an (x, y, z) tuple of integers satisfying x^2 + y^2 + z^2 = r^2.
    For any valid (x, y), both z and -z (if distinct) are included.
    """

    r_squared = r * r
    points: List[Point3D] = []

    for x in range(-r, r + 1):
        x_squared = x * x
        if x_squared > r_squared:
            continue

        for y in range(-r, r + 1):
            y_squared = y * y
            z_squared = r_squared - x_squared - y_squared
            if z_squared < 0:
                continue

            z_float = math.isqrt(z_squared)
            if z_float * z_float != z_squared:
                continue

            z = int(z_float)
            points.append((x, y, z))
            if z != 0:
                points.append((x, y, -z))

    if not points:
        msg = f"No points found for r={r}"
        raise ValueError(msg)

    return points


def geodesic_distance(p1: Point3D, p2: Point3D, r: int) -> float:
    """Return great-circle distance between two stations on sphere of radius r.

    Uses acos of the clamped dot product to obtain the central angle.
    """

    r2 = r * r
    dot = p1[0] * p2[0] + p1[1] * p2[1] + p1[2] * p2[2]

    # Clamp to handle minor floating point issues (though inputs are integral).
    cos_theta = max(min(dot / float(r2), 1.0), -1.0)
    theta = math.acos(cos_theta)
    return r * theta


def road_risk(i: int, j: int, points: Sequence[Point3D], r: int) -> float:
    """Return the risk of the road between stations i and j.

    The risk is (d / (pi * r)) ** 2, where d is geodesic_distance.
    """

    d = geodesic_distance(points[i], points[j], r)
    return (d / (math.pi * r)) ** 2


def compute_m_r(r: int) -> float:
    """Compute M(r), the minimal risk from North to South Pole stations.

    This uses Dijkstra's algorithm on the complete graph of stations where each
    edge weight is the road risk. For the modest r values used in the original
    problem (2**n - 1, n <= 15), this simple implementation is sufficient.

    Note: For larger r, building the full dense graph is infeasible and a more
    sophisticated approach (e.g., geometric pruning or on-demand neighbor
    generation) would be needed.
    """

    points = enumerate_stations(r)

    try:
        north_idx = points.index((0, 0, r))
    except ValueError as exc:  # pragma: no cover - indicates invalid input
        msg = f"North Pole not found for r={r}"
        raise ValueError(msg) from exc

    try:
        south_idx = points.index((0, 0, -r))
    except ValueError as exc:  # pragma: no cover - indicates invalid input
        msg = f"South Pole not found for r={r}"
        raise ValueError(msg) from exc

    n = len(points)
    inf = float("inf")

    # Dijkstra's algorithm on an implicit dense graph.
    distances: List[float] = [inf] * n
    visited: List[bool] = [False] * n
    distances[north_idx] = 0.0

    for _ in range(n):
        # Find unvisited node with smallest known distance.
        u = -1
        min_dist = inf
        for idx in range(n):
            if not visited[idx] and distances[idx] < min_dist:
                min_dist = distances[idx]
                u = idx

        if u == -1:
            break

        if u == south_idx:
            break

        visited[u] = True

        # Relax edges from u to every other station.
        for v in range(n):
            if visited[v] or v == u:
                continue
            risk = road_risk(u, v, points, r)
            alt = distances[u] + risk
            if alt < distances[v]:
                distances[v] = alt

    result = distances[south_idx]
    if math.isinf(result):  # pragma: no cover - indicates disconnected graph
        msg = f"No path found from North to South for r={r}"
        raise RuntimeError(msg)

    return result


def compute_sum_m_r_powers_of_two(limit_exponent: int = 15) -> float:
    """Compute sum of M(2**n - 1) for n from 1 to limit_exponent inclusive."""

    total = 0.0
    for k in range(1, limit_exponent + 1):
        r = (1 << k) - 1
        total += compute_m_r(r)
    return total


def main() -> None:
    """Entry point: compute and print example values and the final sum.

    Matches the behavior of the original Ruby script while remaining concise.
    """

    print("Computing M(r) for r = 2^n - 1 where n = 1 to 15...")
    r_values = [(1 << k) - 1 for k in range(1, 15 + 1)]
    print("r values:", ", ".join(str(r) for r in r_values))

    total = 0.0
    for k in range(1, 15 + 1):
        r = (1 << k) - 1
        m_r = compute_m_r(r)
        total += m_r

        if k <= 3:
            print(f"M({r}) = {m_r:.10f}")

    print(f"Sum M(2^n - 1), n=1..15: {total:.10f}")


if __name__ == "__main__":  # pragma: no cover - manual execution only
    main()

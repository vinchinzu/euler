"""Project Euler Problem 332 – Spherical triangles on integer lattice points.

A spherical triangle is a figure formed on the surface of a sphere by three
great circular arcs intersecting pairwise in three vertices.

Let C(r) be the sphere with centre (0, 0, 0) and radius r.
Let Z(r) be the set of points on the surface of C(r) with integer coordinates.
Let T(r) be the set of spherical triangles with vertices in Z(r).
Degenerate spherical triangles, formed by three points on the same great arc,
are not included in T(r).

Let A(r) be the area of the smallest spherical triangle in T(r).
For example A(14) is 3.294040 (rounded to six decimal places).

Find sum_{r=1}^{50} A(r). Give your answer rounded to six decimal places.
"""

from __future__ import annotations

from math import atan2, isqrt
from typing import List, Sequence, Tuple


Point3D = Tuple[int, int, int]

# Increasing caps used for neighbour pruning; the final pass always considers
# all remaining neighbours to guarantee correctness.
NEIGHBOUR_CAPS: Tuple[int, ...] = (32, 64, 96, 128)


def generate_lattice_points(r: int) -> List[Point3D]:
    """Return integer lattice points on the sphere x^2 + y^2 + z^2 = r^2."""

    if r <= 0:
        return []

    r_sq = r * r
    points: List[Point3D] = []

    for x in range(-r, r + 1):
        for y in range(-r, r + 1):
            z_sq = r_sq - x * x - y * y
            if z_sq < 0:
                continue

            z = isqrt(z_sq)
            if z * z != z_sq:
                continue

            points.append((x, y, z))
            if z != 0:
                points.append((x, y, -z))

    # Preserve ordering while dropping duplicates.
    return list(dict.fromkeys(points))


def _pairwise_dots(points: Sequence[Point3D]) -> List[List[int]]:
    """Return dot products p_i · p_j for all point pairs."""

    n = len(points)
    dots: List[List[int]] = [[0] * n for _ in range(n)]

    for i in range(n):
        x1, y1, z1 = points[i]
        for j in range(i + 1, n):
            x2, y2, z2 = points[j]
            value = x1 * x2 + y1 * y2 + z1 * z2
            dots[i][j] = value
            dots[j][i] = value

    return dots


def _sorted_neighbours(dots: Sequence[Sequence[int]]) -> List[List[int]]:
    """Return all neighbours ordered by descending dot product."""

    n = len(dots)
    neighbours: List[List[int]] = []

    for i in range(n):
        row = dots[i]
        ordering = sorted(
            (j for j in range(n) if j != i),
            key=row.__getitem__,
            reverse=True,
        )
        neighbours.append(ordering)

    return neighbours


def _search_min_area(
    points: Sequence[Point3D],
    dots: Sequence[Sequence[int]],
    neighbours: Sequence[Sequence[int]],
    r: int,
    cap: int,
) -> float:
    """Return the smallest spherical triangle area using first `cap` neighbours."""

    best = float("inf")
    r_sq = r * r

    for i, ordering in enumerate(neighbours):
        px, py, pz = points[i]
        limit = min(cap, len(ordering))
        if limit < 2:
            continue

        for idx_a in range(limit):
            j = ordering[idx_a]
            if j <= i:
                continue

            qx, qy, qz = points[j]
            cx = py * qz - pz * qy
            cy = pz * qx - px * qz
            cz = px * qy - py * qx
            if cx == 0 and cy == 0 and cz == 0:
                continue

            dot_ij = dots[i][j]

            for idx_b in range(idx_a + 1, limit):
                k = ordering[idx_b]
                if k <= j:
                    continue

                rx, ry, rz = points[k]
                det = cx * rx + cy * ry + cz * rz
                if det == 0:
                    continue
                if det < 0:
                    det = -det

                dot_jk = dots[j][k]
                dot_ki = dots[k][i]
                denom = r * (r_sq + dot_ij + dot_jk + dot_ki)
                if denom <= 0:
                    continue

                area = 2.0 * atan2(det, denom)
                if area < best:
                    best = area

    return best if best != float("inf") else 0.0


def compute_A(r: int) -> float:
    """Return A(r): minimal spherical triangle area on the radius-r sphere."""

    points = generate_lattice_points(r)
    if len(points) < 3:
        return 0.0

    dots = _pairwise_dots(points)
    neighbours = _sorted_neighbours(dots)

    caps: List[int] = []
    max_index = len(points) - 1
    for candidate in NEIGHBOUR_CAPS:
        value = min(candidate, max_index)
        if value > 0:
            caps.append(value)
    if not caps or caps[-1] != max_index:
        caps.append(max_index)

    previous = None
    for cap in caps:
        area = _search_min_area(points, dots, neighbours, r, cap)
        if area == 0.0:
            return 0.0
        if previous is not None and abs(area - previous) <= 1e-12:
            return area
        previous = area

    return previous if previous is not None else 0.0


def solve(limit: int = 50) -> float:
    """Compute sum_{r=1}^{limit} A(r)."""

    total = 0.0
    for r in range(1, limit + 1):
        total += compute_A(r)
    return total


def main() -> None:
    """Run the Project Euler 332 solver."""

    result = solve()
    print(f"{result:.6f}")


if __name__ == "__main__":  # pragma: no cover
    main()

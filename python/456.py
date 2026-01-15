"""Project Euler Problem 456: Triangles containing origin.

Find the number of triangles containing the origin and with vertices among the
given points.
"""

from __future__ import annotations

from collections import defaultdict
from math import atan2
from typing import List, Tuple


class Point:
    """2D point."""

    def __init__(self, x: int, y: int) -> None:
        """Initialize point."""
        self.x = x
        self.y = y

    def cross(self, other: Point) -> int:
        """Cross product."""
        return self.x * other.y - self.y * other.x

    def __eq__(self, other: object) -> bool:
        """Equality."""
        if not isinstance(other, Point):
            return False
        return self.x == other.x and self.y == other.y

    def __hash__(self) -> int:
        """Hash."""
        return hash((self.x, self.y))


def nCr(n: int, r: int) -> int:
    """Binomial coefficient."""
    if r < 0 or r > n:
        return 0
    if r == 0 or r == n:
        return 1
    result = 1
    for i in range(min(r, n - r)):
        result = result * (n - i) // (i + 1)
    return result


def sq(n: int) -> int:
    """Square."""
    return n * n


def solve() -> int:
    """Solve Problem 456."""
    N = 2_000_000

    # Generate points
    points_list = []
    n1, n2 = 1, 1
    for _ in range(N):
        n1 = (n1 * 1248) % 32323
        n2 = (n2 * 8421) % 30103
        points_list.append(Point(n1 - 16161, n2 - 15051))

    # Group points by angle
    points_by_angle = defaultdict(list)
    for p in points_list:
        angle = atan2(p.y, p.x)
        points_by_angle[angle].append(p)

    # Create rays sorted by angle
    rays = sorted(points_by_angle.items())

    # Window for sliding
    window: List[Point] = []
    start = 0
    end = 0

    # Initialize window with points from rays[1:]
    for i in range(1, len(rays)):
        for p in rays[i][1]:
            window.append(p)
            end += 1

    ans = nCr(N, 3)

    for angle, ray_points in rays:
        # Remove points that are too far clockwise
        while start < end and ray_points[0].cross(window[start]) > 0:
            start += 1

        # Subtract triangles ending on this ray
        ans -= nCr(end - start, 2) * len(ray_points)

        # Handle points exactly on the boundary
        while start < end and ray_points[0].cross(window[start]) == 0:
            start += 1

        # Subtract triangles with two points on this ray
        ans -= (end - start) * nCr(len(ray_points), 2)

        # Subtract triangles with all three points on this ray
        ans -= nCr(len(ray_points), 3)

        # Add points from this ray to window
        for p in ray_points:
            window.append(p)
            end += 1

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()

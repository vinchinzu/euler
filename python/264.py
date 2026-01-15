"""Project Euler Problem 264: Triangle Centres.

Find the sum of the perimeters of all triangles with lattice point vertices,
circumcenter at the origin, orthocenter at (5, 0), and perimeter at most N.
"""

from __future__ import annotations

from dataclasses import dataclass
from math import hypot, isqrt
from typing import Dict, Set


@dataclass(frozen=True)
class Point:
    """2D integer point."""

    x: int
    y: int

    def reflect_y(self) -> "Point":
        """Reflect across x-axis."""
        return Point(self.x, -self.y)

    def distance_to(self, other: "Point") -> float:
        """Distance to other point."""
        return hypot(other.x - self.x, other.y - self.y)


def sq(n: int) -> int:
    """Square of n."""
    return n * n


def is_square(n: int) -> bool:
    """Check if n is a perfect square."""
    if n < 0:
        return False
    root = isqrt(n)
    return root * root == n


def shoelace(p1: Point, p2: Point, p3: Point) -> float:
    """Shoelace area."""
    return abs(
        (p1.x * (p2.y - p3.y) + p2.x * (p3.y - p1.y) + p3.x * (p1.y - p2.y))
        / 2.0
    )


def solve() -> float:
    """Solve Problem 264."""
    N = 100000
    triangles: Dict[Set[Point], float] = {}

    for Ax in range(N // 4 + 1):
        for Ay in range(Ax % 2 + 1, N // 4 + 1, 2):
            R2 = sq(Ax) + sq(Ay)
            num = 2 * sq(Ay) * R2
            den = sq(5 - Ax) + sq(Ay)
            if den != 0 and num % den == 0:
                disc = 2 * num // den - sq(Ay)
                if is_square(disc):
                    Bx = ((5 - Ax) + isqrt(disc)) // 2
                    Cx = 5 - Ax - Bx
                    By = isqrt(R2 - sq(Bx))
                    if sq(Cx) + sq(Ay + By) != R2:
                        By = -By
                    p1 = Point(Ax, Ay)
                    p2 = Point(Bx, By)
                    p3 = Point(Cx, -Ay - By)
                    perim = (
                        p1.distance_to(p2) + p2.distance_to(p3) + p3.distance_to(p1)
                    )
                    if perim <= N and shoelace(p1, p2, p3) > 0:
                        triangles[frozenset([p1, p2, p3])] = perim
                        triangles[frozenset([p1.reflect_y(), p2.reflect_y(), p3.reflect_y()])] = (
                            perim
                        )

        # Handle Ay = 0 case
        disc = 4 * sq(Ax) - sq(5 - Ax)
        if is_square(disc):
            By = isqrt(disc) // 2
            p1 = Point(Ax, 0)
            p2 = Point((5 - Ax) // 2, By)
            p3 = Point((5 - Ax) // 2, -By)
            perim = (
                p1.distance_to(p2) + p2.distance_to(p3) + p3.distance_to(p1)
            )
            if perim <= N:
                triangles[frozenset([p1, p2, p3])] = perim

    return sum(triangles.values())


def main() -> None:
    """Main entry point."""
    result = solve()
    print(f"{result:.4f}")


if __name__ == "__main__":
    main()

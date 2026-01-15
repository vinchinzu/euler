"""Project Euler Problem 607: Marsh Crossing.

Find the minimum time to get from point A to a point B that is D units away
due east, if a marsh with width L running exactly south-west to north-east is
exactly midway between A and B, and consists of K strips dividing the map
into regions where you can go at the speed given in SPEEDS.

The problem is equivalent to finding the time for a beam of light to go from
A to B, given the same relative speeds in the different regions. Given an
initial orientation angle α, it is possible to trace the beam of light
originating from A at angle α, by using Snell's Law at every border between
two regions.
"""

from __future__ import annotations

import math
from dataclasses import dataclass


@dataclass
class Point:
    """2D point with floating point coordinates."""

    x: float
    y: float

    def dist(self, other: Point) -> float:
        """Distance to another point."""
        dx = self.x - other.x
        dy = self.y - other.y
        return math.sqrt(dx * dx + dy * dy)


@dataclass
class Line:
    """Line in standard form Ax + By = C."""

    A: float
    B: float
    C: float

    @staticmethod
    def from_point_angle(p: Point, angle: float) -> Line:
        """Create line through point p at given angle."""
        A = math.sin(angle)
        B = -math.cos(angle)
        C = A * p.x + B * p.y
        return Line(A, B, C)

    def intersection(self, other: Line) -> Point:
        """Find intersection point with another line."""
        denom = self.A * other.B - self.B * other.A
        x = (self.C * other.B - self.B * other.C) / denom
        y = (self.C * other.A - self.A * other.C) / denom
        return Point(x, y)


def feq(a: float, b: float, eps: float = 1e-10) -> bool:
    """Check if two floats are approximately equal."""
    return abs(a - b) < eps


def solve() -> float:
    """Solve Problem 607."""
    D = 100
    L = 50
    K = 5
    SPEEDS = [10, 9, 8, 7, 6, 5, 10]

    border_xs = []
    for i in range(K + 1):
        border_xs.append(D / 2 - L / math.sqrt(2) + (L / K) * math.sqrt(2) * i)
    border_xs.append(float(D))

    low = 0.0
    high = math.pi / 4
    ans = 0.0

    while not feq(low, high):
        mid = (low + high) / 2
        a = mid
        pos = Point(0.0, 0.0)
        ans = 0.0

        for i in range(len(border_xs)):
            border_line = Line.from_point_angle(Point(border_xs[i], 0.0), math.pi / 4)
            ray_line = Line.from_point_angle(pos, a)
            new_pos = ray_line.intersection(border_line)
            ans += pos.dist(new_pos) / SPEEDS[i]
            pos = new_pos
            if i < len(border_xs) - 1:
                sin_val = SPEEDS[i + 1] * math.sin(math.pi / 4 + a) / SPEEDS[i]
                a = math.asin(sin_val) - math.pi / 4

        if pos.y < 0:
            low = mid
        else:
            high = mid

    return ans


def main() -> float:
    """Main entry point."""
    result = solve()
    print(f"{result:.10f}")
    return result


if __name__ == "__main__":
    main()

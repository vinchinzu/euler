#!/usr/bin/env python3
"""Project Euler Problem 395 - Pythagorean Tree

Find minimum area of rectangle parallel to initial square containing entire tree.

Solution iteratively finds extreme points in each direction.
"""

from math import sqrt

class Point:
    def __init__(self, x, y):
        self.x = x
        self.y = y

    def add(self, other):
        return Point(self.x + other.x, self.y + other.y)

    def subtract(self, other):
        return Point(self.x - other.x, self.y - other.y)

    def complex_multiply(self, other):
        """Multiply as complex numbers"""
        return Point(
            self.x * other.x - self.y * other.y,
            self.x * other.y + self.y * other.x
        )

    def dist(self, other):
        dx = self.x - other.x
        dy = self.y - other.y
        return sqrt(dx * dx + dy * dy)

def solve():
    A, B, C = 3, 4, 5
    L = sqrt(2) / (1 - B / C)

    def find_extreme(get_coord):
        """Find extreme point in direction given by get_coord function"""
        segments = [((Point(0, 0), Point(1, 0)))]
        extremity = float('-inf')

        while True:
            new_segments = []

            for start, end in segments:
                diff = end.subtract(start)

                # Four transformation points for the new squares
                transforms = [
                    Point(-A * B / (C * C), B * B / (C * C)),
                    Point((B - A) * B / (C * C), (B + A) * B / (C * C)),
                    Point((B + A) * B / (C * C), (B + A) * A / (C * C)),
                    Point(1 + A * B / (C * C), A * A / (C * C))
                ]

                new_points = [start.add(diff.complex_multiply(t)) for t in transforms]

                new_segments.append((new_points[0], new_points[1]))
                new_segments.append((new_points[2], new_points[3]))

            # Find new extreme point
            all_points = {}
            for seg in new_segments:
                all_points[get_coord(seg[0])] = seg
                all_points[get_coord(seg[1])] = seg

            new_extremity = max(all_points.keys())

            if abs(extremity - new_extremity) < 1e-12:
                break

            extremity = new_extremity

            # Prune segments that can't contribute further
            segments_set = set()
            for coord, seg in all_points.items():
                seg_len = seg[0].dist(seg[1])
                if coord > new_extremity - L * seg_len:
                    segments_set.add(seg)

            segments = list(segments_set)

        return extremity

    # Find extremes in all four directions
    min_x = -find_extreme(lambda p: -p.x)
    min_y = -find_extreme(lambda p: -p.y)
    max_x = find_extreme(lambda p: p.x)
    max_y = find_extreme(lambda p: p.y)

    area = (max_x - min_x) * (max_y - min_y)
    return f"{area:.10f}"

if __name__ == "__main__":
    print(solve())

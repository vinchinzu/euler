"""Project Euler Problem 460: An ant on the move.

Find the shortest path from (0, 1) to (N, 1) that involves only straight
lines between lattice points, and such that the velocity between two points
(x0, y0) and (x1, y1) is y0 if y0 = y1, and (y1 - y0) / (ln(y1) - ln(y0))
otherwise.
"""

from __future__ import annotations

import math
from collections import defaultdict
from typing import Dict, List, Tuple


class Point:
    """2D integer point."""

    def __init__(self, x: int, y: int) -> None:
        """Initialize point."""
        self.x = x
        self.y = y

    def __eq__(self, other: object) -> bool:
        """Equality."""
        if not isinstance(other, Point):
            return False
        return self.x == other.x and self.y == other.y

    def __hash__(self) -> int:
        """Hash."""
        return hash((self.x, self.y))


def sq(n: int) -> int:
    """Square."""
    return n * n


class Heap:
    """Min heap for Dijkstra's algorithm."""

    def __init__(self, size: int, max_val: float) -> None:
        """Initialize heap."""
        self.size = size
        self.heap = list(range(size))
        self.pos = list(range(size))
        self.keys = [max_val] * size

    def add(self, idx: int, key: float) -> None:
        """Add element."""
        self.keys[idx] = key
        self._sift_up(self.pos[idx])

    def set(self, idx: int, key: float) -> None:
        """Set key."""
        old_key = self.keys[idx]
        self.keys[idx] = key
        if key < old_key:
            self._sift_up(self.pos[idx])
        else:
            self._sift_down(self.pos[idx])

    def remove_min(self) -> int:
        """Remove minimum."""
        min_idx = self.heap[0]
        self.heap[0] = self.heap[self.size - 1]
        self.pos[self.heap[0]] = 0
        self.size -= 1
        if self.size > 0:
            self._sift_down(0)
        return min_idx

    def _sift_up(self, pos: int) -> None:
        """Sift up."""
        while pos > 0:
            parent = (pos - 1) // 2
            if self.keys[self.heap[parent]] <= self.keys[self.heap[pos]]:
                break
            self._swap(pos, parent)
            pos = parent

    def _sift_down(self, pos: int) -> None:
        """Sift down."""
        while True:
            left = 2 * pos + 1
            right = 2 * pos + 2
            smallest = pos
            if left < self.size and self.keys[self.heap[left]] < self.keys[
                self.heap[smallest]
            ]:
                smallest = left
            if right < self.size and self.keys[self.heap[right]] < self.keys[
                self.heap[smallest]
            ]:
                smallest = right
            if smallest == pos:
                break
            self._swap(pos, smallest)
            pos = smallest

    def _swap(self, i: int, j: int) -> None:
        """Swap elements."""
        self.heap[i], self.heap[j] = self.heap[j], self.heap[i]
        self.pos[self.heap[i]] = i
        self.pos[self.heap[j]] = j


def F(d: int) -> float:
    """Compute F(d)."""
    # Generate points near semicircle
    points: List[Point] = []
    for x in range(d + 1):
        for y in range(1, d + 1):
            dist = sq(x - d // 2) + sq(y)
            if sq(d // 2 - 1) <= dist <= sq(d // 2 + 1):
                points.append(Point(x, y))

    ordering: Dict[Point, int] = {p: i for i, p in enumerate(points)}

    # Group points by x coordinate
    points_map: Dict[int, List[int]] = defaultdict(list)
    for p in points:
        points_map[p.x].append(p.y)

    # Dijkstra
    dists = [float("inf")] * len(points)
    heap = Heap(len(points), float("inf"))
    heap.add(0, 0.0)
    for i in range(1, len(points)):
        heap.add(i, float("inf"))

    visited = [False] * len(points)
    while True:
        i = heap.remove_min()
        p = points[i]
        visited[i] = True
        current_dist = dists[i] if dists[i] != float("inf") else 0.0

        if p.x == d and p.y == 1:
            return current_dist

        for dx in range(d - p.x + 1):
            done = True
            for y in points_map.get(p.x + dx, []):
                j = ordering.get(Point(p.x + dx, y))
                if j is None or visited[j]:
                    continue

                # Compute distance
                if p.y == y:
                    vel = p.y
                else:
                    vel = (y - p.y) / (math.log(y) - math.log(p.y))
                edge_dist = math.sqrt(sq(dx) + sq(y - p.y)) / vel
                new_dist = current_dist + edge_dist

                if new_dist < dists[j]:
                    dists[j] = new_dist
                    heap.set(j, new_dist)

                if sq(dx) + sq(y - p.y) < d:
                    done = False

            if done:
                break


def solve() -> float:
    """Solve Problem 460."""
    N = 10_000
    return F(N)


def main() -> float:
    """Main entry point."""
    result = solve()
    print(f"{result:.9f}")
    return result


if __name__ == "__main__":
    main()

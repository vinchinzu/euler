"""Project Euler Problem 107: Minimal Network.

Using network.txt, find the maximum saving which can be achieved by removing
redundant edges whilst ensuring that the network remains connected.
"""

import os
from pathlib import Path
from typing import List, Tuple


class DSU:
    """Disjoint Set Union data structure for Kruskal's algorithm."""

    def __init__(self, n: int) -> None:
        """Initialize DSU with n elements."""
        self.parent = list(range(n))
        self.rank = [0] * n

    def find(self, x: int) -> int:
        """Find root with path compression."""
        if self.parent[x] != x:
            self.parent[x] = self.find(self.parent[x])
        return self.parent[x]

    def union(self, x: int, y: int) -> bool:
        """Union two sets. Returns True if union was performed."""
        px, py = self.find(x), self.find(y)
        if px == py:
            return False
        if self.rank[px] < self.rank[py]:
            self.parent[px] = py
        elif self.rank[px] > self.rank[py]:
            self.parent[py] = px
        else:
            self.parent[py] = px
            self.rank[px] += 1
        return True


def max_saving(network_data: str) -> int:
    """Calculate maximum saving using Kruskal's algorithm."""
    edges: List[Tuple[int, int, int]] = []

    # Parse the matrix
    rows = network_data.strip().split('\n')
    for u, line in enumerate(rows):
        # Split and strip each entry to handle spaces around '-'
        parts = [p.strip() for p in line.strip().split(',')]
        for v, part in enumerate(parts):
            if part == '-':
                continue

            try:
                weight = int(part)
            except ValueError:
                continue

            if u < v:
                edges.append((weight, u, v))

    total_weight = sum(weight for weight, _u, _v in edges)

    # Kruskal's algorithm
    edges.sort(key=lambda x: x[0])  # Sort by weight efficiently
    dsu = DSU(len(rows))
    mst_weight = 0
    edge_count = 0
    target_edges = len(rows) - 1

    for weight, u, v in edges:
        if dsu.union(u, v):
            mst_weight += weight
            edge_count += 1
            if edge_count == target_edges:
                break

    return total_weight - mst_weight


def main() -> int:
    """Main function."""
    script_dir = Path(__file__).parent
    possible_paths = [
        script_dir.parent / 'data' / 'network.txt',
        script_dir / 'network.txt',
    ]
    filename = None
    for path in possible_paths:
        if path.exists():
            filename = str(path)
            break

    if filename is None:
        return 0

    with open(filename, 'r') as f:
        network_data = f.read()
    return max_saving(network_data)


if __name__ == "__main__":
    print(main())

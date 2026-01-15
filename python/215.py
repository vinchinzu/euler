"""Project Euler Problem 215: Crack-free Walls.

Find the number of ways to build a wall of width W and height H using kx1
blocks (for k in WIDTHS) such that no two gaps in adjacent rows line up.
"""

from __future__ import annotations

from collections import defaultdict
from typing import DefaultDict, List, Set


def solve() -> int:
    """Solve Problem 215."""
    W = 32
    H = 10
    WIDTHS = [2, 3]

    row_bitsets: List[int] = []

    def helper(x: int, row_bitset: int) -> None:
        """Generate all possible row configurations."""
        for width in WIDTHS:
            if x + width == W:
                row_bitsets.append(row_bitset)
            elif x + width < W:
                helper(x + width, row_bitset | (1 << (x + width)))

    helper(0, 0)
    n = len(row_bitsets)

    # Build adjacency graph
    next_map: DefaultDict[int, Set[int]] = defaultdict(set)
    for i in range(n):
        for j in range(n):
            if (row_bitsets[i] & row_bitsets[j]) == 0:
                next_map[i].add(j)

    num_ways = [1] * n

    for y in range(2, H + 1):
        new_num_ways = [0] * n
        for i in range(n):
            for j in next_map[i]:
                new_num_ways[j] += num_ways[i]
        num_ways = new_num_ways

    return sum(num_ways)


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()

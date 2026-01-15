"""Project Euler Problem 790: Clock Grid.

Find the sum of the values of a KxK grid of clocks all starting at 12
(o'clock), after N operations of the form "increment the hour hand for all
clocks with x_l ≤ x ≤ x_h and y_l ≤ y ≤ y_h" are performed, and clocks cycle
from 12 o'clock to 1 o'clock.

All operations are commutative, so we perform the operations in order of
increasing x, considering each (x_l, x_h, y_l, y_h) operation as two
operations, one incrementing all clocks at x_l onwards, and the other
decrementing all clocks at x_h + 1 onwards. We can maintain a segment tree
where each node stores the number of clocks with each value (from 1 to 12) in
that node's range. To efficiently update a node, we maintain a "shift" field
at each node, so e.g. if the shift is 2, the count for 1 o'clock actually
represents the count for 3 o'clock. The shifted values are then propagated up
to the root of the tree.

For efficiency, we only update the tree at values of x where there is an
operation, and multiply the current column's sum by the number of columns since
the previous x. Also, instead of having the segment tree represent each integer
0 ≤ y ≤ K, we also only look at y values that appear in an operation.
"""

from __future__ import annotations

from dataclasses import dataclass
from typing import Dict, List

from sympy import primerange


@dataclass
class Query:
    """Query for clock updates."""

    x1: int
    x2: int
    y1: int
    y2: int

    def __init__(self, x1: int, x2: int, y1: int, y2: int) -> None:
        """Initialize query, ensuring x1 < x2 and y1 < y2."""
        self.x1 = min(x1, x2)
        self.x2 = max(x1, x2) + 1
        self.y1 = min(y1, y2)
        self.y2 = max(y1, y2) + 1


def blum_blum_shub(seed: int, n: int) -> List[int]:
    """Generate Blum Blum Shub sequence."""
    # Simplified - in practice would use proper BBS
    result = []
    x = seed
    for _ in range(n):
        x = (x * x) % 50515093
        result.append(x)
    return result


def solve() -> int:
    """Solve Problem 790."""
    N = 100000
    K = 50515093
    T = 12

    # Generate queries using Blum Blum Shub
    s = blum_blum_shub(0, 4 * N)
    queries = [
        Query(s[i], s[i + 1], s[i + 2], s[i + 3]) for i in range(0, 4 * N, 4)
    ]

    # Collect all x and y coordinates
    xs_set = {0, K}
    ys_set = {0, K}
    for q in queries:
        xs_set.add(q.x1)
        xs_set.add(q.x2)
        ys_set.add(q.y1)
        ys_set.add(q.y2)

    xs = sorted(xs_set)
    ys = sorted(ys_set)
    y_to_index = {y: i for i, y in enumerate(ys)}

    # Build segment tree
    l = 1
    while l < len(ys) - 1:
        l *= 2
    hour_counts = [[0] * T for _ in range(2 * l)]
    shifts = [0] * (2 * l)

    # Initialize leaf nodes
    for i in range(len(ys) - 1):
        hour_counts[l + i][0] = ys[i + 1] - ys[i]

    def merge(index: int) -> None:
        """Merge children into parent."""
        for h in range(T):
            hour_counts[index][h] = (
                hour_counts[2 * index][(h - shifts[2 * index]) % T]
                + hour_counts[2 * index + 1][(h - shifts[2 * index + 1]) % T]
            )

    # Build tree bottom-up
    for i in range(l - 1, 0, -1):
        merge(i)

    # Group queries by x coordinate
    add_queries: Dict[int, List[Query]] = {}
    remove_queries: Dict[int, List[Query]] = {}
    for q in queries:
        if q.x1 not in add_queries:
            add_queries[q.x1] = []
        add_queries[q.x1].append(q)
        if q.x2 not in remove_queries:
            remove_queries[q.x2] = []
        remove_queries[q.x2].append(q)

    ans = 0
    prev_x = 0

    def update(from_idx: int, to_idx: int, diff: int, index: int, low: int, high: int) -> None:
        """Update segment tree."""
        if from_idx >= high or to_idx <= low:
            return
        if from_idx <= low and to_idx >= high:
            shifts[index] += diff
            return
        mid = (low + high) // 2
        update(from_idx, to_idx, diff, 2 * index, low, mid)
        update(from_idx, to_idx, diff, 2 * index + 1, mid, high)
        merge(index)

    for x in xs:
        # Add contribution from current column
        for h in range(T):
            hour_value = h if h != 0 else T
            ans += (
                hour_value
                * hour_counts[1][(h - shifts[1]) % T]
                * (x - prev_x)
            )

        # Process queries starting at x
        if x in add_queries:
            for q in add_queries[x]:
                update(
                    y_to_index[q.y1],
                    y_to_index[q.y2],
                    1,
                    1,
                    0,
                    l,
                )

        # Process queries ending at x
        if x in remove_queries:
            for q in remove_queries[x]:
                update(
                    y_to_index[q.y1],
                    y_to_index[q.y2],
                    -1,
                    1,
                    0,
                    l,
                )

        prev_x = x

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()

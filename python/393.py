#!/usr/bin/env python3
"""Project Euler Problem 393 - Ant Migration

Find ways for n² ants on an n×n grid to move simultaneously to adjacent squares
such that no two ants end up in same square and no two ants swap squares.

Solution uses dynamic programming with flow states across edges.
"""

from collections import defaultdict

def solve():
    N = 10

    # Precompute zero-flow combinations
    # For each (top_flow, left_flow), find valid (bottom_flow, right_flow)
    # such that {top, left, bottom, right} = {1, -1, 0, 0}
    zero_flows = defaultdict(list)

    for top in [-1, 0, 1]:
        for left in [-1, 0, 1]:
            for bottom in [-1, 0, 1]:
                for right in [-1, 0, 1]:
                    flows = sorted([top, left, bottom, right])
                    if flows == [-1, 0, 0, 1]:
                        zero_flows[(top, left)].append((bottom, right))

    # Initial state: no flows on any edge
    no_flow = tuple([0] * N)
    counts = {(no_flow, 0): 1}

    # Process each square from top to bottom, left to right
    for row in range(N):
        for col in range(N):
            new_counts = defaultdict(int)

            for (top_flows, left_flow), count in counts.items():
                # Current square's top flow comes from state
                top_flow = top_flows[0]

                # Try all valid zero-flow combinations
                for bottom_flow, right_flow in zero_flows[(top_flow, left_flow)]:
                    # Check constraint: rightmost column must have right_flow = 0
                    if col == N - 1 and right_flow != 0:
                        continue

                    # Update state: shift top_flows and add new bottom_flow
                    new_top_flows = tuple(list(top_flows[1:]) + [bottom_flow])
                    new_state = (new_top_flows, right_flow)
                    new_counts[new_state] += count

            counts = new_counts

    # Answer is count at final state with no flows
    return counts.get((no_flow, 0), 0)

if __name__ == "__main__":
    print(solve())

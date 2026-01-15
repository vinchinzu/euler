"""Project Euler Problem 534: Weak Queens.

Let Q(n, w) be the number of ways to arrange n queens on a n x n chessboard
if each queen can only move at most n-1-w squares vertically or diagonally.
Find Σ_{w=0}^{n-1} Q(N, w).

We brute force with memoization. The number of rows k that the queen can
move ranges from 0 to N-1. For k=0, the number of solutions is obviously
N^N. For k>0, we maintain the set of all solutions for k rows, by
incrementally adding all valid queen positions to each of the previous
solutions for k-1 rows.

Because the board is vertically symmetric, we can reduce the runtime by a
factor of 2.
"""

from __future__ import annotations

from typing import Dict, List, Tuple


def can_append(col: int, configuration: List[int]) -> bool:
    """Check if column can be appended to configuration."""
    for row in range(len(configuration)):
        prev_col = configuration[len(configuration) - row - 1]
        if prev_col == col or prev_col == col + row + 1 or prev_col == col - row - 1:
            return False
    return True


def solve() -> int:
    """Solve Problem 534."""
    N = 14

    ans = pow(N, N)

    configurations: List[List[int]] = [[]]
    for k in range(1, N):
        new_configurations: List[List[int]] = []
        for configuration in configurations:
            max_col = N if configuration else N // 2
            for col in range(max_col):
                if can_append(col, configuration):
                    new_configuration = configuration + [col]
                    new_configurations.append(new_configuration)
        configurations = new_configurations

        # Create ordering map
        ordering: Dict[Tuple[int, ...], int] = {
            tuple(config): i for i, config in enumerate(configurations)
        }

        # Build next configuration indices
        next_config_indices = [[-1] * N for _ in range(len(configurations))]
        for i, configuration in enumerate(configurations):
            for col in range(N):
                if can_append(col, configuration):
                    next_config = configuration[1:] + [col]
                    if 2 * next_config[0] >= N:
                        next_config = [N - 1 - c for c in next_config]
                    next_config_indices[i][col] = ordering.get(tuple(next_config), -1)

        # Count configurations
        counts = [1] * len(configurations)
        for row in range(k, N):
            new_counts = [0] * len(configurations)
            for i in range(len(configurations)):
                for col in range(N):
                    idx = next_config_indices[i][col]
                    if idx != -1:
                        new_counts[i] += counts[idx]
            counts = new_counts

        Q = sum(counts)
        ans += 2 * Q

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()

"""Project Euler Problem 213: Flea Circus.

An NxN grid has 1 flea per square. At each time step, each flea jumps to one
of the orthogonally neighboring squares with equal probability. Find the
expected number of empty squares after K time steps.
"""

from __future__ import annotations

from typing import List, Tuple


def solve() -> float:
    """Solve Problem 213."""
    N = 30
    K = 50

    # Directions: up, down, left, right
    CARDINALS = [(-1, 0), (1, 0), (0, -1), (0, 1)]

    def in_bounds(i: int, j: int, size: int) -> bool:
        """Check if (i, j) is within bounds."""
        return 0 <= i < size and 0 <= j < size

    # Compute probability distributions for fleas in top left quarter
    table: List[List[List[List[float]]]] = [
        [[[0.0] * N for _ in range(N)] for _ in range(N // 2)] for _ in range(N // 2)
    ]

    for fleaI in range(N // 2):
        for fleaJ in range(N // 2):
            grid = [[0.0] * N for _ in range(N)]
            grid[fleaI][fleaJ] = 1.0

            for _ in range(K):
                new_grid = [[0.0] * N for _ in range(N)]
                for i in range(N):
                    for j in range(N):
                        num_valid_dirs = sum(
                            1 for di, dj in CARDINALS if in_bounds(i + di, j + dj, N)
                        )
                        for di, dj in CARDINALS:
                            if in_bounds(i + di, j + dj, N):
                                new_grid[i + di][j + dj] += grid[i][j] / num_valid_dirs
                grid = new_grid

            table[fleaI][fleaJ] = grid

    ans = 0.0
    for i in range(N):
        for j in range(N):
            prob = 1.0
            for fleaI in range(N // 2):
                for fleaJ in range(N // 2):
                    prob *= (
                        (1 - table[fleaI][fleaJ][i][j])
                        * (1 - table[fleaI][fleaJ][i][N - 1 - j])
                        * (1 - table[fleaI][fleaJ][N - 1 - i][j])
                        * (1 - table[fleaI][fleaJ][N - 1 - i][N - 1 - j])
                    )
            ans += prob

    return ans


def main() -> float:
    """Main entry point."""
    result = solve()
    print(f"{result:.6f}")
    return result


if __name__ == "__main__":
    main()

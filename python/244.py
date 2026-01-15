"""Project Euler Problem 244: Sliders.

Find the checksum of the sequence of moves for a 15-Puzzle variant.
"""

from __future__ import annotations

from collections import deque
from dataclasses import dataclass
from typing import Dict, Set, Tuple


@dataclass(frozen=True)
class Board:
    """Represents a board state."""

    hole_i: int
    hole_j: int
    grid: Tuple[Tuple[int, ...], ...]

    def hash(self) -> int:
        """Compute hash of board."""
        h = self.hole_i * 4 + self.hole_j
        for row in self.grid:
            for val in row:
                h = 2 * h + val
        return h


def solve() -> int:
    """Solve Problem 244."""
    N = 4
    C = 243
    M = 100_000_007

    # Initial board
    S = Board(
        0,
        0,
        (
            (0, 1, 0, 0),
            (1, 1, 0, 0),
            (1, 1, 0, 0),
            (1, 1, 0, 0),
        ),
    )

    # Target board
    T = Board(
        0,
        0,
        (
            (0, 0, 1, 0),
            (0, 1, 0, 1),
            (1, 0, 1, 0),
            (0, 1, 0, 1),
        ),
    ).hash()

    CARDINALS = [(-1, 0, ord("U")), (1, 0, ord("D")), (0, -1, ord("L")), (0, 1, ord("R"))]

    visited: Set[int] = set()
    queue: deque[Tuple[Board, int]] = deque([(S, 0)])

    while queue:
        board, checksum = queue.popleft()
        board_hash = board.hash()

        if board_hash == T:
            return checksum

        if board_hash in visited:
            continue
        visited.add(board_hash)

        for di, dj, key in CARDINALS:
            new_i = board.hole_i - di
            new_j = board.hole_j - dj

            if not (0 <= new_i < N and 0 <= new_j < N):
                continue

            # Create new grid
            grid_list = [list(row) for row in board.grid]
            grid_list[board.hole_i][board.hole_j] = grid_list[new_i][new_j]
            grid_list[new_i][new_j] = 0

            new_board = Board(
                new_i,
                new_j,
                tuple(tuple(row) for row in grid_list),
            )
            new_checksum = (checksum * C + key) % M

            if new_board.hash() not in visited:
                queue.append((new_board, new_checksum))

    return 0


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()

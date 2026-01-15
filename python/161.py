"""Project Euler Problem 161: Triominoes."""

from typing import Dict, List, Tuple, Optional


class TrominoTiler:
    """Tiler for tromino problems."""

    def __init__(self, width: int, height: int) -> None:
        """Initialize tiler with grid dimensions."""
        self.width = width
        self.height = height

        # Ensure total cells are divisible by 3
        if (width * height) % 3 != 0:
            raise ValueError("Total number of cells must be divisible by 3")

        self.grid: List[List[bool]] = [[False] * width for _ in range(height)]
        self.memo: Dict[int, int] = {}

        # All tromino shapes (relative coordinates from pivot point)
        self.trominoes: List[List[Tuple[int, int]]] = [
            # I-tromino horizontal: XXX
            [(0, 0), (0, 1), (0, 2)],
            # I-tromino vertical: X
            #                     X
            #                     X
            [(0, 0), (1, 0), (2, 0)],
            # L-tromino shapes:
            # XX    X.    .X    XX
            # X.    XX    XX    .X
            [(0, 0), (0, 1), (1, 0)],
            [(0, 0), (1, 0), (1, 1)],
            [(0, 0), (0, 1), (1, 1)],
            [(0, 0), (1, 0), (1, -1)],
        ]

    def solve(self) -> int:
        """Solve the tiling problem."""
        grid_key = self.grid_to_key()
        return self.backtrack(grid_key)

    def grid_to_key(self) -> int:
        """Convert grid to a hash key for memoization."""
        key = 0
        for r in range(self.height):
            for c in range(self.width):
                key = (key << 1) | (1 if self.grid[r][c] else 0)
        return key

    def key_to_grid(self, key: int) -> List[List[bool]]:
        """Restore grid from key."""
        grid: List[List[bool]] = [[False] * self.width for _ in range(self.height)]
        for r in range(self.height - 1, -1, -1):
            for c in range(self.width - 1, -1, -1):
                grid[r][c] = (key & 1) == 1
                key >>= 1
        return grid

    def backtrack(self, grid_key: int) -> int:
        """Backtracking with memoization."""
        if grid_key in self.memo:
            return self.memo[grid_key]

        # Restore grid from key
        self.grid = self.key_to_grid(grid_key)

        # Find first empty cell
        empty_pos = self.find_first_empty()

        # If no empty cells, we have a complete tiling
        if empty_pos is None:
            self.memo[grid_key] = 1
            return 1

        r, c = empty_pos
        count = 0

        # Try each tromino at this position
        for tromino in self.trominoes:
            if self.can_place(r, c, tromino):
                # Place tromino
                self.place_tromino(r, c, tromino, True)

                # Recurse
                new_key = self.grid_to_key()
                count += self.backtrack(new_key)

                # Remove tromino (backtrack)
                self.place_tromino(r, c, tromino, False)

        self.memo[grid_key] = count
        return count

    def find_first_empty(self) -> Optional[Tuple[int, int]]:
        """Find first empty cell."""
        for r in range(self.height):
            for c in range(self.width):
                if not self.grid[r][c]:
                    return (r, c)
        return None

    def can_place(self, r: int, c: int, tromino: List[Tuple[int, int]]) -> bool:
        """Check if tromino can be placed at position."""
        for dr, dc in tromino:
            nr, nc = r + dr, c + dc
            if (
                nr < 0
                or nr >= self.height
                or nc < 0
                or nc >= self.width
                or self.grid[nr][nc]
            ):
                return False
        return True

    def place_tromino(
        self, r: int, c: int, tromino: List[Tuple[int, int]], fill: bool
    ) -> None:
        """Place or remove tromino."""
        for dr, dc in tromino:
            self.grid[r + dr][c + dc] = fill


def main() -> int:
    """Main function."""
    # The actual problem: 9x12 grid
    tiler = TrominoTiler(9, 12)
    return tiler.solve()


if __name__ == "__main__":
    print(main())

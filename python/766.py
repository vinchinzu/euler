"""Project Euler Problem 766: Sliding Block Puzzle.

Find the number of distinct reachable configurations starting from the given
configuration of a sliding block puzzle, where pieces identical up to translation
are considered indistinguishable.

We perform depth first search, storing the hashes of all configurations of block
types. To avoid stack overflow errors, we perform recursion using a manual stack.
"""

from __future__ import annotations

from dataclasses import dataclass
from typing import List, Set, Tuple


@dataclass
class Point:
    """2D integer point."""

    x: int
    y: int

    def subtract(self, other: Point) -> Point:
        """Subtract another point."""
        return Point(self.x - other.x, self.y - other.y)

    def __hash__(self) -> int:
        """Hash for use in sets."""
        return hash((self.x, self.y))

    def __eq__(self, other: object) -> bool:
        """Equality comparison."""
        if not isinstance(other, Point):
            return False
        return self.x == other.x and self.y == other.y


@dataclass
class Piece:
    """A puzzle piece."""

    type: str
    points: List[Point]
    x: int = 0
    y: int = 0


@dataclass
class Frame:
    """Frame for manual stack recursion."""

    type_index: int = 0
    dir_index: int = 0
    processed: bool = False


# Cardinal directions: up, right, down, left
CARDINALS = [
    Point(0, -1),  # up
    Point(1, 0),   # right
    Point(0, 1),   # down
    Point(-1, 0),  # left
]


def solve() -> int:
    """Solve Problem 766."""
    GRID = [
        ".AABCC",
        ".ABBCD",
        "EFGGHD",
        "IJGGHK",
        "LMNNKK",
    ]

    grid = [list(row) for row in GRID]
    height = len(grid)
    width = len(grid[0]) if grid else 0

    # Collect all points for each piece ID
    all_points: dict[str, List[Point]] = {}
    for y in range(height):
        for x in range(width):
            char = grid[y][x]
            if char != ".":
                if char not in all_points:
                    all_points[char] = []
                all_points[char].append(Point(x, y))

    # Normalize pieces by translation
    pieces: dict[str, Piece] = {}
    types: dict[frozenset[Point], str] = {}
    for char_id, points_list in all_points.items():
        if char_id == ".":
            continue

        # Find minimum point for normalization
        min_point = min(
            points_list, key=lambda p: (p.y, p.x)
        )

        # Create relative points
        relative_points = frozenset(
            p.subtract(min_point) for p in points_list
        )

        # Get or create type
        if relative_points not in types:
            types[relative_points] = chr(ord("A") + len(types))
        piece_type = types[relative_points]

        # Store piece with original positions
        pieces[char_id] = Piece(piece_type, points_list)

    all_types = list(pieces.keys())
    stack: List[Frame] = [Frame()]
    hashes: Set[int] = set()

    while stack:
        frame = stack[-1]
        char_id = all_types[frame.type_index]
        piece = pieces[char_id]
        dir_offset = CARDINALS[frame.dir_index]

        if frame.processed:
            # Undo move
            for p in piece.points:
                grid[piece.y + p.y][piece.x + p.x] = "."
            piece.y -= dir_offset.y
            piece.x -= dir_offset.x
            for p in piece.points:
                grid[piece.y + p.y][piece.x + p.x] = char_id
        elif can_move(grid, char_id, piece, dir_offset, height, width):
            # Make move
            for p in piece.points:
                grid[piece.y + p.y][piece.x + p.x] = "."
            piece.y += dir_offset.y
            piece.x += dir_offset.x
            for p in piece.points:
                grid[piece.y + p.y][piece.x + p.x] = char_id

            frame.processed = True

            # Compute hash
            hash_val = 0
            for row in grid:
                for c in row:
                    hash_val = hash_val * 31 + (
                        ord(pieces[c].type) if c != "." else 0
                    )

            if hash_val not in hashes:
                hashes.add(hash_val)
                stack.append(Frame())
                continue

        frame.processed = False
        frame.dir_index += 1
        if frame.dir_index == len(CARDINALS):
            frame.dir_index = 0
            frame.type_index += 1
            if frame.type_index == len(all_types):
                stack.pop()

    return len(hashes)


def can_move(
    grid: List[List[str]],
    char_id: str,
    piece: Piece,
    dir_offset: Point,
    height: int,
    width: int,
) -> bool:
    """Check if piece can move in given direction."""
    for p in piece.points:
        new_y = piece.y + p.y + dir_offset.y
        new_x = piece.x + p.x + dir_offset.x

        # Check bounds
        if new_y < 0 or new_y >= height or new_x < 0 or new_x >= width:
            return False

        c = grid[new_y][new_x]
        if c != "." and c != char_id:
            return False

    return True


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()

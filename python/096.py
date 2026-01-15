#!/usr/bin/env python3
"""
Su Doku (Problem 96)

Solve all fifty Sudoku puzzles and find the sum of the 3-digit numbers
in the top left corner of each solution grid.
"""

from typing import List, Tuple, Optional


class SudokuSolver:
    """Solver for Sudoku puzzles."""
    
    def __init__(self, grid_str: str) -> None:
        """Initialize solver with grid string."""
        self.grid = self.parse_grid(grid_str)
    
    def parse_grid(self, grid_str: str) -> List[List[int]]:
        """Parse grid string into 2D list."""
        return [[int(c) for c in line] for line in grid_str.strip().split('\n')]
    
    def solve(self) -> bool:
        """Solve the Sudoku puzzle."""
        return self.solve_recursive(self.grid)
    
    def get_top_left_three_digit_number(self) -> int:
        """Get 3-digit number from top left corner."""
        if not self.solved() or len(self.grid) != 9 or len(self.grid[0]) != 9:
            return 0
        return self.grid[0][0] * 100 + self.grid[0][1] * 10 + self.grid[0][2]
    
    def solved(self) -> bool:
        """Check if puzzle is solved."""
        return all(cell != 0 for row in self.grid for cell in row)
    
    def solve_recursive(self, grid: List[List[int]]) -> bool:
        """Recursive backtracking solver."""
        find = self.find_empty(grid)
        if find is None:
            return True
        
        row, col = find
        
        for num in range(1, 10):
            if self.is_safe(grid, row, col, num):
                grid[row][col] = num
                
                if self.solve_recursive(grid):
                    return True
                
                grid[row][col] = 0
        
        return False
    
    def find_empty(self, grid: List[List[int]]) -> Optional[Tuple[int, int]]:
        """Find empty cell in grid."""
        for r in range(9):
            for c in range(9):
                if grid[r][c] == 0:
                    return (r, c)
        return None
    
    def is_safe(self, grid: List[List[int]], row: int, col: int, num: int) -> bool:
        """Check if placing num at (row, col) is safe."""
        return (not self.used_in_row(grid, row, num) and
                not self.used_in_col(grid, col, num) and
                not self.used_in_box(grid, row - row % 3, col - col % 3, num))
    
    def used_in_row(self, grid: List[List[int]], row: int, num: int) -> bool:
        """Check if num is used in row."""
        return any(grid[row][col] == num for col in range(9))
    
    def used_in_col(self, grid: List[List[int]], col: int, num: int) -> bool:
        """Check if num is used in column."""
        return any(grid[row][col] == num for row in range(9))
    
    def used_in_box(self, grid: List[List[int]], box_start_row: int,
                    box_start_col: int, num: int) -> bool:
        """Check if num is used in 3x3 box."""
        return any(
            grid[box_start_row + r_offset][box_start_col + c_offset] == num
            for r_offset in range(3)
            for c_offset in range(3)
        )


def load_puzzles(filename: str) -> List[str]:
    """Load Sudoku puzzles from file."""
    with open(filename) as f:
        content = f.read().strip()
    
    puzzles_str = content.split("Grid ")
    puzzles_str = puzzles_str[1:]  # Remove empty first element
    
    puzzles = []
    for puzzle_block in puzzles_str:
        lines = puzzle_block.split('\n')
        puzzles.append('\n'.join(lines[1:]).strip())
    
    return puzzles


def main() -> None:
    """Solve all puzzles and sum top-left numbers."""
    from pathlib import Path
    script_dir = Path(__file__).parent
    data_file = script_dir.parent / 'data' / 'p096_sudoku.txt'
    
    puzzles = load_puzzles(str(data_file))
    total_sum = 0
    
    for index, puzzle_str in enumerate(puzzles):
        solver = SudokuSolver(puzzle_str)
        if solver.solve():
            top_left_num = solver.get_top_left_three_digit_number()
            total_sum += top_left_num
        else:
            print(f"Could not solve Puzzle #{index + 1}", file=__import__('sys').stderr)
    
    print(total_sum)


if __name__ == "__main__":
    main()

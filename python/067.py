#!/usr/bin/env python3
"""
Project Euler Problem 67: Maximum path sum in a triangle

Find the maximum total from top to bottom by moving to adjacent numbers on the row below.
"""

from pathlib import Path

# Read triangle from file
def read_triangle(filename: str):
    """Read triangle data from file."""
    lines = []
    with open(filename, 'r') as f:
        for line in f:
            line = line.strip()
            if line:
                lines.append([int(x) for x in line.split()])
    return lines


def max_path_sum(triangle):
    """Find maximum path sum from top to bottom using dynamic programming."""
    rows = len(triangle)

    # Start from the second-to-last row and work upwards
    for i in range(rows - 2, -1, -1):
        # Update row i with max path sums from children
        for j in range(len(triangle[i])):
            triangle[i][j] += max(triangle[i + 1][j], triangle[i + 1][j + 1])

    # The top element now contains the maximum path sum
    return triangle[0][0]


def main():
    script_dir = Path(__file__).parent
    # Try multiple possible locations for triangle.txt
    possible_paths = [
        script_dir.parent / 'data' / '0067_triangle.txt',
        script_dir.parent / 'data' / 'triangle.txt',
        script_dir / 'triangle.txt',
        script_dir.parent / 'triangle.txt',
    ]
    
    triangle_file = None
    for path in possible_paths:
        if path.exists():
            triangle_file = str(path)
            break
    
    if triangle_file is None:
        print("Error: triangle.txt not found")
        return
    
    triangle = read_triangle(triangle_file)
    if not triangle:
        print("Error: Empty triangle")
        return
    
    result = max_path_sum(triangle)
    print(result)


if __name__ == "__main__":
    main()

#!/usr/bin/env python3
"""
Path sum: two ways (Problem 81)

Find the minimal path sum from the top left to the bottom right by only
moving right and down in an 80x80 matrix.
"""

def main() -> None:
    """Read matrix and compute minimal path sum."""
    from pathlib import Path
    script_dir = Path(__file__).parent
    # Try multiple possible locations
    possible_paths = [
        script_dir.parent / 'data' / 'matrix.txt',
        script_dir / 'matrix.txt',
        script_dir.parent / 'matrix.txt',
    ]
    matrix_file = None
    for path in possible_paths:
        if path.exists():
            matrix_file = path
            break
    
    if matrix_file is None:
        print("Error: matrix.txt not found")
        return
    
    # Read matrix data
    with open(matrix_file) as f:
        lines = f.read().strip().split('\n')
    
    matrix = []
    for line in lines:
        matrix.append([int(x) for x in line.split(',')])
    
    size = len(matrix)
    
    # Initialize dp table
    dp = [[0] * size for _ in range(size)]
    
    # Base case: top-left corner
    dp[0][0] = matrix[0][0]
    
    # Fill first row
    for j in range(1, size):
        dp[0][j] = matrix[0][j] + dp[0][j-1]
    
    # Fill first column
    for i in range(1, size):
        dp[i][0] = matrix[i][0] + dp[i-1][0]
    
    # Fill rest of dp table
    for i in range(1, size):
        for j in range(1, size):
            dp[i][j] = matrix[i][j] + min(dp[i-1][j], dp[i][j-1])
    
    print(dp[size-1][size-1])


if __name__ == "__main__":
    main()

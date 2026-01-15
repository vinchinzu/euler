#!/usr/bin/env python3
"""
Path sum: three ways (Problem 82)

Find the minimal path sum from any cell in the left column to any cell in
the right column, moving only up, down, and right.
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
    
    # Initialize first column
    for i in range(size):
        dp[i][0] = matrix[i][0]
    
    # Process columns from left to right
    for j in range(1, size):
        # Pass 1: Initialize from left column
        for i in range(size):
            dp[i][j] = dp[i][j-1] + matrix[i][j]
        
        # Pass 2: Consider paths from above
        for i in range(1, size):
            dp[i][j] = min(dp[i][j], dp[i-1][j] + matrix[i][j])
        
        # Pass 3: Consider paths from below
        for i in range(size - 2, -1, -1):
            dp[i][j] = min(dp[i][j], dp[i+1][j] + matrix[i][j])
    
    # Find minimum in last column
    min_path_sum = min(dp[i][size-1] for i in range(size))
    print(min_path_sum)


if __name__ == "__main__":
    main()

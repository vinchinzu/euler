#!/usr/bin/env python3
"""
Path sum: four ways (Problem 83)

Find the minimal path sum from top left to bottom right by moving
left, right, up, and down. Uses Dijkstra's algorithm.
"""

import heapq
from typing import List, Tuple


def main() -> None:
    """Read matrix and compute minimal path sum using Dijkstra."""
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
    
    matrix: List[List[int]] = []
    for line in lines:
        matrix.append([int(x) for x in line.split(',')])
    
    size = len(matrix)
    
    # min_sums[r][c] stores minimum sum to reach (r,c) from (0,0)
    min_sums: List[List[float]] = [[float('inf')] * size for _ in range(size)]
    
    # Priority queue: (cost, row, col)
    pq: List[Tuple[int, int, int]] = []
    
    # Initialize starting cell
    min_sums[0][0] = matrix[0][0]
    heapq.heappush(pq, (min_sums[0][0], 0, 0))
    
    # Possible moves: right, down, left, up
    moves = [(0, 1), (1, 0), (0, -1), (-1, 0)]
    
    while pq:
        cost, r, c = heapq.heappop(pq)
        
        # Skip if we found a shorter path already
        if cost > min_sums[r][c]:
            continue
        
        # Explore neighbors
        for dr, dc in moves:
            nr, nc = r + dr, c + dc
            
            if 0 <= nr < size and 0 <= nc < size:
                new_cost = cost + matrix[nr][nc]
                
                if new_cost < min_sums[nr][nc]:
                    min_sums[nr][nc] = new_cost
                    heapq.heappush(pq, (new_cost, nr, nc))
    
    print(int(min_sums[size-1][size-1]))


if __name__ == "__main__":
    main()

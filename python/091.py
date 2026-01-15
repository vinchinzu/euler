#!/usr/bin/env python3
"""
Right triangles with integer coordinates (Problem 91)

Given that 0 <= x1, y1, x2, y2 <= 50, how many right triangles can be formed?
"""

GRID_MAX = 50


def main() -> None:
    """Count right triangles."""
    right_triangle_count = 0
    
    for x1 in range(GRID_MAX + 1):
        for y1 in range(GRID_MAX + 1):
            for x2 in range(GRID_MAX + 1):
                for y2 in range(GRID_MAX + 1):
                    # Skip invalid cases
                    if x1 == 0 and y1 == 0:
                        continue
                    if x2 == 0 and y2 == 0:
                        continue
                    if x1 == x2 and y1 == y2:
                        continue
                    
                    # Avoid double counting
                    if (x1 > x2) or (x1 == x2 and y1 > y2):
                        continue
                    
                    # Calculate squared distances
                    d_op_sq = x1*x1 + y1*y1
                    d_oq_sq = x2*x2 + y2*y2
                    d_pq_sq = (x1-x2)**2 + (y1-y2)**2
                    
                    # Check for right angle
                    is_right_triangle = False
                    if d_op_sq + d_oq_sq == d_pq_sq:
                        is_right_triangle = True
                    elif d_op_sq + d_pq_sq == d_oq_sq:
                        is_right_triangle = True
                    elif d_oq_sq + d_pq_sq == d_op_sq:
                        is_right_triangle = True
                    
                    if is_right_triangle:
                        right_triangle_count += 1
    
    print(right_triangle_count)


if __name__ == "__main__":
    main()

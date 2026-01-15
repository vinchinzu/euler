#!/usr/bin/env python3
"""
Cuboid route (Problem 86)

Find the least value of M such that the number of distinct cuboids with
integer shortest path exceeds one million.
"""

import math

TARGET_SOLUTIONS = 1_000_000


def main() -> None:
    """Find M where solutions exceed target."""
    solutions_found_count = 0
    m_val = 0
    
    while True:
        m_val += 1
        l_dim = m_val
        
        # Iterate through possible sums of W+H
        for sum_wh in range(2, 2 * l_dim + 1):
            path_length_squared = l_dim * l_dim + sum_wh * sum_wh
            path_length = math.sqrt(path_length_squared)
            
            if path_length == math.floor(path_length):  # Integer path
                # Count valid pairs (W,H) that sum to sum_wh
                upper_bound_for_h = sum_wh // 2
                lower_bound_for_h = max(1, sum_wh - l_dim) if sum_wh > l_dim else 1
                
                if lower_bound_for_h <= upper_bound_for_h:
                    num_valid_pairs = upper_bound_for_h - lower_bound_for_h + 1
                    solutions_found_count += num_valid_pairs
        
        if solutions_found_count > TARGET_SOLUTIONS:
            print(m_val)
            break


if __name__ == "__main__":
    main()

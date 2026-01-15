#!/usr/bin/env python3
"""
Counting rectangles (Problem 85)

Find the area of the grid with the nearest solution to 2,000,000 rectangles.
"""

TARGET_RECTS = 2_000_000
M_SEARCH_LIMIT = 2000


def main() -> None:
    """Find grid area closest to target rectangle count."""
    min_difference = float('inf')
    area_for_min_difference = 0
    
    for m in range(1, M_SEARCH_LIMIT + 1):
        m_term = m * (m + 1) // 2
        
        # Early termination optimization
        if m_term > TARGET_RECTS and m_term - TARGET_RECTS > min_difference:
            if min_difference != float('inf'):
                break
        
        for n in range(1, m + 1):
            n_term = n * (n + 1) // 2
            current_num_rectangles = m_term * n_term
            current_difference = abs(current_num_rectangles - TARGET_RECTS)
            
            if current_difference < min_difference:
                min_difference = current_difference
                area_for_min_difference = m * n
            
            # Early termination for inner loop
            if current_num_rectangles > TARGET_RECTS:
                break
    
    print(area_for_min_difference)


if __name__ == "__main__":
    main()

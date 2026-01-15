#!/usr/bin/env python3
"""
Almost equilateral triangles (Problem 94)

Find the sum of the perimeters of all almost equilateral triangles with
integral side lengths and area, with perimeters not exceeding one billion.
"""

LIMIT = 1_000_000_000


def main() -> None:
    """Find sum of perimeters using recurrence relation."""
    sum_of_perimeters = 0
    
    # X_k sequence for X^2 - 3Y^2 = 1
    # Recurrence: X_{n+1} = 4*X_n - X_{n-1}
    xk_minus_1 = 1  # X0
    xk = 2          # X1
    current_k = 1
    
    while True:
        # Calculate X_{current_k+1}
        xk_plus_1 = 4 * xk - xk_minus_1
        index_of_xk_plus_1 = current_k + 1
        
        if index_of_xk_plus_1 % 2 == 0:
            # Even index: case (a,a,a+1)
            perimeter = 2 * xk_plus_1 + 2
        else:
            # Odd index: case (a,a,a-1)
            perimeter = 2 * xk_plus_1 - 2
        
        if perimeter > LIMIT:
            break
        
        sum_of_perimeters += perimeter
        
        # Update for next iteration
        xk_minus_1 = xk
        xk = xk_plus_1
        current_k = index_of_xk_plus_1
    
    print(sum_of_perimeters)


if __name__ == "__main__":
    main()

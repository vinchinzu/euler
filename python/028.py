#!/usr/bin/env python3
"""
Project Euler Problem 28: Number spiral diagonals

Starting with the number 1 and moving to the right in a clockwise direction a 5 by 5
spiral is formed. It can be verified that the sum of the numbers on the diagonals is 101.

What is the sum of the numbers on the diagonals in a 1001 by 1001 spiral formed in the same way?
"""

from math import prod


def spiral_sum_formula(n: int) -> int:
    """
    Calculate the sum of the numbers on the diagonals of an n x n spiral using a direct formula.
    n must be an odd number.
    """
    if n == 1:
        return 1
    
    m = (n - 1) // 2  # Number of layers around the central '1'
    
    # Sum formulas:
    # Σ k² = m(m+1)(2m+1)/6
    # Σ k = m(m+1)/2
    sum_k_squared = m * (m + 1) * (2 * m + 1) // 6
    sum_k = m * (m + 1) // 2
    
    total_sum = 1 + 16 * sum_k_squared + 4 * sum_k + 4 * m
    return total_sum


def main():
    result = spiral_sum_formula(1001)
    print(result)


if __name__ == "__main__":
    main()

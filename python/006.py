#!/usr/bin/env python3
"""
Project Euler Problem 6: Sum square difference

The sum of the squares of the first ten natural numbers is,
1^2 + 2^2 + ... + 10^2 = 385.

The square of the sum of the first ten natural numbers is,
(1 + 2 + ... + 10)^2 = 55^2 = 3025.

Hence the difference between the sum of the squares of the first ten
natural numbers and the square of the sum is 3025 - 385 = 2640.

Find the difference between the sum of the squares of the first one
hundred natural numbers and the square of the sum.
"""


def difference_between_square_of_sum_and_sum_of_squares(n: int) -> int:
    """Calculate the difference between (sum of 1..n)^2 and sum of squares of 1..n."""
    if n < 1:
        raise ValueError("n must be a positive integer")
    
    # O(1) formula calculations
    sum_n = n * (n + 1) // 2
    sum_of_squares = n * (n + 1) * (2 * n + 1) // 6
    
    return sum_n ** 2 - sum_of_squares


def main():
    result = difference_between_square_of_sum_and_sum_of_squares(100)
    print(result)


if __name__ == "__main__":
    main()

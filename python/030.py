#!/usr/bin/env python3
"""
Project Euler Problem 30: Digit fifth powers

Surprisingly there are only three numbers that can be written as the sum of fourth powers
of their digits: 1634, 8208, 9474. As 1 = 1^4 is not a sum it is not included.
The sum of these numbers is 19316.

Find the sum of all the numbers that can be written as the sum of fifth powers of their digits.
"""

# Precompute fifth powers of digits 0-9
FIFTH_POWERS = [i**5 for i in range(10)]


def sum_of_fifth_powers(n: int) -> int:
    """Calculate the sum of fifth powers of digits of n."""
    if n <= 0:
        return 0
    return sum(FIFTH_POWERS[int(d)] for d in str(n))


def main():
    # Upper limit: 6 * 9^5 = 354294 (maximum possible sum for 6-digit number)
    max_n = 6 * FIFTH_POWERS[9]
    
    total_sum = 0
    for i in range(2, max_n + 1):
        if i == sum_of_fifth_powers(i):
            total_sum += i
    
    print(total_sum)


if __name__ == "__main__":
    main()

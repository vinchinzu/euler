#!/usr/bin/env python3
"""
Project Euler Problem 34: Digit factorials

Find the sum of all positive integers that are equal to the sum of the
factorial of their digits (excluding the trivial 1 and 2).
"""

from math import factorial


def digit_factorial_sum(n: int, factorials: list) -> int:
    """Calculate sum of factorials of digits."""
    total = 0
    while n > 0:
        total += factorials[n % 10]
        n //= 10
    return total


def main():
    # Precompute factorials 0-9
    factorials = [factorial(i) for i in range(10)]
    
    # Determine upper bound: 10^(d-1) > d * 9! means larger numbers can't work
    max_digits = 1
    while 10 ** (max_digits - 1) <= max_digits * factorials[9]:
        max_digits += 1
    upper_limit = max_digits * factorials[9]
    
    # Find all numbers that equal sum of digit factorials
    result = 0
    for n in range(10, upper_limit + 1):
        if digit_factorial_sum(n, factorials) == n:
            result += n
    
    print(result)


if __name__ == "__main__":
    main()

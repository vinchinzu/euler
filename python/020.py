#!/usr/bin/env python3
"""
Project Euler Problem 20: Factorial digit sum

n! means n × (n - 1) × ... × 3 × 2 × 1.

For example, 10! = 10 × 9 × ... × 3 × 2 × 1 = 3628800,
and the sum of the digits in the number 10! is 3 + 6 + 2 + 8 + 8 + 0 + 0 = 27.

Find the sum of the digits in the number 100!.
"""

from math import prod


def factorial(n: int) -> int:
    """Compute n! as an integer."""
    return prod(range(1, n + 1))


def digit_sum(number: int) -> int:
    """Calculate the sum of digits of a positive integer."""
    return sum(int(d) for d in str(number))


def main():
    result = digit_sum(factorial(100))
    print(result)


if __name__ == "__main__":
    main()

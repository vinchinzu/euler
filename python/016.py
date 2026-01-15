#!/usr/bin/env python3
"""
Project Euler Problem 16: Power digit sum

2^15 = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.
What is the sum of the digits of the number 2^1000?
"""


def sum_digits(number: int) -> int:
    """Calculate the sum of digits of a positive integer."""
    return sum(int(d) for d in str(number))


def main():
    power = 2 ** 1000
    result = sum_digits(power)
    print(result)


if __name__ == "__main__":
    main()

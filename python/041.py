#!/usr/bin/env python3
"""
Project Euler Problem 41: Pandigital primes

We shall say that an n-digit number is pandigital if it makes use of all the digits 1 to n exactly once.
What is the largest n-digit pandigital prime that exists?
"""

from itertools import permutations
from math import isqrt


def is_prime(n: int) -> bool:
    """Check if a number is prime."""
    if n < 2:
        return False
    if n == 2:
        return True
    if n % 2 == 0:
        return False
    if n % 3 == 0:
        return False
    i = 5
    while i * i <= n:
        if n % i == 0 or n % (i + 2) == 0:
            return False
        i += 6
    return True


def main():
    # n=8 and n=9 pandigital numbers are all divisible by 3
    # so we only need to check n=7 and below
    for n in range(7, 0, -1):
        digits = ''.join(str(i) for i in range(1, n + 1))
        # Generate permutations in descending order
        for perm in sorted(permutations(digits, n), reverse=True):
            num = int(''.join(perm))
            if is_prime(num):
                print(num)
                return


if __name__ == "__main__":
    main()

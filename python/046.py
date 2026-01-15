#!/usr/bin/env python3
"""
Project Euler Problem 46: Goldbach's other conjecture

It was proposed by Christian Goldbach that every odd composite number
can be written as the sum of a prime and twice a square.
Find the smallest odd composite that cannot be written as such.
"""

from math import isqrt


def is_prime(n: int) -> bool:
    if n < 2:
        return False
    if n == 2:
        return True
    if n % 2 == 0:
        return False
    i = 3
    while i * i <= n:
        if n % i == 0:
            return False
        i += 2
    return True


def is_goldbach(n: int) -> bool:
    """Check if n = prime + 2 * square."""
    for prime in range(3, n, 2):
        if not is_prime(prime):
            continue
        remaining = n - prime
        if remaining % 2 != 0:
            continue
        square = remaining // 2
        if isqrt(square) ** 2 == square:
            return True
    return False


def main():
    n = 9
    while True:
        if n % 2 == 1 and not is_prime(n) and not is_goldbach(n):
            print(n)
            break
        n += 2


if __name__ == "__main__":
    main()

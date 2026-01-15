#!/usr/bin/env python3
"""
Project Euler Problem 37: Truncatable primes

Find the sum of the only eleven primes that are both truncatable from left to right and right to left.
"""

from math import sqrt


def is_prime(n: int) -> bool:
    """Check if a number is prime."""
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


def is_left_truncatable(n: int) -> bool:
    """Check if prime is left truncatable."""
    if n < 10:
        return False
    s = str(n)
    for i in range(1, len(s)):
        if not is_prime(int(s[i:])):
            return False
    return True


def is_right_truncatable(n: int) -> bool:
    """Check if prime is right truncatable."""
    if n < 10:
        return False
    s = str(n)
    for i in range(1, len(s)):
        if not is_prime(int(s[:-i])):
            return False
    return True


def main():
    truncatable_primes = []
    num = 11
    
    while len(truncatable_primes) < 11:
        if is_prime(num) and is_left_truncatable(num) and is_right_truncatable(num):
            truncatable_primes.append(num)
        num += 2
        if num % 5 == 0:
            num += 2
    
    print(sum(truncatable_primes))


if __name__ == "__main__":
    main()

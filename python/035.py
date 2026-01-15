#!/usr/bin/env python3
"""
Project Euler Problem 35: Circular Primes

The number, 197, is called a circular prime because all rotations of the digits:
197, 971, and 719, are themselves prime.

There are thirteen such primes below 100: 2, 3, 5, 7, 11, 13, 17, 31, 37, 71, 73, 79, and 97.

How many circular primes are there below one million?
"""

from typing import List, Set


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


def generate_primes_below(limit: int) -> List[int]:
    """Generate all primes below limit using Sieve of Eratosthenes."""
    if limit < 2:
        return []
    
    sieve = [True] * limit
    sieve[0] = sieve[1] = False
    
    for i in range(2, int(limit ** 0.5) + 1):
        if sieve[i]:
            for j in range(i * i, limit, i):
                sieve[j] = False
    
    return [i for i, prime in enumerate(sieve) if prime]


def has_disqualifying_digits(num: int) -> bool:
    """Check if number has digits that disqualify it from being circular prime."""
    if num < 10:
        return False
    digits = str(num)
    return any(d in digits for d in '024568')


def is_circular_prime(num: int, prime_set: Set[int]) -> bool:
    """Check if all rotations of num are prime."""
    if has_disqualifying_digits(num):
        return False
    
    if num < 10:
        return True
    
    s = str(num)
    for i in range(len(s)):
        rotation = int(s[i:] + s[:i])
        if rotation not in prime_set:
            return False
    return True


def main():
    LIMIT = 1_000_000
    
    primes = generate_primes_below(LIMIT)
    prime_set = set(primes)
    
    count = sum(1 for prime in primes if is_circular_prime(prime, prime_set))
    print(count)


if __name__ == "__main__":
    main()

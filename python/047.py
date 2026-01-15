#!/usr/bin/env python3
"""
Project Euler Problem 47: Distinct prime factors

Find the first four consecutive integers to have four distinct prime factors each.
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


def prime_factors(n: int) -> set:
    """Return distinct prime factors of n."""
    factors = set()
    
    # Check for 2
    while n % 2 == 0:
        factors.add(2)
        n //= 2
    
    # Check for odd factors
    i = 3
    while i * i <= n:
        while n % i == 0:
            factors.add(i)
            n //= i
        i += 2
    
    if n > 1:
        factors.add(n)
    
    return factors


def main():
    TARGET = 4
    n = 2 * 3 * 5 * 7  # Start from product of first 4 primes
    
    while True:
        count = 0
        consecutive = []
        temp = n
        for _ in range(TARGET):
            if len(prime_factors(temp)) == TARGET:
                consecutive.append(temp)
                count += 1
            else:
                break
            temp += 1
        
        if count == TARGET:
            print(n)
            break
        
        n += 1


if __name__ == "__main__":
    main()

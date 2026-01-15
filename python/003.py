#!/usr/bin/env python3
"""
Project Euler Problem 3: Largest prime factor

The prime factors of 13195 are 5, 7, 13 and 29.
What is the largest prime factor of the number 600851475143?
"""


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


def largest_prime_factor(n: int) -> int:
    """Find the largest prime factor of n."""
    factors = []
    d = 2
    while d * d <= n:
        if n % d == 0 and is_prime(d):
            factors.append(d)
            while n % d == 0:
                n //= d
        d += 1 if d == 2 else 2  # Check 2, then odd numbers only
    
    if n > 1:
        factors.append(n)
    
    return factors[-1] if factors else n


def main():
    n = 600_851_475_143
    result = largest_prime_factor(n)
    print(result)


if __name__ == "__main__":
    main()

#!/usr/bin/env python3
"""
Project Euler Problem 27: Quadratic primes

Euler discovered the remarkable quadratic formula: n² + n + 41
It turns out that the formula will produce 40 primes for the consecutive values n = 0 to 39.

The incredible formula n² - 79n + 1601 was discovered, which produces 80 primes for the
consecutive values n = 0 to 79. The product of the coefficients, -79 and 1601, is -126479.

Considering quadratics of the form: n² + an + b, where |a| < 1000 and |b| < 1000
Find the product of the coefficients, a and b, for the quadratic expression that
produces the maximum number of primes for consecutive values of n, starting with n = 0.
"""

from typing import List, Optional


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


def count_consecutive_primes(a: int, b: int, prime_cache: Optional[dict] = None) -> int:
    """Count consecutive primes for quadratic n² + an + b."""
    if prime_cache is None:
        prime_cache = {}
    
    n = 0
    while True:
        val = n * n + a * n + b
        if val not in prime_cache:
            prime_cache[val] = is_prime(val)
        if not prime_cache[val]:
            break
        n += 1
    return n


def main():
    # Precompute primes for b (1 to 1000)
    primes_b = [b for b in range(1, 1001) if is_prime(b)]
    
    prime_cache = {}
    max_primes = 0
    product = 0
    
    for a in range(-999, 1000):
        for b in primes_b:
            count = count_consecutive_primes(a, b, prime_cache)
            if count > max_primes:
                max_primes = count
                product = a * b
    
    print(product)


if __name__ == "__main__":
    main()

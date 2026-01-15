#!/usr/bin/env python3
"""
Project Euler Problem 7: 10,001st prime

By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13,
we can see that the 6th prime is 13.

What is the 10,001st prime number?
"""


def generate_primes_up_to(limit: int) -> list[int]:
    """Generate all prime numbers up to the given limit using the Sieve of Eratosthenes."""
    if limit < 2:
        return []
    
    sieve = [True] * (limit + 1)
    sieve[0] = False
    sieve[1] = False
    
    i = 2
    while i * i <= limit:
        if sieve[i]:
            for j in range(i * i, limit + 1, i):
                sieve[j] = False
        i += 1
    
    return [i for i, is_prime in enumerate(sieve) if is_prime]


def find_nth_prime(n: int) -> int:
    """Find the nth prime number using the prime number theorem for limit estimation."""
    if n < 1:
        raise ValueError("n must be a positive integer")
    if n == 1:
        return 2
    
    import math
    # Estimate upper bound using prime number theorem: p(n) ≈ n * (ln(n) + ln(ln(n)))
    # Add 20% safety margin
    ln_n = math.log(n)
    ln_ln_n = math.log(ln_n)
    limit = int(n * (ln_n + ln_ln_n) * 1.2)
    limit = max(limit, 120000)  # Minimum reasonable limit
    
    primes = generate_primes_up_to(limit)
    
    # Ensure we have enough primes
    while len(primes) < n:
        limit *= 2
        primes = generate_primes_up_to(limit)
    
    return primes[n - 1]


def main():
    result = find_nth_prime(10001)
    print(result)


if __name__ == "__main__":
    main()

#!/usr/bin/env python3
"""
Project Euler Problem 77: Prime Summations

What is the first value which can be written as a sum of primes in over five thousand
different ways?
"""

LIMIT = 5_000

def is_prime(n):
    """Check if n is prime."""
    if n < 2:
        return False
    if n % 2 == 0:
        return False
    for i in range(3, int(n ** 0.5) + 1):
        if n % i == 0:
            return False
    return True

def main():
    """Find first value that can be written as sum of primes in over 5000 different ways."""
    # Use dynamic programming: ways[n] = number of ways to write n as sum of primes
    # Generate primes up to a reasonable limit
    max_n = 100  # Start small, increase if needed
    is_prime_sieve = [True] * (max_n + 1)
    is_prime_sieve[0] = is_prime_sieve[1] = False

    for i in range(2, int(max_n ** 0.5) + 1):
        if is_prime_sieve[i]:
            for multiple in range(i * i, max_n + 1, i):
                is_prime_sieve[multiple] = False

    primes = [i for i in range(2, max_n + 1) if is_prime_sieve[i]]

    while True:
        ways = [0] * (max_n + 1)
        ways[0] = 1  # Base case: one way to make 0

        # For each prime, update ways array
        for prime in primes:
            for n in range(prime, max_n + 1):
                ways[n] += ways[n - prime]

        # Check if any value exceeds 5000
        for n in range(2, max_n + 1):
            if ways[n] > LIMIT:
                print(n)
                return

        # If not found, increase limit and regenerate primes
        max_n *= 2
        is_prime_sieve = [True] * (max_n + 1)
        is_prime_sieve[0] = is_prime_sieve[1] = False
        for i in range(2, int(max_n ** 0.5) + 1):
            if is_prime_sieve[i]:
                for multiple in range(i * i, max_n + 1, i):
                    is_prime_sieve[multiple] = False
        primes = [i for i in range(2, max_n + 1) if is_prime_sieve[i]]


if __name__ == "__main__":
    main()

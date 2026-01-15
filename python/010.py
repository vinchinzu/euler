#!/usr/bin/env python3
"""
Project Euler Problem 10: Summation of primes

The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
Find the sum of all the primes below two million.
"""


def sum_primes_below(limit: int) -> int:
    """Calculate sum of all primes below the given limit using Sieve of Eratosthenes."""
    if limit <= 2:
        return 0
    
    # Create boolean array: False for 0,1; True for 2..(limit-1)
    is_prime = [False, False] + [True] * (limit - 2)
    
    # Sieve of Eratosthenes
    sqrt_limit = int(limit ** 0.5)
    for i in range(2, sqrt_limit + 1):
        if is_prime[i]:
            for j in range(i * i, limit, i):
                is_prime[j] = False
    
    # Sum all primes
    return sum(i for i, prime in enumerate(is_prime) if prime)


def main():
    result = sum_primes_below(2_000_000)
    print(result)


if __name__ == "__main__":
    main()

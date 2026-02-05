#!/usr/bin/env python3
"""Project Euler Problem 399 - Square-free Fibonacci

Find the Nth square-free Fibonacci number using Wall's Conjecture.

Wall's Conjecture: If F_k is first Fibonacci divisible by p, then F_{k*p}
is first divisible by p².
"""

from math import log10, sqrt

def get_primes(limit):
    """Sieve of Eratosthenes"""
    if limit < 2:
        return []
    sieve = [True] * (limit + 1)
    sieve[0] = sieve[1] = False
    for i in range(2, int(limit**0.5) + 1):
        if sieve[i]:
            for j in range(i*i, limit + 1, i):
                sieve[j] = False
    return [i for i in range(2, limit + 1) if sieve[i]]

def solve():
    N = 10**8
    L = 2 * N
    M = 10**16

    # Mark which Fibonacci indices are square-free
    is_square_free = [True] * L

    primes = get_primes(L)

    for p in primes:
        # Find first Fibonacci index divisible by p
        first_index = 1
        a, b = 1, 1

        while p * first_index < L:
            if a == 0:
                # Mark all multiples of p * first_index as not square-free
                for i in range(p * first_index, L, p * first_index):
                    is_square_free[i] = False
                break

            new_b = (a + b) % p
            a = b
            b = new_b
            first_index += 1

    # Find the Nth square-free Fibonacci index
    index = -1
    count = 0
    while count < N:
        index += 1
        if is_square_free[index]:
            count += 1

    # Compute last 16 digits of F_index
    a, b = 1, 1
    for i in range(index):
        new_b = (a + b) % M
        a = b
        b = new_b

    # Compute scientific notation approximation
    phi = (1 + sqrt(5)) / 2
    log_value = (index + 1) * log10(phi) - log10(sqrt(5))
    exponent = int(log_value)
    mantissa = 10 ** (log_value - exponent)

    return f"{a},{mantissa:.1f}e{exponent}"

if __name__ == "__main__":
    print(solve())

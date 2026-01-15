#!/usr/bin/env python3
"""
Project Euler Problem 70: Totient minimum

Find value of n, 1 < n < 10^7, for which φ(n) is a permutation of n
and ratio n/φ(n) produces a minimum.
"""

import math

N_LIMIT = 10_000_000

def sieve(limit):
    """Sieve of Eratosthenes to generate primes."""
    sieve = [True] * (limit + 1)
    sieve[0] = sieve[1] = False

    for i in range(2, int(math.sqrt(limit)) + 1):
        if sieve[i]:
            for j in range(i * i, limit + 1, i):
                sieve[j] = False

    primes = [i for i in range(2, limit + 1) if sieve[i]]
    return primes

def totient(n, primes):
    """Calculate φ(n) using prime factors."""
    if n == 1:
        return 1

    original_n = n
    result = n

    # Divide out each prime factor once
    for p in primes:
        if p * p > n:
            break

        if n % p == 0:
            result -= result // p
            while n % p == 0:
                n //= p

    # If there are remaining factors (larger than sqrt), divide out
    if n > 1:
        result -= result // n

    return result

def are_permutations(num1, num2):
    """Check if two numbers are permutations of each other."""
    return sorted(str(num1)) == sorted(str(num2))


def min_totient_min():
    """
    To minimize n/φ(n), n should have few prime factors,
    and those factors should be large.
    We iterate through possible n and find minimum ratio.
    """
    primes = sieve(int(N_LIMIT ** 0.5) + 1000)
    min_ratio = float('inf')
    result_n = 0

    # Check n as product of 2 primes (p1 * p2) - this minimizes n/φ(n)
    for i, p1 in enumerate(primes):
        if p1 * p1 >= N_LIMIT:
            break

        for p2 in primes[i:]:
            n = p1 * p2
            if n >= N_LIMIT:
                break

            phi_n = (p1 - 1) * (p2 - 1)
            current_ratio = n / phi_n

            if are_permutations(n, phi_n):
                if current_ratio < min_ratio:
                    min_ratio = current_ratio
                    result_n = n

    return result_n


def main():
    result = min_totient_min()
    print(result)


if __name__ == "__main__":
    main()

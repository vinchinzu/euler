#!/usr/bin/env python3
"""
Project Euler Problem 69: Totient maximum

Find the value of n <= 1,000,000 for which n/φ(n) is a maximum.
"""

import math

LIMIT = 1_000_000

def max_totient_max():
    """
    Find the value of n <= 1,000,000 for which n/φ(n) is maximized.

    The ratio n/φ(n) can be expressed as Π (p / (p-1)) for each distinct prime factor p of n.
    To maximize this value, we want to multiply by terms p/(p-1).
    These terms are always > 1.
    For smaller primes, the term p/(p-1) is larger.

    So, to maximize n/φ(n), n should be the product of the smallest distinct primes
    such that n is still within the limit (<= 1,000,000).
    This type of number (product of initial primes) is called a primorial.
    """
    result_n = 1
    current_prime_candidate = 2

    # Helper function to check for primality
    def is_prime(num, known_primes):
        if num <= 1:
            return False
        if num == 2 or num == 5:
            return False

        # Check divisibility only by previously found primes up to sqrt(num)
        sqrt_num = int(math.sqrt(num))
        for p in known_primes:
            if p > sqrt_num:
                break
            if num % p == 0:
                return False

        return True

    primes_found = []

    # Main loop - build up primes and find the maximum n/φ(n)
    while True:
        is_p = True

        # Check against already found primes
        sqrt_candidate = int(math.sqrt(current_prime_candidate))
        for p in primes_found:
            if p > sqrt_candidate:
                break
            if current_prime_candidate % p == 0:
                is_p = False
                break

        if is_p:
            primes_found.append(current_prime_candidate)

            # Check if multiplying by this prime would keep us within the limit
            if result_n * current_prime_candidate <= LIMIT:
                result_n *= current_prime_candidate
            else:
                # The next product would exceed limit, so current result_n is our answer
                break

        # Increment to next candidate (2 -> 3, then 3 -> 5, 7, 9, 11, etc.)
        if current_prime_candidate == 2:
            current_prime_candidate = 3
        else:
            current_prime_candidate += 2

    return result_n


def main():
    result = max_totient_max()
    print(result)


if __name__ == "__main__":
    main()

#!/usr/bin/env python3
"""
Project Euler Problem 50: Consecutive prime sum

Which prime, below one-million, can be written as the sum of the most consecutive primes?
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


def main():
    LIMIT = 1_000_000
    
    # Generate primes below LIMIT
    primes = [n for n in range(2, LIMIT) if is_prime(n)]
    prime_set = set(primes)
    
    max_count = 0
    result = 0
    
    for start in range(len(primes)):
        for end in range(start + max_count, len(primes)):
            total = sum(primes[start:end])
            if total >= LIMIT:
                break
            if total in prime_set:
                count = end - start
                if count > max_count:
                    max_count = count
                    result = total
    
    print(result)


if __name__ == "__main__":
    main()

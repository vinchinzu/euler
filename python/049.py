#!/usr/bin/env python3
"""
Project Euler Problem 49: Prime permutations

The arithmetic sequence, 1487, 4817, and 8147, is curious because:
- Each term is a prime number
- Each term is a permutation of the digits of the others
- Each term increases by the same amount

Find the other 12-digit number formed by concatenating three 4-digit primes.
"""

from itertools import permutations


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
    # Find all 4-digit primes
    primes = [n for n in range(1000, 10000) if is_prime(n)]
    prime_set = set(primes)
    
    for n1 in primes:
        if n1 == 1487:
            continue
        s1 = ''.join(sorted(str(n1)))
        for n2 in primes:
            if n2 <= n1:
                continue
            diff = n2 - n1
            n3 = n2 + diff
            if n3 > 9999:
                break
            if n3 in prime_set:
                s2 = ''.join(sorted(str(n2)))
                s3 = ''.join(sorted(str(n3)))
                if s1 == s2 == s3:
                    print(f"{n1}{n2}{n3}")
                    return


if __name__ == "__main__":
    main()

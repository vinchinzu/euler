"""Project Euler Problem 291: Panaitopol Primes.

Find the number of primes p < 5*10^15 that can be expressed as
(x^4 - y^4) / (x^3 + y^3) for some positive integers x > y.

Mathematical reduction:
- Such primes must have the form f(y) = (y+1)^2 + y^2 = 2y^2 + 2y + 1.
- We sieve these values: for each prime p <= sqrt(5*10^15), find the two
  roots y1, y2 of f(y) ≡ 0 (mod p) via computing sqrt(-1) mod p (only
  possible when p ≡ 1 mod 4), then mark all multiples as composite.
- Any f(y) with no prime factor <= sqrt(N) is itself prime.
"""

import math
import numpy as np


def solve() -> int:
    N = 5 * 10**15
    limit = 50000000  # f(limit) >= N

    # Step 1: Sieve primes up to sqrt(N) ~ 70.7M
    sqrt_N = int(math.isqrt(N)) + 1
    sieve_small = np.ones(sqrt_N + 1, dtype=bool)
    sieve_small[0] = sieve_small[1] = False
    for i in range(2, int(math.isqrt(sqrt_N)) + 1):
        if sieve_small[i]:
            sieve_small[i * i :: i] = False
    primes = np.nonzero(sieve_small)[0]
    del sieve_small

    # Only primes p ≡ 1 (mod 4) can have -1 as a quadratic residue
    primes = primes[primes % 4 == 1]

    # Boolean array: is_prime[y] means f(y) is prime
    is_prime = np.ones(limit, dtype=bool)
    is_prime[0] = False  # f(0) = 1

    # Candidate bases for finding sqrt(-1) mod p
    CANDIDATES = (
        2, 3, 5, 6, 7, 10, 11, 13, 14, 15, 17, 19, 21, 22, 23,
        26, 29, 31, 33, 34, 37, 38, 41, 42, 43, 46, 47, 51, 53,
    )

    # Step 2: For each prime p ≡ 1 (mod 4), find roots of f(y) ≡ 0 (mod p)
    # and mark those positions as composite (unless f(y) == p, meaning y is prime)
    for p in primes:
        p = int(p)
        exp = (p - 1) >> 2

        # Find r with r^2 ≡ -1 (mod p) using candidate non-residues
        r = 0
        for a in CANDIDATES:
            if a >= p:
                continue
            t = pow(a, exp, p)
            if t * t % p == p - 1:
                r = t
                break
        else:
            for a in range(CANDIDATES[-1] + 1, p):
                t = pow(a, exp, p)
                if t * t % p == p - 1:
                    r = t
                    break

        if r == 0:
            continue

        # Roots of f(y) ≡ 0 (mod p): y ≡ (±r - 1) / 2 (mod p)
        inv2 = (p + 1) >> 1
        y1 = ((r - 1) * inv2) % p
        y2 = ((-r - 1) * inv2) % p

        roots = (y1, y2) if y1 != y2 else (y1,)
        for yr in roots:
            start = yr
            if start == 0:
                start = p  # skip y=0
            elif start < limit and 2 * start * start + 2 * start + 1 == p:
                start += p  # f(yr) == p means f(yr) is prime; skip it
            if start < limit:
                is_prime[start::p] = False

    return int(np.sum(is_prime))


if __name__ == "__main__":
    print(solve())

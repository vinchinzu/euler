#!/usr/bin/env python3
"""
Project Euler 853: Pisano Period

Find the sum of all n < 10^9 for which the Pisano period pi(n) = 120.

Key insight: If pi(n) = K, then n divides F_K (the K-th Fibonacci number).
So we compute F_120, find all its divisors up to 10^9, and check which
have Pisano period exactly 120.

To check pi(n) = K exactly, we verify:
  1) F_K ≡ 0 (mod n) and F_{K+1} ≡ 1 (mod n)  [period divides K]
  2) For every proper divisor d of K where d < K:
     NOT (F_d ≡ 0 (mod n) and F_{d+1} ≡ 1 (mod n))  [period is not smaller]
"""

import sys
from math import gcd


def fibonacci_exact(k):
    """Compute exact F_k using fast doubling."""
    if k == 0:
        return 0
    if k == 1:
        return 1

    def fib_pair(n):
        """Return (F_n, F_{n+1})."""
        if n == 0:
            return (0, 1)
        a, b = fib_pair(n >> 1)
        c = a * (2 * b - a)
        d = a * a + b * b
        if n & 1:
            return (d, c + d)
        else:
            return (c, d)

    return fib_pair(k)[0]


def fibonacci_mod(k, m):
    """Compute (F_k mod m, F_{k+1} mod m) using fast doubling."""
    if m == 1:
        return (0, 0)
    if k == 0:
        return (0, 1)

    def fib_pair_mod(n, m):
        if n == 0:
            return (0, 1)
        a, b = fib_pair_mod(n >> 1, m)
        c = a * (2 * b - a) % m
        d = (a * a + b * b) % m
        if n & 1:
            return (d, (c + d) % m)
        else:
            return (c, d)

    return fib_pair_mod(k, m)


def factorize(n):
    """Return prime factorization of n as dict {prime: exponent}."""
    factors = {}
    d = 2
    while d * d <= n:
        while n % d == 0:
            factors[d] = factors.get(d, 0) + 1
            n //= d
        d += 1
    if n > 1:
        factors[n] = factors.get(n, 0) + 1
    return factors


def divisors_of_k(k):
    """Return sorted list of divisors of k."""
    divs = [1]
    for p, e in factorize(k).items():
        new_divs = []
        pe = 1
        for i in range(e + 1):
            for d in divs:
                new_divs.append(d * pe)
            pe *= p
        divs = new_divs
    divs.sort()
    return divs


def proper_divisors_of_k(k):
    """Return sorted list of proper divisors of k (less than k)."""
    return [d for d in divisors_of_k(k) if d < k]


def has_pisano_period_exactly(n, K, proper_divs):
    """Check if Pisano period of n is exactly K.

    Requires: proper_divs is the list of proper divisors of K.
    """
    if n <= 0:
        return False
    if n == 1:
        return K == 1

    # First check F_K ≡ 0 and F_{K+1} ≡ 1 (mod n)
    fk, fk1 = fibonacci_mod(K, n)
    if fk != 0 or fk1 != 1:
        return False

    # Check that no proper divisor d of K satisfies F_d ≡ 0 and F_{d+1} ≡ 1 (mod n)
    for d in proper_divs:
        fd, fd1 = fibonacci_mod(d, n)
        if fd == 0 and fd1 == 1:
            return False

    return True


def solve():
    K = 120
    N = 10**9

    # Compute F_120 exactly
    F_K = fibonacci_exact(K)

    # Factorize F_120
    factors = factorize(F_K)

    # Generate all divisors of F_K that are <= N
    # We do this recursively to avoid generating huge lists
    primes_list = sorted(factors.keys())
    exponents = [factors[p] for p in primes_list]

    candidates = []

    def generate_divisors(idx, current):
        if current > N:
            return
        if idx == len(primes_list):
            candidates.append(current)
            return
        p = primes_list[idx]
        e = exponents[idx]
        val = current
        for i in range(e + 1):
            generate_divisors(idx + 1, val)
            if i < e:
                val *= p
                if val > N:
                    break

    generate_divisors(0, 1)

    # Get proper divisors of K=120
    proper_divs = proper_divisors_of_k(K)

    # Check each candidate
    total = 0
    for n in candidates:
        if has_pisano_period_exactly(n, K, proper_divs):
            total += n

    print(total)


if __name__ == "__main__":
    solve()

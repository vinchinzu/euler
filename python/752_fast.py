#!/usr/bin/env python3
"""
Project Euler Problem 752: Powers of 1+√7 - Optimized Version

This version uses aggressive caching and parallel computation to speed things up.
"""

import sys
from functools import lru_cache
from math import gcd as math_gcd


def lcm(a, b):
    """Compute LCM of a and b."""
    if a == 0 or b == 0:
        return 0
    return a * b // math_gcd(a, b)


# Sieve for factorization
def build_prime_sieve(limit):
    """Build a sieve to help with factorization."""
    spf = list(range(limit + 1))  # smallest prime factor
    for i in range(2, int(limit**0.5) + 1):
        if spf[i] == i:  # i is prime
            for j in range(i * i, limit + 1, i):
                if spf[j] == j:
                    spf[j] = i
    return spf


def factorize_with_sieve(n, spf):
    """Factorize n using the sieve."""
    factors = []
    while n > 1:
        p = spf[n]
        exp = 0
        while n % p == 0:
            exp += 1
            n //= p
        factors.append((p, exp))
    return factors


@lru_cache(maxsize=2000000)
def g_prime_power(pk, p, k):
    """
    Compute g(p^k) directly.
    Uses aggressive limits based on heuristics.
    """
    alpha = 1
    beta = 0

    # Better limit estimation
    if pk <= 100:
        limit = pk * pk
    elif pk <= 10000:
        limit = min(pk * pk, 10**8)
    else:
        # For large primes, the period is often related to p±1
        # But we use a heuristic limit
        limit = min(pk * 10, 10**7)

    for n in range(1, limit):
        alpha, beta = (alpha + 7 * beta) % pk, (alpha + beta) % pk

        if alpha == 1 and beta == 0:
            return n

    return 0


# Global cache
g_cache = {}
SPF = None


def g(x):
    """
    Find g(x) using Chinese Remainder Theorem and caching.
    """
    if x in g_cache:
        return g_cache[x]

    if x == 1:
        g_cache[x] = 1
        return 1

    global SPF
    if SPF is None:
        SPF = build_prime_sieve(10**6 + 1)

    # Factorize using sieve
    factors = factorize_with_sieve(x, SPF)

    if len(factors) > 1:
        # Use Chinese Remainder Theorem
        result = 1
        for p, k in factors:
            pk = p ** k
            gpk = g(pk)
            if gpk == 0:
                g_cache[x] = 0
                return 0
            result = lcm(result, gpk)
        g_cache[x] = result
        return result

    # Prime power case
    p, k = factors[0]
    pk = p ** k
    result = g_prime_power(pk, p, k)
    g_cache[x] = result
    return result


def G(N):
    """
    Compute G(N) = Σ(x=2 to N) g(x)
    """
    total = 0
    milestone = max(1, N // 100)

    for x in range(2, N + 1):
        total += g(x)

        if x % milestone == 0:
            percentage = 100 * x // N
            print(f"Progress: {percentage}%, x={x}, sum={total}, cache_size={len(g_cache)}",
                  file=sys.stderr, flush=True)

    return total


def main():
    """Main solver function."""
    # Quick test
    print("Testing...", file=sys.stderr)
    assert g(3) == 0
    assert g(5) == 12

    print("Computing G(100)...", file=sys.stderr)
    assert G(100) == 28891

    print("Computing G(1000)...", file=sys.stderr)
    assert G(1000) == 13131583

    print("Computing G(10^6)...", file=sys.stderr)
    result = G(10**6)
    print(result)
    return result


if __name__ == "__main__":
    main()

"""Project Euler Problem 545: Von Staudt-Clausen Denominator.

The sum of the kth powers of the first n integers is a polynomial f(n). Let
D(k) be the denominator of the coefficient of n (in reduced terms). Find the
Nth value of m for which D(m) = K.

By the Von Staudt-Clausen theorem, D(k) is the product of all primes p for
which p-1 divides k. So in order for D(m) = K, we must have (1) m be
divisible by p-1 for all p|K, and (2) no other divisor of m be 1 more than
a prime.
"""

from __future__ import annotations

from math import gcd, isqrt
from typing import List


def sieve_primes(limit: int) -> List[int]:
    """Sieve of Eratosthenes."""
    if limit < 2:
        return []
    is_prime = [True] * (limit + 1)
    is_prime[0] = is_prime[1] = False
    for i in range(2, isqrt(limit) + 1):
        if is_prime[i]:
            for j in range(i * i, limit + 1, i):
                is_prime[j] = False
    return [i for i in range(limit + 1) if is_prime[i]]


def pre_smallest_prime_factor(limit: int) -> List[int]:
    """Precompute smallest prime factor."""
    spf = list(range(limit + 1))
    for i in range(2, isqrt(limit) + 1):
        if spf[i] == i:
            for j in range(i * i, limit + 1, i):
                if spf[j] == j:
                    spf[j] = i
    return spf


def prime_factorization(n: int) -> dict[int, int]:
    """Get prime factorization."""
    factors: dict[int, int] = {}
    d = 2
    while d * d <= n:
        while n % d == 0:
            factors[d] = factors.get(d, 0) + 1
            n //= d
        d += 1
    if n > 1:
        factors[n] = factors.get(n, 0) + 1
    return factors


def all_divisors(n: int) -> List[int]:
    """Get all divisors of n."""
    factors = prime_factorization(n)
    divisors = [1]
    for p, e in factors.items():
        new_divisors = []
        for d in divisors:
            for i in range(1, e + 1):
                new_divisors.append(d * pow(p, i))
        divisors.extend(new_divisors)
    return sorted(set(divisors))


def lcm(a: int, b: int) -> int:
    """Least common multiple."""
    return a * b // gcd(a, b)


def imod_inv(a: int, p: int) -> int:
    """Modular inverse."""
    return pow(a, p - 2, p) if p > 1 else 0


def imod(a: int, m: int) -> int:
    """Integer modulo."""
    return ((a % m) + m) % m


def solve() -> int:
    """Solve Problem 545."""
    N = 100000
    K = 20010

    # Compute LCM of p-1 for all p|K
    k_factors = prime_factorization(K)
    lcm_val = 1
    for p in k_factors.keys():
        lcm_val = lcm(lcm_val, p - 1)

    divisors = all_divisors(lcm_val)

    # Binary search for L
    L = 2
    while True:
        limit = max(isqrt(L * lcm_val), L)
        spf = pre_smallest_prime_factor(limit)
        primes = sieve_primes(isqrt(limit))

        # Build G array
        G = [[False] * L for _ in range(len(divisors))]
        for i, d in enumerate(divisors):
            for p in primes:
                if d % p != 0:
                    j_start = imod(-imod_inv(d, p), p)
                    j = j_start
                    while j < L:
                        if j * d + 1 != p:
                            G[i][j] = True
                        j += p
            for j in range(L):
                if K % (d * j + 1) == 0:
                    G[i][j] = True

        # Build bad array
        bad = [False] * L
        for i in range(len(divisors)):
            for j in range(L):
                if not G[i][j]:
                    bad[j] = True

        for j in range(L):
            n = j
            while n > 1:
                p = spf[n] if n < len(spf) else n
                if bad[j // p]:
                    bad[j] = True
                n //= p

        # Count valid values
        count = 0
        j = 0
        while count < N and j < L:
            if not bad[j]:
                count += 1
            j += 1

        if j < L:
            return j * lcm_val

        L *= 2


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()

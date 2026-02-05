"""Project Euler Problem 517: A real recursion.

Define the function g_a(x) = 1 for x<a and g_a(x) = g_a(x-1) + g_a(x-a)
for x≥a. Find Σ g_{√p}(p) over all prime A ≤ p ≤ B.
"""

from __future__ import annotations

from math import floor, isqrt, sqrt
from typing import List


def is_probable_prime(n: int) -> bool:
    """Check if n is probably prime using trial division."""
    if n < 2:
        return False
    if n == 2:
        return True
    if n % 2 == 0:
        return False
    for i in range(3, isqrt(n) + 1, 2):
        if n % i == 0:
            return False
    return True


def sieve_primes(limit: int) -> List[bool]:
    """Simple sieve returning prime flags up to limit."""
    is_prime = [True] * (limit + 1)
    if limit >= 0:
        is_prime[0] = False
    if limit >= 1:
        is_prime[1] = False
    for i in range(2, isqrt(limit) + 1):
        if is_prime[i]:
            step = i
            start = i * i
            is_prime[start : limit + 1 : step] = [False] * (((limit - start) // step) + 1)
    return is_prime


def ncr_mod(n: int, k: int, mod: int, fact: List[int], inv_fact: List[int]) -> int:
    """Compute nCr mod mod using precomputed factorials."""
    if k < 0 or k > n:
        return 0
    return fact[n] * inv_fact[k] % mod * inv_fact[n - k] % mod


def solve() -> int:
    """Solve Problem 517."""
    A = 10**7
    B = 10**7 + 10000
    M = 10**9 + 7
    ans = 0

    # Precompute factorials up to B (mod is prime and > B).
    fact = [1] * (B + 1)
    for i in range(2, B + 1):
        fact[i] = (fact[i - 1] * i) % M
    inv_fact = [1] * (B + 1)
    inv_fact[B] = pow(fact[B], M - 2, M)
    for i in range(B, 0, -1):
        inv_fact[i - 1] = (inv_fact[i] * i) % M

    prime_flags = sieve_primes(B)

    for p in range(A, B):
        if prime_flags[p]:
            a = sqrt(p)
            num_a = 0
            while num_a * a < p:
                floor_val = floor(p - num_a * a)
                n = num_a + floor_val
                ans = (ans + ncr_mod(n, num_a, M, fact, inv_fact)) % M
                num_a += 1

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()

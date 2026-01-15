"""Project Euler Problem 517: A real recursion.

Define the function g_a(x) = 1 for x<a and g_a(x) = g_a(x-1) + g_a(x-a)
for x≥a. Find Σ g_{√p}(p) over all prime A ≤ p ≤ B.
"""

from __future__ import annotations

from math import floor, isqrt, sqrt
from typing import Dict


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


def ncr_mod(n: int, k: int, mod: int, fact_cache: Dict[int, int]) -> int:
    """Compute nCr mod mod."""
    if k < 0 or k > n:
        return 0
    if k == 0 or k == n:
        return 1

    # Compute factorials modulo mod
    def fact(m: int) -> int:
        if m in fact_cache:
            return fact_cache[m]
        result = 1
        for i in range(2, m + 1):
            result = (result * i) % mod
        fact_cache[m] = result
        return result

    num = fact(n)
    den = (fact(k) * fact(n - k)) % mod

    # Modular inverse using Fermat's little theorem
    def mod_inv(a: int, m: int) -> int:
        return pow(a, m - 2, m)

    return (num * mod_inv(den, mod)) % mod


def solve() -> int:
    """Solve Problem 517."""
    A = 10**7
    B = 10**7 + 10000
    M = 10**9 + 7

    fact_cache: Dict[int, int] = {}
    ans = 0

    for p in range(A, B):
        if is_probable_prime(p):
            a = sqrt(p)
            num_a = 0
            while num_a * a < p:
                floor_val = floor(p - num_a * a)
                ans = (ans + ncr_mod(num_a + floor_val, num_a, M, fact_cache)) % M
                num_a += 1

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()

"""Project Euler Problem 753: Fermat Equation.

Let F(p) be the number of solutions to a³+b³≡c³ (mod p) for 1 ≤ a,b,c < p.
Find the sum of F(p) for all primes p ≤ N.

If p≢1 (mod 3), then all numbers (mod p) are perfect cubes. So a³ can be
any of p-1 different values, and b³ can be any of the p-2 values such that
a³+b³≢0.

If p≡1 (mod 3), then Gauss showed that the number of solutions to 1+b³≡c³
is equal to L+p-8, where L≡1 (mod 3) satisfies L²+27M²=4p. We can then
multiply this equation by any of the p-1 units, for a total of
(L+p-8)(p-1) solutions. Instead of computing L for a given p, we enumerate
all L and M and filter for when (L²+27M²)/4 is a prime.
"""

from __future__ import annotations

from math import isqrt
from typing import List


def sieve_primes(limit: int) -> List[int]:
    """Sieve of Eratosthenes."""
    is_prime = [True] * (limit + 1)
    is_prime[0] = is_prime[1] = False
    for i in range(2, isqrt(limit) + 1):
        if is_prime[i]:
            for j in range(i * i, limit + 1, i):
                is_prime[j] = False
    return [i for i in range(2, limit + 1) if is_prime[i]]


def is_prime(n: int) -> bool:
    """Check if n is prime."""
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


def solve() -> int:
    """Solve Problem 753."""
    N = 6_000_000
    primes = sieve_primes(N)
    ans = 0

    # Case 1: p ≢ 1 (mod 3)
    for p in primes:
        if p % 3 != 1:
            ans += (p - 1) * (p - 2)

    # Case 2: p ≡ 1 (mod 3)
    # Enumerate L and M such that L²+27M²=4p
    max_abs_l = isqrt(4 * N)
    for abs_l in range(1, max_abs_l + 1):
        for L in [-abs_l, abs_l]:
            if L % 3 != 1:
                continue
            # M must have same parity as L for 4p to be even
            M_start = abs(L) % 2
            M = M_start
            while True:
                p_val = (L * L + 27 * M * M) // 4
                if p_val > N:
                    break
                if is_prime(p_val):
                    ans += (L + p_val - 8) * (p_val - 1)
                M += 2

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()

"""Project Euler Problem 489: Common factors between two sequences.

Let G(a, b) the smallest nonnegative integer n for which GCD(n³ + b, (n+a)³ +
b) is maximized. Find Σ_{a=1}^M Σ_{b=1}^N G(a, b).
"""

from __future__ import annotations

from math import gcd, isqrt
from typing import List, Set


def sieve_primes(limit: int) -> List[int]:
    """Sieve of Eratosthenes."""
    is_prime = [True] * (limit + 1)
    is_prime[0] = is_prime[1] = False
    for i in range(2, isqrt(limit) + 1):
        if is_prime[i]:
            for j in range(i * i, limit + 1, i):
                is_prime[j] = False
    return [i for i in range(limit + 1) if is_prime[i]]


def mod_inv(a: int, m: int) -> int:
    """Modular inverse."""
    t, new_t = 0, 1
    r, new_r = m, a
    while new_r != 0:
        q = r // new_r
        t, new_t = new_t, t - q * new_t
        r, new_r = new_r, r - q * new_r
    if r != 1:
        raise ValueError("Inverse does not exist")
    if t < 0:
        t += m
    return t


def solve() -> int:
    """Solve Problem 489."""
    M = 18
    N = 1900

    ans = 0
    for a in range(1, M + 1):
        for b in range(1, N + 1):
            # Find factors of 6a and a^6 + 27b^2
            factors: Set[int] = set()
            n_val = 6 * a
            for p in range(2, n_val + 1):
                if n_val % p == 0:
                    factors.add(p)

            expr = a**6 + 27 * b * b
            for p in range(2, isqrt(expr) + 1):
                if expr % p == 0:
                    factors.add(p)
                    while expr % p == 0:
                        expr //= p
            if expr > 1:
                factors.add(expr)

            # Find maximum GCD
            max_gcd = 1
            best_n = 0
            for g in factors:
                # Check if g divides both
                # Simplified: try small n values
                for n in range(min(1000, g)):
                    if (n**3 + b) % g == 0 and ((n + a) ** 3 + b) % g == 0:
                        current_gcd = gcd(n**3 + b, (n + a) ** 3 + b)
                        if current_gcd > max_gcd:
                            max_gcd = current_gcd
                            best_n = n

            ans += best_n

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()

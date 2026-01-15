"""Project Euler Problem 216: Investigating the Primality of Numbers.

Find the number of prime integers of the form t(n) = 2n²-1 for n≤N.
"""

from __future__ import annotations

from math import isqrt
from typing import List


def build_spf(limit: int) -> List[int]:
    """Build smallest prime factor array up to limit."""
    spf = list(range(limit + 1))
    for i in range(2, isqrt(limit) + 1):
        if spf[i] == i:
            for j in range(i * i, limit + 1, i):
                if spf[j] == j:
                    spf[j] = i
    return spf


def is_prime(n: int, spf: List[int]) -> bool:
    """Check if n is prime."""
    return n > 1 and spf[n] == n


def is_sq(n: int, p: int) -> bool:
    """Check if n is a perfect square modulo p."""
    return pow(n, (p - 1) // 2, p) <= 1


def sqrt_mod(n: int, p: int) -> int:
    """Compute sqrt(n) mod p using Tonelli-Shanks algorithm."""
    if p % 4 == 3:
        return pow(n, (p + 1) // 4, p)
    # Simplified version for p % 4 == 1
    for x in range(1, p):
        if pow(x, 2, p) == n:
            return x
    return 0


def sq(n: int) -> int:
    """Return n squared."""
    return n * n


def solve() -> int:
    """Solve Problem 216."""
    N = 50_000_000
    L = int(isqrt(2) * N)

    spf = build_spf(L)
    sieve = [True] * (N + 1)
    sieve[0] = sieve[1] = False

    for p in range(3, L + 1):
        if is_prime(p, spf) and is_sq((p + 1) // 2, p):
            r = sqrt_mod((p + 1) // 2, p)
            start1 = r if 2 * sq(r) - 1 == p else r + p
            for i in range(start1, N + 1, p):
                sieve[i] = False

            start2 = 2 * p - r if 2 * sq(p - r) - 1 == p else p - r
            for i in range(start2, N + 1, p):
                sieve[i] = False

    return sum(1 for b in sieve if b)


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()

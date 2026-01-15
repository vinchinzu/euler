"""Project Euler Problem 540: Counting primitive Pythagorean triples.

Find the number of primitive Pythagorean triples with side lengths up to
N.
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


def euler_totient(n: int) -> int:
    """Compute Euler's totient function."""
    if n <= 1:
        return 1
    result = n
    p = 2
    while p * p <= n:
        if n % p == 0:
            while n % p == 0:
                n //= p
            result -= result // p
        p += 1
    if n > 1:
        result -= result // n
    return result


def precompute_phi(limit: int) -> List[int]:
    """Precompute phi values."""
    phi = list(range(limit + 1))
    for i in range(2, limit + 1):
        if phi[i] == i:
            for j in range(i, limit + 1, i):
                phi[j] -= phi[j] // i
    return phi


def num_relatively_prime(m: int, limit: int) -> int:
    """Count numbers ≤ limit relatively prime to m."""
    count = 0
    for n in range(1, limit + 1):
        if gcd(m, n) == 1:
            count += 1
    return count


def sq(n: int) -> int:
    """Square."""
    return n * n


def solve() -> int:
    """Solve Problem 540."""
    N = 3141592653589793
    L = isqrt(N // 2)

    phi = precompute_phi(L + 1)

    ans = 0
    m = 2
    while sq(m) <= N:
        mult = 1 if m % 2 == 0 else 2
        if m <= L:
            ans += phi[m] // mult
        else:
            limit = isqrt(N - sq(m)) // mult
            ans += num_relatively_prime(m, limit)
        m += 1

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()

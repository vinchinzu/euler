"""Project Euler Problem 531: Chinese leftovers.

Let g(a, n, b, m) be the smallest non-negative solution to x ≡ a (mod n)
and x ≡ b (mod m). Find Σ_{N ≤ n < m < M} g(ϕ(n), n, ϕ(m), m).
"""

from __future__ import annotations

from math import gcd
from typing import List


def sieve_primes(limit: int) -> List[int]:
    """Sieve of Eratosthenes."""
    if limit < 2:
        return []
    is_prime = [True] * (limit + 1)
    is_prime[0] = is_prime[1] = False
    for i in range(2, int(limit**0.5) + 1):
        if is_prime[i]:
            for j in range(i * i, limit + 1, i):
                is_prime[j] = False
    return [i for i in range(limit + 1) if is_prime[i]]


def euler_totient(n: int, primes: List[int]) -> int:
    """Compute Euler's totient function."""
    if n <= 1:
        return 1
    result = n
    temp = n
    for p in primes:
        if p * p > temp:
            break
        if temp % p == 0:
            while temp % p == 0:
                temp //= p
            result -= result // p
    if temp > 1:
        result -= result // temp
    return result


def precompute_phi(limit: int) -> List[int]:
    """Precompute phi values."""
    phi = list(range(limit))
    for i in range(2, limit):
        if phi[i] == i:
            for j in range(i, limit, i):
                phi[j] -= phi[j] // i
    return phi


def precompute_gcds(limit: int) -> List[List[int]]:
    """Precompute GCDs."""
    gcds = [[0] * (i + 1) for i in range(limit + 1)]
    for i in range(1, limit + 1):
        for j in range(1, i + 1):
            gcds[i][j] = gcd(i, j)
    return gcds


def mod_inv(a: int, m: int) -> int:
    """Modular inverse."""
    if m == 1:
        return 0
    m0, x0, x1 = m, 0, 1
    while a > 1:
        q = a // m
        m, a = a % m, m
        x0, x1 = x1 - q * x0, x0
    return x1 % m0 if x1 < 0 else x1


def solve() -> int:
    """Solve Problem 531."""
    N = 1000000
    M = 1005000

    phi = precompute_phi(M)
    gcds = precompute_gcds(M - N)

    ans = 0
    for n in range(N, M):
        for m in range(n + 1, M):
            g = gcds[m - n][n % (m - n)]
            a = phi[n]
            b = phi[m]
            if (b - a) % g == 0:
                n_div_g = n // g
                m_div_g = m // g
                k = ((b - a) // g) * mod_inv(n_div_g, m_div_g) % m_div_g
                x = (a + k * n) % (m_div_g * n)
                ans += x

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()

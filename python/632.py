"""Project Euler Problem 632: Square prime factors.

Let C_k(N) be the number of integers between 1 and N inclusive that are
divisible by p² for exactly k primes p. Find the product of all nonzero C_k(N).
"""

from __future__ import annotations

from math import isqrt

from sympy import factorint, primerange


def ilog2(n: int) -> int:
    """Integer logarithm base 2."""
    result = 0
    while n > 1:
        n //= 2
        result += 1
    return result


def nCr(n: int, k: int, mod: int) -> list[list[int]]:
    """Precompute binomial coefficients modulo mod."""
    result = [[0] * (k + 1) for _ in range(n + 1)]
    for i in range(n + 1):
        result[i][0] = 1
        for j in range(1, min(i + 1, k + 1)):
            result[i][j] = (result[i - 1][j - 1] + result[i - 1][j]) % mod
    return result


def mobius_sieve(limit: int) -> list[int]:
    """Möbius function sieve."""
    mobius = [1] * (limit + 1)
    is_prime = [True] * (limit + 1)
    for i in range(2, limit + 1):
        if is_prime[i]:
            for j in range(i, limit + 1, i):
                is_prime[j] = False
                mobius[j] *= -1
            for j in range(i * i, limit + 1, i * i):
                mobius[j] = 0
    return mobius


def omega_sieve(limit: int) -> list[int]:
    """Number of distinct prime factors."""
    omega = [0] * (limit + 1)
    is_prime = [True] * (limit + 1)
    for i in range(2, limit + 1):
        if is_prime[i]:
            for j in range(i, limit + 1, i):
                is_prime[j] = False
                omega[j] += 1
    return omega


def solve() -> int:
    """Solve Problem 632."""
    N = 10**16
    M = 10**9 + 7
    L = isqrt(N)

    mobius = mobius_sieve(L)
    nCrs_table = nCr(ilog2(L), ilog2(L), M)
    omegas = omega_sieve(L)

    C = [0] * (ilog2(L) + 1)
    C[0] = N % M

    for n in range(2, isqrt(N) + 1):
        if mobius[n] == 0:
            continue
        k = omegas[n]
        count = (N // (n * n)) % M
        for i in range(k + 1):
            parity = 1 if i % 2 == 0 else -1
            C[k - i] = (C[k - i] + parity * nCrs_table[k][i] * count) % M

    ans = 1
    for num in C:
        if num != 0:
            ans = (ans * num) % M
    return ans % M


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()

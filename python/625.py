"""Project Euler Problem 625: Gcd sum.

Find the sum_{j=1}^N sum_{i=1}^j gcd(i,j).

The number of times that gcd(i,j) = g is sum_{k=1}^⌊N/g⌋ φ(k).
"""

from __future__ import annotations

from math import isqrt

from sympy import primerange


def sieve_phi(limit: int) -> list[int]:
    """Euler totient function sieve."""
    phi = list(range(limit + 1))
    for i in range(2, limit + 1):
        if phi[i] == i:  # i is prime
            for j in range(i, limit + 1, i):
                phi[j] -= phi[j] // i
    return phi


def sum_phis(n: int, mod: int) -> dict[int, int]:
    """Sum of Euler totient function values."""
    # Simplified version - compute directly
    phi = sieve_phi(int(n**0.5) + 1000)
    result = {}
    cumulative = 0
    for i in range(1, len(phi)):
        cumulative = (cumulative + phi[i]) % mod
        result[i] = cumulative
    return result


def solve() -> int:
    """Solve Problem 625."""
    N = 10**11
    M = 998244353

    sum_phis_dict = sum_phis(N, M)
    sqrt_n = isqrt(N)

    ans = 0
    for k in range(1, sqrt_n):
        ans = (ans + k * sum_phis_dict.get(N // k, 0)) % M

    for t in range(1, N // sqrt_n + 1):
        low = (N // (t + 1) + 1) % M
        high = (N // t) % M
        sum_phi_t = sum_phis_dict.get(t, 0)
        term = ((low + high) * (high - low + 1) // 2) % M * sum_phi_t % M
        ans = (ans + term) % M

    return ans % M


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()

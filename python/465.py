"""Project Euler Problem 465: Polygons with visible boundaries.

Find the number of polygons with all vertices on lattice points with coordinate
magnitudes up to N, and where the origin is strictly inside the locus of points
from which the entire polygon's boundary is visible.
"""

from __future__ import annotations

from math import gcd, isqrt
from typing import List


def pre_phi(limit: int) -> List[int]:
    """Precompute Euler's totient function."""
    phi = list(range(limit + 1))
    for i in range(2, limit + 1):
        if phi[i] == i:  # i is prime
            for j in range(i, limit + 1, i):
                phi[j] = phi[j] // i * (i - 1)
    return phi


def pow_mod(base: int, exp: int, mod: int) -> int:
    """Modular exponentiation."""
    result = 1
    base %= mod
    while exp > 0:
        if exp % 2 == 1:
            result = (result * base) % mod
        base = (base * base) % mod
        exp //= 2
    return result


def sq(n: int, mod: int) -> int:
    """Square modulo mod."""
    return (n * n) % mod


class QuotientValues:
    """Helper for quotient-based summations."""

    def __init__(self, n: int, mod: int) -> None:
        """Initialize with sum of phi values."""
        self.n = n
        self.mod = mod
        phi = pre_phi(n)
        self.sum_phi = [0] * (n + 1)
        for i in range(1, n + 1):
            self.sum_phi[i] = (self.sum_phi[i - 1] + phi[i]) % mod

    def div(self, q: int) -> int:
        """Sum phi(x) for x where floor(n/x) = q."""
        if q == 0:
            return 0
        upper = self.n // q
        lower = self.n // (q + 1) + 1
        if lower > upper:
            return 0
        return (self.sum_phi[upper] - self.sum_phi[lower - 1]) % self.mod


def solve() -> int:
    """Solve Problem 465."""
    N = 7**13
    M = 10**9 + 7
    L = isqrt(N)
    phi = pre_phi(N // L + 1)

    sum_phis1 = QuotientValues(N, M - 1)
    sum_phis2 = QuotientValues(N, M)

    T = 1
    for x in range(1, N // L + 1):
        T = (T * pow_mod(N // x + 1, phi[x], M)) % M
    for q in range(1, L):
        diff = (sum_phis1.div(q) - sum_phis1.div(q + 1)) % (M - 1)
        T = (T * pow_mod(q + 1, diff, M)) % M

    ans = (pow_mod(T, 8, M) - 1) % M
    ans = (ans - (sq(2 * N + 1, M) - 1) * pow_mod(T, 4, M)) % M
    for x in range(1, N // L + 1):
        ans = (ans + 4 * phi[x] * sq(N // x, M)) % M
    for q in range(1, L):
        diff = (sum_phis2.div(q) - sum_phis2.div(q + 1)) % M
        ans = (ans + 4 * diff * sq(q, M)) % M

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()

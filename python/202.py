"""Project Euler Problem 202: Laserbeam.

Three mirrors are placed in an equilateral triangle, with infinitesimal holes
at each vertex. Find the number of ways that a laser beam can enter through
a vertex, bounce exactly N times, then exit through the same vertex.

Uses coordinate system and inclusion-exclusion principle.
"""

from __future__ import annotations

from typing import Dict, List, Set


def lprime_factor(n: int) -> Dict[int, int]:
    """Return prime factorization of n as a dictionary."""
    factors: Dict[int, int] = {}
    d = 2
    while d * d <= n:
        while n % d == 0:
            factors[d] = factors.get(d, 0) + 1
            n //= d
        d += 1
    if n > 1:
        factors[n] = factors.get(n, 0) + 1
    return factors


def parity(n: int) -> int:
    """Return 1 if n is even, -1 if odd."""
    return 1 if n % 2 == 0 else -1


def solve() -> int:
    """Solve Problem 202."""
    N = 12017639147
    target = (N + 3) // 2
    prime_factors = list(lprime_factor(target).keys())

    ans = 0
    for subset in range(1 << len(prime_factors)):
        prod = 1
        for i in range(len(prime_factors)):
            if (subset & (1 << i)) > 0:
                prod *= prime_factors[i]
        bit_count = bin(subset).count("1")
        ans += parity(bit_count) * (target // prod - 2) // 3

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()

"""Project Euler Problem 214: Totient Chains.

Find the sum of all primes p less than N such that the totient chain
p, ϕ(p), ϕ(ϕ(p)), ... 1 contains K terms.
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


def pre_phi(limit: int, spf: List[int]) -> List[int]:
    """Precompute Euler's totient function."""
    phi = list(range(limit + 1))
    for i in range(2, limit + 1):
        if spf[i] == i:  # i is prime
            for j in range(i, limit + 1, i):
                phi[j] = phi[j] // i * (i - 1)
    return phi


def is_prime(n: int, spf: List[int]) -> bool:
    """Check if n is prime."""
    return n > 1 and spf[n] == n


def solve() -> int:
    """Solve Problem 214."""
    N = 40_000_000
    K = 25

    spf = build_spf(N)
    phi = pre_phi(N, spf)

    chain_lens = [0] * N
    ans = 0

    for i in range(1, N):
        chain_lens[i] = chain_lens[phi[i]] + 1
        if is_prime(i, spf) and chain_lens[i] == K:
            ans += i

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()

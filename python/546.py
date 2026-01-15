"""Project Euler Problem 546: Floor Function Recurrence.

Let f_k(n) = Σ_{i=0}^n f_k(⌊i/k⌋). Find Σ_{k=2}^K f_k(N).

For a given k, define the cumulative sums F[0](n) = f_k(n) and
F[s](n) = Σ_{i=1}^n F[s-1](i). We have F[s](n) = 1 for n=0 and F[s](n) = 0
for n<0.

It turns out that we can write F[s](n) as a linear combination of
F[t](⌊n/k⌋-t) for 0≤t≤s+1, where the coefficients depend only on s, t, and
r=n%k.
"""

from __future__ import annotations

from dataclasses import dataclass
from functools import lru_cache
from typing import Dict


@dataclass(frozen=True)
class Key:
    """Cache key."""

    n: int
    k: int
    s: int


def imod(a: int, m: int) -> int:
    """Integer modulo."""
    return ((a % m) + m) % m


@lru_cache(maxsize=None)
def f(n: int, k: int, s: int, mod: int) -> int:
    """Compute f_k(n) with cumulative sum level s."""
    if n == 0:
        return 1
    if n < 0:
        return 0

    # Compute coefficients c[s][t][r]
    c = [[0] * k for _ in range(s + 2)]
    for r in range(k):
        c[0][r] = 1

    for ss in range(s + 1):
        for t in range(ss, -1, -1):
            for r in range(1, k):
                c[t][r] = (c[t][r] + c[t][r - 1]) % mod
            for r in range(k):
                c[t + 1][r] = (c[t + 1][r] + c[t][k - 1]) % mod

    result = 0
    r = imod(n, k)
    for t in range(s + 2):
        if t < len(c) and r < len(c[t]):
            result = (result + c[t][r] * f(n // k - t, k, t, mod)) % mod

    return result


def solve() -> int:
    """Solve Problem 546."""
    N = 10**14
    K = 10
    M = 10**9 + 7

    ans = 0
    for k in range(2, K + 1):
        ans = (ans + f(N, k, 0, M)) % M

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()

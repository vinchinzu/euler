"""Project Euler Problem 759: A Squared Recurrence.

Find Σ_{n=1}^N f(n)² where f(1) = 1, f(2n) = 2f(n), and
f(2n+1) = 2n+1 + 2f(n) + f(n)/n.

By induction we can see that f(n) = n * b(n), where b(n) is the number
of 1s in the binary representation of n: f(2n) = 2n*b(n) = 2n*b(2n), and
f(2n+1) = 2n+1 + 2n*b(n) + n*b(n)/n = (2n+1) (b(n)+1) = (2n+1)b(2n+1).

We can compute the general form:

S(n,K,L) = Σ_{i=0}^n i^K b(i)^L
         = Σ_{i=0, i odd}^n i^K b(i)^L + Σ_{i=0, i even}^n i^K b(i)^L
         = Σ_{i=0}^⌊(n-1)/2⌋ (2i+1)^K (b(i)+1)^L + Σ_{i=0}^⌊n/2⌋ (2i)^K b(i)^L
         = Σ_{k=0}^K Σ_{l=0}^L 2^k nCr(K,k) nCr(L,l) S(⌊(n-1)/2⌋,k,l)
           + S(⌊n/2⌋,K,L).

The requested expression is S(N,2,2).
"""

from __future__ import annotations

from functools import lru_cache
from typing import Dict, Tuple


def pow_mod(base: int, exp: int, mod: int) -> int:
    """Modular exponentiation."""
    result = 1
    base %= mod
    while exp > 0:
        if exp & 1:
            result = (result * base) % mod
        base = (base * base) % mod
        exp >>= 1
    return result


def nCr(n: int, k: int) -> int:
    """Binomial coefficient."""
    if k < 0 or k > n:
        return 0
    if k == 0 or k == n:
        return 1
    k = min(k, n - k)
    result = 1
    for i in range(k):
        result = result * (n - i) // (i + 1)
    return result


def solve() -> int:
    """Solve Problem 759."""
    N = 10**16
    M = 10**9 + 7

    @lru_cache(maxsize=None)
    def S(n: int, K: int, L: int) -> int:
        """Compute S(n, K, L) modulo M."""
        if n == 0:
            return 1 if (K == 0 and L == 0) else 0

        result = (pow_mod(2, K, M) * S(n // 2, K, L)) % M

        for k in range(K + 1):
            for l in range(L + 1):
                term = (
                    pow_mod(2, k, M)
                    * nCr(K, k)
                    * nCr(L, l)
                    * S((n - 1) // 2, k, l)
                )
                result = (result + term) % M

        return result

    return S(N, 2, 2)


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()

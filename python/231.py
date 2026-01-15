"""Project Euler Problem 231: The Prime Factorisation of Binomial Coefficients.

Find the sum of the terms in the prime factorization of nCr(N, K).
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


def solve() -> int:
    """Solve Problem 231."""
    N = 20_000_000
    K = 15_000_000

    spf = build_spf(N)
    ans = 0

    for i in range(N - K):
        # Numerator: N - i
        n = N - i
        while n > 1:
            ans += spf[n]
            n //= spf[n]

        # Denominator: i + 1
        n = i + 1
        while n > 1:
            ans -= spf[n]
            n //= spf[n]

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()

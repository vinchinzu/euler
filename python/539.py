"""Project Euler Problem 539: Odd elimination.

Let P(n) be the last number remaining if we start with the numbers from
1 to n, and repeatedly remove every other number from the left starting
from the first, then remove every other number from the right starting
from the last, and so on. Find sum_{k=1}^N P(k).
"""

from __future__ import annotations

import sys
from typing import Dict

sys.setrecursionlimit(200)


def solve() -> int:
    """Solve Problem 539."""
    N = 10**18
    M = 987654321

    cache_p: Dict[int, int] = {}
    cache_s: Dict[int, int] = {}

    def tr(n: int) -> int:
        """Triangular number n*(n+1)/2 mod M."""
        return n % M * ((n + 1) % M) % M * pow(2, -1, M) % M

    def P(n: int) -> int:
        """Compute P(n)."""
        if n in cache_p:
            return cache_p[n]
        if n <= 1:
            return n
        if n % 2 == 1:
            result = P(n - 1)
        else:
            result = n + 2 - 2 * P(n // 2)
        cache_p[n] = result
        return result

    def S(n: int) -> int:
        """Compute sum S(n) = sum_{k=1}^n P(k) mod M."""
        if n in cache_s:
            return cache_s[n]
        if n <= 1:
            return n % M

        if n % 2 == 0:
            result = (P(n) % M + S(n - 1)) % M
        else:
            half = n // 2
            result = (1 + 2 * (2 * tr(half) + 2 * (half % M) - 2 * S(half))) % M
        cache_s[n] = result
        return result

    return S(N) % M


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()

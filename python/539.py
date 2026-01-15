"""Project Euler Problem 539: Odd elimination.

Let P(n) be the last number remaining if we start with the numbers from
1 to n, and repeatedly remove every other number from the left starting
from the first, then remove every other number from the right starting
from the last, and so on. Find Σ_{k=1}^N P(k).
"""

from __future__ import annotations

from typing import Dict


def tr(n: int) -> int:
    """Triangular number."""
    return n * (n + 1) // 2


def P(n: int, cache: Dict[int, int]) -> int:
    """Compute P(n)."""
    if n in cache:
        return cache[n]
    if n <= 1:
        return 1
    if n % 2 == 1:
        result = P(n - 1, cache)
    else:
        result = n + 2 - 2 * P(n // 2, cache)
    cache[n] = result
    return result


def S(n: int, cache_p: Dict[int, int], cache_s: Dict[int, int]) -> int:
    """Compute sum S(n) = Σ_{k=1}^n P(k)."""
    if n in cache_s:
        return cache_s[n]
    if n <= 1:
        return 1

    if n % 2 == 1:
        result = S(n - 1, cache_p, cache_s) + P(n, cache_p)
    else:
        half = n // 2
        result = (
            1
            + 2
            * (
                2 * tr(half)
                + 2 * half
                - 2 * S(half, cache_p, cache_s)
            )
        )
    cache_s[n] = result
    return result


def solve() -> int:
    """Solve Problem 539."""
    N = 10**18
    M = 987654321

    cache_p: Dict[int, int] = {}
    cache_s: Dict[int, int] = {}
    return S(N, cache_p, cache_s) % M


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()

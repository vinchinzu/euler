"""Project Euler Problem 542: Geometric Progression.

Let S(n) be the maximum sum of a geometric progression with at least 3
distinct terms ≤ n. Find Σ_{k=4}^N (-1)^k S(k).

Suppose the largest term is r*k^e. It's clear that we should choose a ratio
(k-1)/k in favor of any other ratio (k-i)/k. Forming a geometric
progression with e+1 terms gives a sum of:

S = r (k^{e+1} - (k-1)^{e+1}).

Since for a given (e, k), the ratio of r*k^e to the sum is the same, we
choose the maximum possible r = ⌊n/k^e⌋.
"""

from __future__ import annotations

from functools import lru_cache
from math import log2
from typing import Dict


def ilog2(n: int) -> int:
    """Integer logarithm base 2."""
    return int(log2(n)) if n > 0 else 0


def parity(n: int) -> int:
    """Return (-1)^n."""
    return 1 if n % 2 == 0 else -1


@lru_cache(maxsize=None)
def S(n: int) -> int:
    """Maximum sum of geometric progression ≤ n."""
    if n < 3:
        return 0
    
    max_S = 0
    max_e = ilog2(n)
    
    for e in range(max_e, 1, -1):
        if (e + 1) * n < max_S:
            break
        k = 2
        while pow(k, e) <= 2 * n:
            r = n // pow(k, e)
            if r > 0:
                sum_val = (pow(k, e + 1) - pow(k - 1, e + 1)) * r
                max_S = max(max_S, sum_val)
            k += 1
    
    return max_S


def T(low: int, high: int) -> int:
    """Sum of (-1)^k S(k) from low to high-1."""
    if low + 1 == high:
        return 0 if (low + high) % 2 == 0 else parity(low) * S(low)
    
    if S(low) == S(high):
        # All values in range have same S, compute sum
        count = high - low
        if count % 2 == 0:
            return 0
        return parity(low) * S(low)
    
    mid = (low + high) // 2
    return T(low, mid) + T(mid, high)


def solve() -> int:
    """Solve Problem 542."""
    N = 10**17
    return T(4, N + 1)


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()

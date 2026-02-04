"""Project Euler Problem 256: Tatami-Free Rooms.

Find the smallest s such that there are exactly N Tatami-free rooms with
area s.

For a room a*b (a<=b, a>2), it is tatami-free iff ceil((b-1)/(a+1)) > (b+1)//(a-1).
Equivalently, no integer exists in the interval ((b-1)/(a+1), (b+1)/(a-1)).

The tatami-free b values for a given a form gaps between "bad" intervals.
Bad interval for integer k: [k*(a-1)-1, k*(a+1)+1].
Gap between k and k+1: [k*(a+1)+2, (k+1)*(a-1)-2].
Gap size = a - 2k - 4, positive when k <= (a-5)/2.
"""

from __future__ import annotations

import numpy as np
from math import isqrt


def solve() -> int:
    """Solve Problem 256."""
    N = 200
    L = 10**8

    # For each a >= 3 and each gap k, the tatami-free b values form
    # contiguous ranges. The corresponding s = a*b values are
    # a*lo, a*(lo+1), ..., a*hi, i.e., a*lo to a*hi with step a.
    # We use numpy to bulk-increment counts for these ranges.
    counts = np.zeros(L + 1, dtype=np.int16)

    sq = isqrt(L)
    for a in range(3, sq + 1):
        max_b = L // a
        max_k = (a - 5) // 2
        for k in range(max_k + 1):
            lo = k * (a + 1) + 2
            hi = (k + 1) * (a - 1) - 2
            if lo < a:
                lo = a
            if hi > max_b:
                hi = max_b
            if lo > hi:
                continue
            # Increment counts for s = a*lo, a*(lo+1), ..., a*hi
            counts[a * lo: a * hi + 1: a] += 1

    # Find smallest s with exactly N tatami-free pairs
    candidates = np.where(counts == N)[0]
    if len(candidates) == 0:
        return 0
    return int(candidates[0])


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()

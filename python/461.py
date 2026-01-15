"""Project Euler Problem 461: Almost Pi.

Let f_n(k) = e^{k/n} - 1. Find a² + b² + c² + d² for the tuple (a, b, c, d)
that minimizes |f_n(a) + f_n(b) + f_n(c) + f_n(d) - π|.
"""

from __future__ import annotations

import math
from typing import List


def sq(n: int) -> int:
    """Square."""
    return n * n


def solve() -> int:
    """Solve Problem 461."""
    N = 10_000
    f = [math.exp(i / N) - 1 for i in range(2 * N)]

    # Compute all pairs
    vals: List[float] = []
    for k1 in range(1, N):
        if f[k1] >= math.pi:
            break
        for k2 in range(k1, N):
            if f[k1] + f[k2] >= math.pi:
                break
            vals.append(f[k1] + f[k2])

    vals.sort()

    # Two-pointer algorithm
    start = 0
    end = len(vals) - 1
    min_error = float("inf")
    min_start = -1
    min_end = -1

    while start < end:
        error = vals[start] + vals[end] - math.pi
        if abs(error) < min_error:
            min_error = abs(error)
            min_start = start
            min_end = end
        if error < 0:
            start += 1
        else:
            end -= 1

    # Find the pairs that sum to these values
    ans = 0
    target1 = vals[min_start]
    target2 = vals[min_end]

    for k1 in range(1, N):
        if f[k1] >= math.pi:
            break
        for k2 in range(k1, N):
            if f[k1] + f[k2] >= math.pi:
                break
            sum_val = f[k1] + f[k2]
            if abs(sum_val - target1) < 1e-10 or abs(sum_val - target2) < 1e-10:
                ans += sq(k1) + sq(k2)

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()

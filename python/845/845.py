#!/usr/bin/env python3
"""Project Euler Problem 845 — Nth integer with prime digit sum.

Find the N-th positive integer whose digit sum is prime (N = 10^16).
Binary search on the answer m, counting how many integers up to m have
a prime digit sum via a recursive, memoized digit-counting function.
"""

import sys

sys.setrecursionlimit(10000)


def solve():
    N = 10**16
    B = 10

    # Precompute small primes — max digit sum for a 17-digit number is 9*17 = 153
    sieve = [False, False] + [True] * 199
    for i in range(2, 15):
        if sieve[i]:
            for j in range(i * i, 201, i):
                sieve[j] = False

    cache = {}

    def count(mx, s):
        """Count of integers in [0..mx] whose digit sum + s is prime."""
        if mx <= 0:
            return 1 if mx == 0 and sieve[s] else 0
        key = (mx, s)
        if key in cache:
            return cache[key]
        total = 0
        r = mx % B
        q = mx // B
        for d in range(B):
            total += count(q - (1 if d > r else 0), s + d)
        cache[key] = total
        return total

    low, high = 0, (1 << 62)
    while low + 1 < high:
        mid = (low + high) // 2
        if count(mid, 0) < N:
            low = mid
        else:
            high = mid

    return high


if __name__ == "__main__":
    print(solve())

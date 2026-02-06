#!/usr/bin/env python3
"""
Project Euler Problem 383: Divisibility comparison between factorials

Let f(n) = largest x such that 5^x | n. Find the number of positive integers
i <= 10^18 such that f((2i-1)!) < 2*f(i!).

By Legendre's formula, f(n!) = floor(n/5) + floor(n/25) + ...
We need: sum of (2*floor(i/5^k) - floor((2i-1)/5^k)) over k >= 1 to be > 0.

This gives a digit-recursion in base 5: for each remainder r (0..4), evaluate the
first term of the difference and recurse on floor(i/5) with an updated diff value.
Memoize on (n, diff, totalDiff).
"""
import math


def solve():
    N = 10**18
    K = 5
    cache = {}

    def T(n, diff, total_diff):
        if n <= 0:
            return 1 if (n == 0 and total_diff > 0) else 0

        key = (n, diff, total_diff)
        if key in cache:
            return cache[key]

        result = 0
        for r in range(K):
            new_diff = -math.floor((2 * r - diff) / K)
            n_next = n // K - (1 if r > n % K else 0)
            result += T(n_next, new_diff, total_diff + new_diff)

        cache[key] = result
        return result

    return T(N, 1, 0) - 1


if __name__ == "__main__":
    print(solve())

#!/usr/bin/env python3
"""
Project Euler Problem 384: Rudin-Shapiro sequence

The Rudin-Shapiro sequence a(n) counts adjacent pairs of 1s in binary(n).
Let s(n) = sum_{k=0}^{n} (-1)^{a(k)} (summatory function).
Let g(t, c) = index where value t appears for the c-th time in s(0), s(1), ...
Find sum_{t=2}^{45} g(F(t+1), F(t)), where F is the Fibonacci sequence.

Key insight: s(n) and count(n, val) can be computed recursively by exploiting the
binary structure. For n near 2^k, the (-1)^a sequence splits into "same" and
"negated" halves. Binary search finds g(t, c).
"""
import math


def solve():
    N = 45
    s_cache = {}
    count_cache = {}

    def s(n):
        """Summatory Rudin-Shapiro: s(n) = sum_{k=0}^{n} (-1)^{a(k)}."""
        if n in s_cache:
            return s_cache[n]
        if n <= 1:
            return n + 1
        x = 1 << (n.bit_length() - 1)  # highest power of 2 <= n
        if n >= x + x // 2:
            result = s(x - 1) + 2 * s(x // 2 - 1) - s(n - x)
        else:
            result = s(x - 1) + s(n - x)
        s_cache[n] = result
        return result

    def count(n, val):
        """Count occurrences of value val in s(0), s(1), ..., s(n)."""
        key = (n, val)
        if key in count_cache:
            return count_cache[key]
        if val < 0 or val > 6 * math.isqrt(n + 1):
            return 0
        if n == -1:
            return 1 if val == 0 else 0
        x = 1 << ((n + 1).bit_length() - 1)  # highest power of 2 <= n+1
        c = count(x - 2, val)
        if n >= x + x // 2:
            s_x = s(x - 1)
            s_mid = s(x + x // 2 - 1)
            mirror_val = 2 * s_mid - s_x - val
            c += count(x // 2 - 2, val - s_x) + count(n - x, mirror_val) - count(x // 2 - 2, mirror_val)
        else:
            c += count(n - x, val - s(x - 1))
        count_cache[key] = c
        return c

    def g(t, c):
        """Find the index where value t appears for the c-th time in s(0), s(1), ..."""
        lo, hi = 0, (1 << 63)
        while lo + 1 < hi:
            mid = (lo + hi) // 2
            if count(mid, t) < c:
                lo = mid
            else:
                hi = mid
        return hi

    def fibonacci(n):
        a, b = 0, 1
        for _ in range(n):
            a, b = b, a + b
        return a

    total = 0
    for t in range(2, N + 1):
        total += g(fibonacci(t + 1), fibonacci(t))
    return total


if __name__ == "__main__":
    print(solve())

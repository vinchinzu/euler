"""Project Euler Problem 747: Triangular Pizza.

Let Ψ(n) be the number of ways to choose a point in a triangular pizza and
make n straight cuts from that point to the boundary such that it forms n
triangular pieces of the same area. Find Σ_{n=3}^N Ψ(n).

There are three ways to cut the pizza:
1. Make three cuts to the three corners: nCr(n-2, 2) ways
2. Make a cut to two corners and extend: n-2 ways
3. Make a cut to two corners with collinear cuts: iterate over (a,b) pairs
"""

from __future__ import annotations

from math import isqrt


def ncr(n: int, r: int, mod: int) -> int:
    """Binomial coefficient C(n, r) modulo mod."""
    if r < 0 or r > n:
        return 0
    if r == 0 or r == n:
        return 1
    result = 1
    for i in range(min(r, n - r)):
        result = (result * (n - i) * pow(i + 1, mod - 2, mod)) % mod
    return result


def tr(n: int) -> int:
    """Triangular number: n*(n+1)//2."""
    return n * (n + 1) // 2


def sq(n: int) -> int:
    """Square of n."""
    return n * n


def is_sq(n: int) -> bool:
    """Check if n is a perfect square."""
    root = isqrt(n)
    return root * root == n


def solve() -> int:
    """Solve Problem 747."""
    n = 10**8
    m = 10**9 + 7

    ans = ncr(n, 3, m) + 6 * tr(n - 2) % m

    for a in range(1, isqrt(2 * n) + 1):
        min_n = sq(2 * a + 1)
        if min_n <= n:
            ans = (ans + 6 * (n - min_n) + 3) % m

        b = a + 1
        while True:
            min_n = (a + 1) * (b + 1) + a * b + isqrt(
                4 * (a + 1) * (b + 1) * a * b
            )
            if min_n > n:
                break
            ans = (ans + 12 * (n - min_n)) % m
            if is_sq(4 * (a + 1) * (b + 1) * a * b):
                ans = (ans + 6) % m
            b += 1

    return ans % m


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()

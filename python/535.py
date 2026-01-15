"""Project Euler Problem 535: Fractal Sequence.

Define a sequence S such that if the first occurrence of each integer is
circled, then the circled numbers are consecutive integers starting with 1,
every non-circled number a_i has ⌊√a_i⌋ circled numbers before it, and the
sequence of non-circled numbers is equal to the sequence S itself (the
fractal property). Find T = Σ_{i=1}^N S_i.
"""

from __future__ import annotations

from functools import lru_cache
from math import isqrt


def sq(n: int) -> int:
    """Square."""
    return n * n


def tr(n: int, mod: int) -> int:
    """Triangular number modulo mod."""
    return (n * (n + 1) // 2) % mod


def sum_powers(n: int, k: int) -> int:
    """Sum of k-th powers from 1 to n."""
    if k == 1:
        return n * (n + 1) // 2
    if k == 2:
        return n * (n + 1) * (2 * n + 1) // 6
    result = 0
    for i in range(1, n + 1):
        result += pow(i, k)
    return result


@lru_cache(maxsize=None)
def f(n: int) -> int:
    """Number of non-circled numbers in first n terms."""
    if n == 0:
        return 0
    low = 0
    high = n
    while low + 1 < high:
        mid = (low + high) // 2
        bound = 2 * pow(mid / 2, 1.5) / 3
        if bound <= n and sum_sqrts(mid) + mid <= n:
            low = mid
        else:
            high = mid
    return low


@lru_cache(maxsize=None)
def sum_sqrts(n: int) -> int:
    """Sum of floor square roots."""
    if n == 0:
        return 0
    f_n = f(n)
    l = isqrt(n - f_n)
    return (
        sum_sqrts(f_n)
        + (n - f_n - sq(l) + 1) * l
        + 2 * sum_powers(l - 1, 2)
        + sum_powers(l - 1, 1)
    )


def T(n: int, mod: int) -> int:
    """Sum of first n terms of sequence S."""
    if n == 0:
        return 0
    return (T(f(n), mod) + tr(n - f(n), mod)) % mod


def solve() -> int:
    """Solve Problem 535."""
    N = 10**18
    M = 10**9
    return T(N, M)


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()

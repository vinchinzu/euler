"""Project Euler Problem 294: Sum of digits - experience #23.

Find the number of positive integers with up to N digits that are divisible
by 23 and whose digit sum is 23.

Given two integers a, b with respective remainders r1, r2 (mod 23) and
respective digit sums s1, s2, and if b consists of n digits, then the
concatenated integer a.b has remainder r1(10^n) + r2 (mod 23) and digit sum
s1 + s2.

Among one digit numbers, there is one integer with remainder d and sum d for
every 0≤d≤9. Given a table f1(r1, s1) of how many m digit numbers have
remainder r1 and digit sum s1, and a corresponding table f2(r2, s2) for n
digit numbers, we can compute a table f(r, s) for m+n digit numbers. We use
repeated doubling to construct a table for N digit numbers, and read off the
entry for r=0 and s=23.
"""

from __future__ import annotations

from typing import List


def pow_mod(base: int, exp: int, mod: int) -> int:
    """Modular exponentiation."""
    if mod == 1:
        return 0
    result = 1
    base %= mod
    while exp > 0:
        if exp % 2 == 1:
            result = (result * base) % mod
        exp //= 2
        base = (base * base) % mod
    return result


def combine(
    f1: List[List[int]], f2: List[List[int]], n: int, K: int, B: int, M: int
) -> List[List[int]]:
    """Combine two tables."""
    mult = pow_mod(B, n, K)
    f: List[List[int]] = [[0] * (K + 1) for _ in range(K)]
    for r1 in range(K):
        for s1 in range(K + 1):
            for r2 in range(K):
                for s2 in range(K + 1):
                    if s1 + s2 <= K:
                        r = (r1 * mult + r2) % K
                        f[r][s1 + s2] = (
                            f[r][s1 + s2] + f1[r1][s1] * f2[r2][s2]
                        ) % M
    return f


def helper(
    n: int, single: List[List[int]], K: int, B: int, M: int
) -> List[List[int]]:
    """Recursive helper."""
    if n == 1:
        return single
    table = helper(n // 2, single, K, B, M)
    table = combine(table, table, n // 2, K, B, M)
    if n % 2 == 1:
        table = combine(table, single, 1, K, B, M)
    return table


def solve() -> int:
    """Solve Problem 294."""
    N = 11**12
    K = 23
    M = 10**9
    B = 10

    # Single digit table
    single: List[List[int]] = [[0] * (K + 1) for _ in range(K)]
    for d in range(B):
        single[d][d] = 1

    table = helper(N, single, K, B, M)
    return table[0][K]


def main() -> None:
    """Main entry point."""
    result = solve()
    print(result)


if __name__ == "__main__":
    main()

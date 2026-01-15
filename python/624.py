"""Project Euler Problem 624: Two heads are better than one.

Find the probability a/b that the first occurrence of two consecutive heads ends
after a multiple of N coin flips.

If P(k, n) is the probability that the first occurrence of two consecutive heads
occurs after k (mod n) coin flips, then we have P(2,n) = 1 + P(0,n)/4 +
P(1,n)/2, and P(k,n) = P(k-2,n)/4 + P(k-1,n)/2 for all k. This leads to a
matrix equation. Let D_{n-1} be determinant of the bottom left (n-1)x(n-1)
matrix. Then by the definition of the determinant, we can obtain the recurrence
relations:
D_n = -2D_{n-1} + 4D_{n-2}
a = (-1)^n (4D_{n-2} - 1)
b = 4^n + (-1)^n (2D_{n-1} - 8D_{n-2} + 1).
"""

from __future__ import annotations

import numpy as np


def mod_inverse(a: int, m: int) -> int:
    """Modular inverse using extended Euclidean algorithm."""
    if m == 1:
        return 0
    t, new_t = 0, 1
    r, new_r = m, a % m
    while new_r != 0:
        q = r // new_r
        t, new_t = new_t, t - q * new_t
        r, new_r = new_r, r - q * new_r
    if r != 1:
        raise ValueError("Modular inverse does not exist")
    if t < 0:
        t += m
    return t


def pow_mod(base: int, exp: int, mod: int) -> int:
    """Modular exponentiation."""
    result = 1
    base %= mod
    while exp > 0:
        if exp & 1:
            result = (result * base) % mod
        base = (base * base) % mod
        exp >>= 1
    return result


def matrix_pow_mod(matrix: list[list[int]], exp: int, mod: int) -> list[list[int]]:
    """Matrix exponentiation modulo mod."""
    n = len(matrix)
    result = [[1 if i == j else 0 for j in range(n)] for i in range(n)]
    base = [row[:] for row in matrix]
    while exp > 0:
        if exp & 1:
            new_result = [[0] * n for _ in range(n)]
            for i in range(n):
                for j in range(n):
                    for k in range(n):
                        new_result[i][j] = (new_result[i][j] + result[i][k] * base[k][j]) % mod
            result = new_result
        new_base = [[0] * n for _ in range(n)]
        for i in range(n):
            for j in range(n):
                for k in range(n):
                    new_base[i][j] = (new_base[i][j] + base[i][k] * base[k][j]) % mod
        base = new_base
        exp >>= 1
    return result


def solve() -> int:
    """Solve Problem 624."""
    N = 10**18
    M = 10**9 + 9

    A = [[-2, 4], [1, 0]]
    A_pow_n_minus_1 = matrix_pow_mod(A, N - 1, M)
    A_pow_n_minus_2 = matrix_pow_mod(A, N - 2, M)

    d1 = A_pow_n_minus_1[0][0]
    d2 = A_pow_n_minus_2[0][0]

    parity_n = 1 if N % 2 == 0 else -1
    a = (parity_n * (4 * d2 - 1)) % M
    b = (pow_mod(4, N, M) + parity_n * (2 * d1 - 8 * d2 + 1)) % M

    ans = (a * mod_inverse(b, M)) % M
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()

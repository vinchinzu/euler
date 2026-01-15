"""Project Euler Problem 458: Permutations of Project.

Find the number of strings of length N consisting of K letters, that do not
have a substring of length K consisting of one of each letter.
"""

from __future__ import annotations

from typing import List


def pow_mod(base: int, exp: int, mod: int) -> int:
    """Modular exponentiation."""
    result = 1
    base %= mod
    while exp > 0:
        if exp % 2 == 1:
            result = (result * base) % mod
        base = (base * base) % mod
        exp //= 2
    return result


def matrix_multiply(
    a: List[List[int]], b: List[List[int]], mod: int
) -> List[List[int]]:
    """Multiply two matrices modulo mod."""
    n = len(a)
    result = [[0] * n for _ in range(n)]
    for i in range(n):
        for k in range(n):
            if a[i][k] != 0:
                for j in range(n):
                    result[i][j] = (result[i][j] + a[i][k] * b[k][j]) % mod
    return result


def matrix_power(matrix: List[List[int]], exp: int, mod: int) -> List[List[int]]:
    """Matrix exponentiation."""
    n = len(matrix)
    result = [[1 if i == j else 0 for j in range(n)] for i in range(n)]
    base = [row[:] for row in matrix]

    while exp > 0:
        if exp % 2 == 1:
            result = matrix_multiply(result, base, mod)
        base = matrix_multiply(base, base, mod)
        exp //= 2

    return result


def solve() -> int:
    """Solve Problem 458."""
    N = 10**12
    K = 7
    M = 10**9

    # Build transition matrix
    A = [[0] * K for _ in range(K)]
    for n in range(1, K):
        A[n][n - 1] = K - n + 1
        for i in range(n, K):
            A[n][i] = 1

    An = matrix_power(A, N, M)
    ans = 0
    for row in An:
        ans = (ans + row[0]) % M

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()

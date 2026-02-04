"""Project Euler Problem 594: Rhombus tilings of an octagon.

Find the number of ways to tile an octagon with sides of alternating
lengths A,B,A,B,A,B,A,B with unit squares and unit 45° rhombi.

See Theorem 1 in the paper "A formula for the number of tilings of an
octagon by rhombi" for the formula implemented here.
"""

from __future__ import annotations

from itertools import product
from math import comb as _comb
from typing import List


def nCr(n: int, k: int) -> int:
    """Binomial coefficient that returns 0 for invalid inputs."""
    if k < 0 or n < 0 or k > n:
        return 0
    return _comb(n, k)


def det(matrix: List[List[int]]) -> int:
    """Compute determinant of a matrix (simplified for small matrices)."""
    n = len(matrix)
    if n == 1:
        return matrix[0][0]
    if n == 2:
        return matrix[0][0] * matrix[1][1] - matrix[0][1] * matrix[1][0]
    # For larger matrices, use a simple recursive approach
    result = 0
    for j in range(n):
        minor = [[matrix[i][k] for k in range(n) if k != j] for i in range(1, n)]
        sign = 1 if j % 2 == 0 else -1
        result += sign * matrix[0][j] * det(minor)
    return result


def solve() -> int:
    """Solve Problem 594."""
    A = 4
    B = 2

    ans = 0

    # Generate all possible index combinations
    for indices in product(range((A + 1) ** 2), repeat=B * B):
        x = [[0] * (B + 2) for _ in range(B + 2)]
        y = [[0] * (B + 2) for _ in range(B + 2)]

        # Set boundary conditions
        for k in range(1, B + 1):
            x[B + 1][k] = A
            x[k][B + 1] = A
            y[0][k] = A
            y[k][B + 1] = A

        # Set interior values
        for i in range(1, B + 1):
            for j in range(1, B + 1):
                idx = (i - 1) * B + (j - 1)
                val = indices[idx]
                x[i][j] = val // (A + 1)
                y[i][j] = val % (A + 1)

        # Compute number of tilings
        num_tilings = 1
        for u in range(1, B + 2):
            M = [[0] * B for _ in range(B)]
            P = [[0] * B for _ in range(B)]

            for i in range(1, B + 1):
                for j in range(1, B + 1):
                    M[i - 1][j - 1] = nCr(
                        x[j][u] - x[i][u - 1] + y[j][u] - y[i][u - 1],
                        x[j][u] - x[i][u - 1] + j - i,
                    )
                    P[i - 1][j - 1] = nCr(
                        x[u][j] - x[u - 1][i] + y[u - 1][i] - y[u][j],
                        x[u][j] - x[u - 1][i] + j - i,
                    )

            num_tilings *= det(M) * det(P)

        ans += num_tilings

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()

"""Project Euler Problem 825: Chasing Game.

Two cars start on opposite sides of a track of length 2n, facing the same
direction. They alternate turns, where on each turn a car advances either
1, 2, or 3 (with equal probability), until one car catches up with another
car, and the moving car wins. If S(n) is the difference between the
winning probabilities of the two cars, find Σ_{n=2}^N S(n).

We can compute the probabilities for small values of n using a system of
equations in P_k, the probability that a car wins if it is about to move
and k away from the other car. The probabilities are rational fractions
a_n/b_n, where a_n and b_n are linear recurrences with characteristic
polynomials (x²-4x+1)(x-1) and (x²-4x+1)² respectively, and we can use
standard methods to find closed formulas for them:

a_n = (3+√3)/2 (2-√3)^n + (3-√3)/2 (2+√3)^n - 2(-1)^n
b_n = (3+√3)/2 n (2-√3)^n - 1/2 (2-√3)^n + (3-√3)/2 n (2+√3)^n - 1/2 (2+√3)^n

For large values of n, all terms other than the ones with (2+√3)^n are
negligible, so this simplifies to

a_n ≈ 1 / (n - 1/(3-√3)).

We compute a_n for small n directly by solving the linear system, and then
add the rest of the terms using the formula for a Harmonic sequence.
"""

from __future__ import annotations

import math
from typing import List


def solve_linear_system(A: List[List[float]], B: List[float]) -> List[float]:
    """Solve linear system Ax = B using Gaussian elimination."""
    n = len(A)
    # Create augmented matrix
    aug = [row + [B[i]] for i, row in enumerate(A)]

    # Forward elimination
    for i in range(n):
        # Find pivot
        max_row = i
        for k in range(i + 1, n):
            if abs(aug[k][i]) > abs(aug[max_row][i]):
                max_row = k
        aug[i], aug[max_row] = aug[max_row], aug[i]

        # Eliminate
        for k in range(i + 1, n):
            factor = aug[k][i] / aug[i][i]
            for j in range(i, n + 1):
                aug[k][j] -= factor * aug[i][j]

    # Back substitution
    x = [0.0] * n
    for i in range(n - 1, -1, -1):
        x[i] = aug[i][n]
        for j in range(i + 1, n):
            x[i] -= aug[i][j] * x[j]
        x[i] /= aug[i][i]

    return x


def harmonic(n: float) -> float:
    """Approximate harmonic number."""
    gamma = 0.5772156649015329
    return gamma + math.log(n) + 1 / (2 * n) - 1 / (12 * n * n)


def S(n: int) -> float:
    """Compute S(n)."""
    # Build linear system
    A: List[List[float]] = [[0.0] * (2 * n) for _ in range(2 * n)]
    B: List[float] = [1.0] * (2 * n)

    for i in range(2 * n):
        A[i][i] = 1.0
        for j in range(1, 4):
            if j < i:
                A[i][2 * n - i + j] += 1.0 / 3.0

    X = solve_linear_system(A, B)
    return 2 * X[n] - 1


def solve() -> float:
    """Solve Problem 825."""
    N = 10**14
    L = 100

    ans = 0.0
    for n in range(2, L + 1):
        ans += S(n)

    # Add harmonic series approximation for large n
    sqrt3 = math.sqrt(3)
    offset = 1 / (3 - sqrt3)
    ans += harmonic(N) - harmonic(L - offset)

    return ans


def main() -> float:
    """Main entry point."""
    result = solve()
    print(f"{result:.8f}")
    return result


if __name__ == "__main__":
    main()

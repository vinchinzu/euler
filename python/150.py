"""Project Euler Problem 150.

Find the smallest possible sum of a sub-triangle in a 1000-row triangular grid.

Algorithm (from Java reference):
Uses a coordinate system (i, j) for the triangle where you start at top,
move i steps down-right, then j steps down-left. Precomputes rightSums and
leftSums arrays for cache-friendly access when computing triangle sums.

The sum of numbers k rows below (i,j) in the equilateral triangle with
top vertex at (i,j) equals R(i, j+k) - L(j, i+k).
"""

import numpy as np
from numba import njit


@njit(cache=True)
def solve():
    N = 1000

    # Generate sequence S[k] for k in 1..N*(N+1)/2
    total = N * (N + 1) // 2
    S = np.zeros(total + 1, dtype=np.int64)
    t = 0
    for k in range(1, total + 1):
        t = (615949 * t + 797807) % (1 << 20)
        S[k] = t - (1 << 19)

    # rightSums[j][i] and leftSums[i][j] - matching Java array layout
    rightSums = np.zeros((N, N), dtype=np.int64)
    leftSums = np.zeros((N, N), dtype=np.int64)

    for i in range(N):
        for j in range(N - i):
            row = i + j
            idx = row * (row + 1) // 2 + i + 1
            if i > 0:
                rightSums[j, i] = rightSums[j + 1, i - 1] + S[idx]
            else:
                rightSums[j, i] = S[idx]

            if i > 0:
                leftSums[i, j] = leftSums[i - 1, j + 1] + S[row * (row + 1) // 2 + i]
            else:
                leftSums[i, j] = 0

    # Find minimum triangle sum
    ans = 1 << 62
    for i in range(N):
        for j in range(N - i):
            totalSum = 0
            for k in range(N - i - j):
                # Java: rightSums[i][j + k] - leftSums[j][i + k]
                totalSum += rightSums[i, j + k] - leftSums[j, i + k]
                if totalSum < ans:
                    ans = totalSum

    return ans


def main() -> int:
    return int(solve())


if __name__ == "__main__":
    print(main())

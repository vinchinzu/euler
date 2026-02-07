#!/usr/bin/env python3
"""Project Euler 782 - Distinct Rows and Columns.

The complexity of an n x n binary matrix is |{distinct rows} U {distinct columns}|.
c(n,k) = minimum complexity of an n x n binary matrix with exactly k ones.
C(n) = sum_{k=0}^{n^2} c(n,k).

Formula: C(n) = 3*n^2 - 1 - N2 + N4
where N2 = count of k in [1,n^2-1] with c(n,k)=2 (block matrices)
      N4 = count of k in [1,n^2-1] with c(n,k)>=4 (not achievable with comp<=3)

Key constructions for comp<=3:
1. Products: k = d*m for 1 <= d,m <= n-1 (2-type block matrix)
2. Complements: if k is achievable, so is n^2-k (flip all bits)
3. 3x3 kernel matrices: block matrices with 3 row/column groups where
   every column pattern equals some row pattern. These give quadratic
   forms k = sum M[i][j]*n_i*n_j on the simplex n_0+n_1+n_2=n.
"""

import numpy as np
import itertools


def solve(n=10000):
    N = n * n

    # Sieve: achievable[k] = 1 if c(n,k) <= 3
    achievable = np.zeros(N + 1, dtype=np.uint8)
    achievable[0] = 1
    achievable[N] = 1

    # S2: comp=2 values from 2x2 block matrices
    S2 = set()
    for c in range(1, n):
        S2.add(c * c)
        S2.add(N - c * c)
    for x in range(1, n):
        y = n - x
        S2.add(x * x + y * y)
        S2.add(2 * x * y)
    S2.discard(0)
    S2.discard(N)
    N2 = len(S2)
    for k in S2:
        achievable[k] = 1

    # Construction 1: Products d*m with 1 <= d,m <= n-1
    for d in range(1, n):
        achievable[d:d * n:d] = 1

    # Construction 2: Complement symmetry (c(n,k) = c(n,n^2-k))
    achievable_rev = achievable[::-1].copy()
    np.maximum(achievable, achievable_rev, out=achievable)
    del achievable_rev

    # Construction 3: Kernel 3x3 matrices
    # Enumerate all 3x3 binary matrices where every column is also a row
    rows_3 = list(itertools.product([0, 1], repeat=3))
    kernel_forms = set()
    for r0 in rows_3:
        for r1 in rows_3:
            for r2 in rows_3:
                M = [r0, r1, r2]
                cols = [tuple(M[i][j] for i in range(3)) for j in range(3)]
                if all(col in M for col in cols):
                    A = M[0][0]; B = M[1][1]; C = M[2][2]
                    D01 = M[0][1] + M[1][0]
                    D02 = M[0][2] + M[2][0]
                    D12 = M[1][2] + M[2][1]
                    # k(a,b) = aa*a^2 + bb*b^2 + ab*a*b + a1*a + b1*b + c0
                    # where c = n - a - b
                    aa = A + C - D02
                    bb = B + C - D12
                    ab = D01 + 2 * C - D02 - D12
                    a1 = n * (D02 - 2 * C)
                    b1 = n * (D12 - 2 * C)
                    c0 = C * n * n
                    kernel_forms.add((aa, bb, ab, a1, b1, c0))

    for (aa, bb, ab, a1, b1, c0) in kernel_forms:
        for a in range(n + 1):
            b_max = n - a
            if b_max < 0:
                continue
            b = np.arange(b_max + 1, dtype=np.int64)
            k = aa * a * a + bb * b * b + ab * a * b + a1 * a + b1 * b + c0
            valid = (k > 0) & (k < N)
            if valid.any():
                achievable[k[valid]] = 1

    N4 = int(np.sum(achievable[1:N] == 0))
    return 3 * N - 1 - N2 + N4


if __name__ == "__main__":
    print(solve())

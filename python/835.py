"""Project Euler Problem 835: Supernatural Triangles.

Find the sum of the perimeters of all supernatural triangles (triangles
where two sides are consecutive integers) with perimeter <= 10^N.

First suppose one of the legs and the hypotenuse are consecutive, so the
side lengths are a, b, b+1. Then a^2=2b+1, so the sides are parameterized
as a=2t+1, b=2t^2+2t for t>=1. The perimeter is 4t^2+6t+2, and the
quadratic formula tells us the maximum t is (sqrt(1+4*10^N) - 3) / 4,
which for even N is 10^{N/2}/2 - 1.

Now suppose the two legs are consecutive, so the side lengths are a, a+1, c.
Then we have the Pell equation a^2+(a+1)^2=c^2 => (2a+1)^2-2c^2=-1, and
the perimeter is (2a+1)+c. Since this is a Pell equation, the solutions
satisfy a recurrence p_t = 6p_{t-1}-p_{t-2} with p_1=2. The prefix sum
S(n) = sum(p_1..p_n) satisfies S(n) = 7S(n-1) - 7S(n-2) + S(n-3),
which we evaluate via matrix exponentiation. We subtract 2 (degenerate
triangle 0,1,1) and 12 (triangle 3,4,5 already counted in first case).
"""

from __future__ import annotations

import math


def mat_mul(A: list, B: list, mod: int) -> list:
    """Multiply two matrices modulo mod."""
    n = len(A)
    m = len(B[0])
    k = len(B)
    C = [[0] * m for _ in range(n)]
    for i in range(n):
        for j in range(m):
            s = 0
            for l in range(k):
                s += A[i][l] * B[l][j]
            C[i][j] = s % mod
    return C


def mat_pow(M_mat: list, exp: int, mod: int) -> list:
    """Matrix exponentiation modulo mod."""
    n = len(M_mat)
    result = [[1 if i == j else 0 for j in range(n)] for i in range(n)]
    base = [row[:] for row in M_mat]
    while exp > 0:
        if exp & 1:
            result = mat_mul(result, base, mod)
        base = mat_mul(base, base, mod)
        exp >>= 1
    return result


def solve() -> int:
    """Solve Problem 835."""
    N = 10**10
    M = 1234567891
    B = 10

    inv2 = pow(2, -1, M)
    inv6 = pow(6, -1, M)

    ans = 0

    # First case: leg and hypotenuse consecutive
    # limit1 = 10^(N/2) / 2 - 1, computed mod M
    limit1 = (pow(B, N // 2, M) * inv2 - 1) % M

    # sum of 4t^2+6t+2 for t=1..limit1, mod M
    # = 4 * sum(t^2) + 6 * sum(t) + 2 * limit1
    # sum(t^2) = n(n+1)(2n+1)/6 mod M (using modular inverse)
    # sum(t) = n(n+1)/2 mod M
    n = limit1
    sum_t = n * (n + 1) % M * inv2 % M
    sum_t2 = n * (n + 1) % M * (2 * n + 1) % M * inv6 % M

    ans = (4 * sum_t2 + 6 * sum_t + 2 * n) % M

    # Second case: two legs consecutive (Pell equation)
    # p_t = 6*p_{t-1} - p_{t-2}, p_0=0, p_1=2
    # S(n) = sum(p_1..p_n) satisfies S(n) = 7S(n-1) - 7S(n-2) + S(n-3)
    # S(0)=0, S(1)=2, S(2)=14

    sqrt2 = math.sqrt(2)
    log_base = math.log(3 + 2 * sqrt2)
    limit2 = int((N * math.log(B) + math.log(2 * sqrt2)) / log_base)

    if limit2 == 0:
        S_limit2 = 0
    elif limit2 == 1:
        S_limit2 = 2
    elif limit2 == 2:
        S_limit2 = 14
    else:
        # Matrix exponentiation for the recurrence
        # [S(n), S(n-1), S(n-2)] = M_rec^(n-2) * [S(2), S(1), S(0)]
        rec_mat = [[7, -7, 1], [1, 0, 0], [0, 1, 0]]
        result = mat_pow(rec_mat, limit2 - 2, M)
        S_limit2 = (result[0][0] * 14 + result[0][1] * 2) % M

    # Subtract 2 (degenerate 0,1,1) and 12 (triangle 3,4,5 counted in first case)
    ans = (ans + S_limit2 - 14) % M

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()

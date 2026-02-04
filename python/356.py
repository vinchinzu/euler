#!/usr/bin/env python3
"""Project Euler Problem 356 - Largest Roots of Cubic Polynomials

For each i, g(x) = x^3 - 2^i * x^2 + i = 0.
a_i is the largest root. Compute sum floor(a_i^K) mod 10^8 for K=987654321, i=1..30.

Key insight: S_k = a^k + b^k + c^k satisfies S_k = 2^i * S_{k-1} - i * S_{k-3}
and S_k is always an integer. Since |b|,|c| < 1 for i >= 2,
floor(a^K) = S_K - 1 if b^K+c^K > 0, else S_K.

Use matrix exponentiation mod 10^8 for each i.
"""

MOD = 100_000_000
K = 987_654_321

def mat_mul(A, B, mod):
    """Multiply two 3x3 matrices mod m."""
    n = 3
    C = [[0]*n for _ in range(n)]
    for i in range(n):
        for j in range(n):
            s = 0
            for k in range(n):
                s += A[i][k] * B[k][j]
            C[i][j] = s % mod
    return C

def mat_pow(M, p, mod):
    """Compute M^p mod m using fast exponentiation."""
    n = 3
    # Identity matrix
    result = [[1 if i==j else 0 for j in range(n)] for i in range(n)]
    base = [row[:] for row in M]
    while p > 0:
        if p & 1:
            result = mat_mul(result, base, mod)
        base = mat_mul(base, base, mod)
        p >>= 1
    return result

def compute_SK_mod(i, K, mod):
    """Compute S_K mod m where S_k = 2^i * S_{k-1} - i * S_{k-3}.

    S_0 = 3, S_1 = 2^i, S_2 = (2^i)^2 = 2^(2i) (since S_2 = 2^i * S_1 - 0 * S_0 - i * S_{-1}...)

    Actually: S_0 = 3, S_1 = p = 2^i, S_2 = p^2 - 2q = (2^i)^2 - 0 = 2^(2i).
    Wait, for x^3 - px^2 + qx - r with p=2^i, q=0, r=-i:
    S_0 = 3
    S_1 = p = 2^i
    S_2 = p*S_1 - 2q = (2^i)^2
    General: S_k = p*S_{k-1} - q*S_{k-2} + r*S_{k-3} = 2^i * S_{k-1} - i * S_{k-3}
    """
    p = pow(2, i, mod)
    mi = i % mod  # i mod m (for the -i coefficient)

    if K == 0:
        return 3 % mod
    if K == 1:
        return p
    if K == 2:
        return (p * p) % mod

    # State vector: [S_k, S_{k-1}, S_{k-2}]
    # Transition: [S_{k+1}, S_k, S_{k-1}] = [[2^i, 0, -i], [1, 0, 0], [0, 1, 0]] * [S_k, S_{k-1}, S_{k-2}]

    S0 = 3 % mod
    S1 = p
    S2 = (p * p) % mod

    # Matrix [[2^i, 0, -i], [1, 0, 0], [0, 1, 0]]
    # But -i mod m:
    neg_i = (-i) % mod

    M = [[p, 0, neg_i],
         [1, 0, 0],
         [0, 1, 0]]

    # We have state at k=2: [S2, S1, S0]
    # After multiplying by M^(K-2), we get state at k=K: [SK, S_{K-1}, S_{K-2}]

    Mpow = mat_pow(M, K - 2, mod)

    # Result = Mpow * [S2, S1, S0]^T
    SK = (Mpow[0][0] * S2 + Mpow[0][1] * S1 + Mpow[0][2] * S0) % mod

    return SK

def sign_of_bc_power(i, K):
    """Determine the sign of b^K + c^K for cubic x^3 - 2^i x^2 + i.

    For i >= 2: b and c are real with |b|, |c| < 1 (for i >= 3 this is clear,
    for i=2 need to check).

    b + c = 2^i - a_i (small positive)
    b * c = -i / a_i (negative)

    So one root is positive and one is negative.
    Let b > 0 > c with b > |c| (since b+c > 0).

    For K odd: b^K + c^K. Since b > |c| and K is odd, b^K > |c|^K, so b^K + c^K > 0.

    For i = 1: roots are (1+sqrt(5))/2, 1, (1-sqrt(5))/2.
    b = 1, c = (1-sqrt(5))/2 ≈ -0.618.
    b^K + c^K = 1 + (-0.618)^K.
    For K odd: 1 - 0.618^K > 0. So positive.

    Actually wait, for i=1, |b|=1 which is not < 1. So b^K doesn't go to 0.
    b^K + c^K = 1 + ((1-sqrt(5))/2)^K.
    For K odd (very large): ((1-sqrt(5))/2)^K ≈ 0 (alternating, decreasing).
    So b^K + c^K ≈ 1. Then a^K = S_K - (1 + c^K) ≈ S_K - 1.
    floor(a^K) = S_K - 1 - floor_adj where floor_adj depends on fractional part.

    Hmm, for i=1 we need more careful analysis.

    Returns +1 if b^K + c^K > 0, -1 if < 0, 0 if = 0.
    """
    # For all i from 1 to 30, and K odd:
    # The sum b^K + c^K > 0.
    #
    # Proof sketch for i >= 2:
    # b + c > 0 and b*c < 0, so |b| > |c|, b > 0 > c.
    # K is odd, so b^K > 0 > c^K, and |b^K| > |c^K|.
    # Hence b^K + c^K > 0.
    #
    # For i=1: b=1, c=(1-sqrt(5))/2. b^K + c^K = 1 + c^K.
    # |c| < 1 so |c^K| < 1. Since K is odd, c^K < 0. So 0 < b^K + c^K < 1.
    # Actually c^K < 0 since c < 0 and K is odd. And |c^K| < 1. So b^K + c^K = 1 + c^K in (0, 1).

    return 1  # positive for all i=1..30 with K odd

def solve():
    total = 0
    for i in range(1, 31):
        SK = compute_SK_mod(i, K, MOD)
        sgn = sign_of_bc_power(i, K)
        if sgn > 0:
            floor_aK = (SK - 1) % MOD
        else:
            floor_aK = SK
        total = (total + floor_aK) % MOD
    return total

if __name__ == "__main__":
    print(solve())

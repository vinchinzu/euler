"""Project Euler Problem 402: Integer-valued Polynomials.

M(a,b,c) = max m dividing n^4+a*n^3+b*n^2+c*n for all integers n.
S(N) = sum_{0<a,b,c<=N} M(a,b,c).
Find last 9 digits of sum_{k=2}^{1234567890123} S(F_k).

Approach:
1. M(a,b,c) = gcd(24, 6(6+a), 2(7+3a+b), 1+a+b+c) and depends only on a,b,c mod 24.
2. S(N) is a cubic polynomial in N for each N mod 24 (quasipolynomial with period 24).
3. Pisano period of 24 is 24, so F_k mod 24 depends only on k mod 24.
4. Group k by k mod 24. For each group, F_k advances by 24 Fibonacci steps per term.
5. Track Fibonacci pairs (F_k, F_{k+1}) and monomial products up to degree 3 using
   an 11x11 matrix, accumulating 288*S(F_k) (integer polynomial) in the cumsum slot.
6. Work modulo 288*10^9 so that dividing cumsum by 288 gives the answer mod 10^9.
"""
from math import gcd
from fractions import Fraction

MOD = 10 ** 9
WMOD = 288 * MOD  # Working modulus for exact integer division by 288


def precompute():
    """Precompute M_table, Fibonacci mod 24, and integer polynomial coefficients."""
    # M(a,b,c) depends only on a%24, b%24, c%24
    M_tab = [[[0] * 24 for _ in range(24)] for _ in range(24)]
    for a in range(24):
        for b in range(24):
            for c in range(24):
                p1 = 1 + a + b + c
                p2 = 16 + 8 * a + 4 * b + 2 * c
                p3 = 81 + 27 * a + 9 * b + 3 * c
                p4 = 256 + 64 * a + 16 * b + 4 * c
                M_tab[a][b][c] = gcd(gcd(p1, p2), gcd(p3, p4))

    # S(N) via residue counts
    def count_res(N, j):
        q, r = divmod(N, 24)
        return q if j == 0 else q + (1 if j <= r else 0)

    def S_exact(N):
        if N <= 0:
            return 0
        cnt = [count_res(N, j) for j in range(24)]
        t = 0
        for a in range(24):
            if not cnt[a]:
                continue
            for b in range(24):
                if not cnt[b]:
                    continue
                for c in range(24):
                    if not cnt[c]:
                        continue
                    t += cnt[a] * cnt[b] * cnt[c] * M_tab[a][b][c]
        return t

    # Fibonacci mod 24 (Pisano period = 24)
    fmod24 = [0, 1]
    for i in range(2, 26):
        fmod24.append((fmod24[-1] + fmod24[-2]) % 24)

    # For each s in 0..23, fit 288*S(N) = e0 + e1*N + e2*N^2 + e3*N^3
    i288 = {}
    for s in range(24):
        xs = [24, 48, 72, 96] if s == 0 else [s, s + 24, s + 48, s + 72]
        ys = [S_exact(x) for x in xs]
        A = [[Fraction(xs[i] ** j) for j in range(4)] for i in range(4)]
        bv = [Fraction(y) for y in ys]
        for col in range(4):
            piv = col
            while A[piv][col] == 0:
                piv += 1
            A[col], A[piv] = A[piv], A[col]
            bv[col], bv[piv] = bv[piv], bv[col]
            for row in range(4):
                if row == col:
                    continue
                f = A[row][col] / A[col][col]
                for j in range(4):
                    A[row][j] -= f * A[col][j]
                bv[row] -= f * bv[col]
        i288[s] = tuple(int((bv[i] / A[i][i]) * 288) for i in range(4))

    return fmod24, i288


def m2x2(A, B, p):
    return [
        [
            (A[0][0] * B[0][0] + A[0][1] * B[1][0]) % p,
            (A[0][0] * B[0][1] + A[0][1] * B[1][1]) % p,
        ],
        [
            (A[1][0] * B[0][0] + A[1][1] * B[1][0]) % p,
            (A[1][0] * B[0][1] + A[1][1] * B[1][1]) % p,
        ],
    ]


def p2x2(M, e, p):
    r = [[1, 0], [0, 1]]
    b = [row[:] for row in M]
    while e > 0:
        if e & 1:
            r = m2x2(r, b, p)
        b = m2x2(b, b, p)
        e >>= 1
    return r


def fmod_f(n, p):
    if n <= 0:
        return 0
    if n == 1:
        return 1 % p
    return p2x2([[1, 1], [1, 0]], n - 1, p)[0][0]


def mm(A, B, p):
    n = len(A)
    m_ = len(B[0])
    k = len(B)
    C = [[0] * m_ for _ in range(n)]
    for i in range(n):
        for q in range(k):
            if A[i][q]:
                for j in range(m_):
                    C[i][j] = (C[i][j] + A[i][q] * B[q][j]) % p
    return C


def mpow(M, e, p):
    n = len(M)
    r = [[1 if i == j else 0 for j in range(n)] for i in range(n)]
    b = [row[:] for row in M]
    while e > 0:
        if e & 1:
            r = mm(r, b, p)
        b = mm(b, b, p)
        e >>= 1
    return r


def build_T(aa, bb, cc, dd, e0, e1, e2, e3, p):
    """Build 11x11 transition matrix for tracking monomials of (G, H) up to degree 3
    and accumulating 288*S(G) in the cumsum slot.

    G' = cc*H + dd*G (new Fibonacci value)
    H' = aa*H + bb*G (next Fibonacci value)
    """
    T = [[0] * 11 for _ in range(11)]
    d, c, b, a = dd % p, cc % p, bb % p, aa % p
    # G'^3, G'^2*H', G'*H'^2, H'^3
    T[0] = [d*d%p*d%p, 3*d%p*d%p*c%p, 3*d%p*c%p*c%p, c*c%p*c%p, 0,0,0,0,0,0,0]
    T[1] = [b*d%p*d%p, (a*d%p*d%p+2*b%p*d%p*c%p)%p, (2*a%p*d%p*c%p+b*c%p*c%p)%p, a*c%p*c%p, 0,0,0,0,0,0,0]
    T[2] = [d*b%p*b%p, (2*d%p*a%p*b%p+c*b%p*b%p)%p, (d*a%p*a%p+2*c%p*a%p*b%p)%p, c*a%p*a%p, 0,0,0,0,0,0,0]
    T[3] = [b*b%p*b%p, 3*b%p*b%p*a%p, 3*b%p*a%p*a%p, a*a%p*a%p, 0,0,0,0,0,0,0]
    # G'^2, G'*H', H'^2
    T[4] = [0,0,0,0, d*d%p, 2*d%p*c%p, c*c%p, 0,0,0,0]
    T[5] = [0,0,0,0, b*d%p, (a*d%p+b*c%p)%p, a*c%p, 0,0,0,0]
    T[6] = [0,0,0,0, b*b%p, 2*a%p*b%p, a*a%p, 0,0,0,0]
    # G', H'
    T[7] = [0,0,0,0, 0,0,0, d, c, 0,0]
    T[8] = [0,0,0,0, 0,0,0, b, a, 0,0]
    # constant
    T[9] = [0,0,0,0, 0,0,0, 0,0, 1, 0]
    # cumsum += e3*G'^3 + e2*G'^2 + e1*G' + e0
    row10 = [0] * 11
    for j in range(4):
        row10[j] = (e3 * T[0][j]) % p
    for j in range(4, 7):
        row10[j] = (e2 * T[4][j]) % p
    for j in range(7, 9):
        row10[j] = (e1 * T[7][j]) % p
    row10[9] = e0 % p
    row10[10] = 1
    T[10] = row10
    for i in range(11):
        for j in range(11):
            T[i][j] = T[i][j] % p
    return T


def solve():
    N_TARGET = 1234567890123
    fmod24, i288 = precompute()

    A24 = p2x2([[1, 1], [1, 0]], 24, WMOD)
    aa, bb = A24[0]
    cc, dd = A24[1]

    total_288 = 0

    for i in range(24):
        s = fmod24[i]
        m_start = 0 if i >= 2 else 1
        m_end = (N_TARGET - i) // 24
        if m_end < m_start:
            continue
        num_terms = m_end - m_start + 1

        e = i288[s]
        e0, e1, e2, e3 = [x % WMOD for x in e]

        T = build_T(aa, bb, cc, dd, e0, e1, e2, e3, WMOD)

        k0 = i + 24 * m_start
        G0 = fmod_f(k0, WMOD)
        H0 = fmod_f(k0 + 1, WMOD)

        S0_288 = (e0 + e1 * G0 + e2 * G0 * G0 + e3 * G0 * G0 * G0) % WMOD

        state = [
            pow(G0, 3, WMOD),
            G0 * G0 % WMOD * H0 % WMOD,
            G0 * H0 % WMOD * H0 % WMOD,
            pow(H0, 3, WMOD),
            G0 * G0 % WMOD,
            G0 * H0 % WMOD,
            H0 * H0 % WMOD,
            G0,
            H0,
            1,
            S0_288,
        ]

        if num_terms == 1:
            total_288 = (total_288 + S0_288) % WMOD
            continue

        Tp = mpow(T, num_terms - 1, WMOD)
        val = 0
        for j in range(11):
            val = (val + Tp[10][j] * state[j]) % WMOD

        total_288 = (total_288 + val) % WMOD

    return (total_288 // 288) % MOD


if __name__ == "__main__":
    print(solve())

/*
 * Project Euler Problem 402: Integer-valued Polynomials.
 *
 * M(a,b,c) = max m dividing n^4+a*n^3+b*n^2+c*n for all integers n.
 * S(N) = sum_{0<a,b,c<=N} M(a,b,c).
 * Find last 9 digits of sum_{k=2}^{1234567890123} S(F_k).
 *
 * Approach:
 * 1. M(a,b,c) depends only on a,b,c mod 24.
 * 2. S(N) is a quasipolynomial with period 24 (cubic in N for each residue).
 * 3. Pisano period of Fib mod 24 is 24, so F_k mod 24 depends on k mod 24.
 * 4. Group k by k mod 24. For each group, use 11x11 matrix exponentiation
 *    tracking Fibonacci pairs and monomial products up to degree 3.
 * 5. Work modulo 288*10^9 for exact integer division.
 */
#include <stdio.h>
#include <string.h>

typedef long long ll;
typedef __int128 lll;

#define MOD_BASE 1000000000LL
#define WMOD (288LL * MOD_BASE)  /* 288000000000 */
#define DIM 11

static int M_tab[24][24][24];
static int fmod24[26];

/* Rational number as exact fraction for fitting */
typedef struct { ll num; ll den; } Frac;

static Frac frac_make(ll n, ll d) {
    Frac f; f.num = n; f.den = d;
    /* Simplify */
    ll a = n < 0 ? -n : n, b = d < 0 ? -d : d;
    while (b) { ll t = b; b = a % b; a = t; }
    if (a > 0) { f.num /= a; f.den /= a; }
    if (f.den < 0) { f.num = -f.num; f.den = -f.den; }
    return f;
}

static Frac frac_sub(Frac a, Frac b) {
    return frac_make(a.num * b.den - b.num * a.den, a.den * b.den);
}

static Frac frac_div(Frac a, Frac b) {
    return frac_make(a.num * b.den, a.den * b.num);
}

static Frac frac_mul(Frac a, Frac b) {
    return frac_make(a.num * b.num, a.den * b.den);
}

static Frac frac_from_ll(ll v) { Frac f; f.num = v; f.den = 1; return f; }

static ll gcd_ll(ll a, ll b) {
    if (a < 0) a = -a;
    if (b < 0) b = -b;
    while (b) { ll t = b; b = a % b; a = t; }
    return a;
}

/* 2x2 matrix multiply mod p using __int128 */
typedef struct { ll a[2][2]; } M2;

static M2 m2_mul(M2 A, M2 B, ll p) {
    M2 C;
    for (int i = 0; i < 2; i++)
        for (int j = 0; j < 2; j++) {
            lll s = 0;
            for (int k = 0; k < 2; k++)
                s += (lll)A.a[i][k] * B.a[k][j];
            C.a[i][j] = (ll)(s % p);
        }
    return C;
}

static M2 m2_pow(M2 M, ll e, ll p) {
    M2 R = {{{1, 0}, {0, 1}}};
    M2 B = M;
    while (e > 0) {
        if (e & 1) R = m2_mul(R, B, p);
        B = m2_mul(B, B, p);
        e >>= 1;
    }
    return R;
}

static ll fib_mod(ll n, ll p) {
    if (n <= 0) return 0;
    if (n == 1) return 1 % p;
    M2 F = {{{1, 1}, {1, 0}}};
    M2 R = m2_pow(F, n - 1, p);
    return R.a[0][0];
}

/* 11x11 matrix multiply mod p */
typedef struct { ll a[DIM][DIM]; } M11;

static M11 m11_mul(M11 A, M11 B, ll p) {
    M11 C;
    memset(&C, 0, sizeof(C));
    for (int i = 0; i < DIM; i++)
        for (int k = 0; k < DIM; k++) {
            if (A.a[i][k] == 0) continue;
            for (int j = 0; j < DIM; j++) {
                lll v = (lll)A.a[i][k] * B.a[k][j];
                C.a[i][j] = (ll)(((__int128)C.a[i][j] + v) % p);
            }
        }
    return C;
}

static M11 m11_pow(M11 M, ll e, ll p) {
    M11 R;
    memset(&R, 0, sizeof(R));
    for (int i = 0; i < DIM; i++) R.a[i][i] = 1;
    M11 B = M;
    while (e > 0) {
        if (e & 1) R = m11_mul(R, B, p);
        B = m11_mul(B, B, p);
        e >>= 1;
    }
    return R;
}

static int count_res(ll N, int j) {
    ll q = N / 24;
    int r = (int)(N % 24);
    if (j == 0) return (int)q;
    return (int)(q + (j <= r ? 1 : 0));
}

static ll S_exact(ll N) {
    if (N <= 0) return 0;
    int cnt[24];
    for (int j = 0; j < 24; j++) cnt[j] = count_res(N, j);
    ll t = 0;
    for (int a = 0; a < 24; a++) {
        if (!cnt[a]) continue;
        for (int b = 0; b < 24; b++) {
            if (!cnt[b]) continue;
            for (int c = 0; c < 24; c++) {
                if (!cnt[c]) continue;
                t += (ll)cnt[a] * cnt[b] * cnt[c] * M_tab[a][b][c];
            }
        }
    }
    return t;
}

/* Integer polynomial coefficients for 288*S(N) */
static ll i288[24][4];

static void precompute(void) {
    /* M_tab */
    for (int a = 0; a < 24; a++)
        for (int b = 0; b < 24; b++)
            for (int c = 0; c < 24; c++) {
                int p1 = 1 + a + b + c;
                int p2 = 16 + 8*a + 4*b + 2*c;
                int p3 = 81 + 27*a + 9*b + 3*c;
                int p4 = 256 + 64*a + 16*b + 4*c;
                int g1 = (int)gcd_ll(p1, p2);
                int g2 = (int)gcd_ll(p3, p4);
                M_tab[a][b][c] = (int)gcd_ll(g1, g2);
            }

    /* Fibonacci mod 24 */
    fmod24[0] = 0; fmod24[1] = 1;
    for (int i = 2; i < 26; i++)
        fmod24[i] = (fmod24[i-1] + fmod24[i-2]) % 24;

    /* Fit 288*S(N) as cubic polynomial for each residue class mod 24 */
    for (int s = 0; s < 24; s++) {
        ll xs[4], ys[4];
        if (s == 0) { xs[0]=24; xs[1]=48; xs[2]=72; xs[3]=96; }
        else { xs[0]=s; xs[1]=s+24; xs[2]=s+48; xs[3]=s+72; }
        for (int i = 0; i < 4; i++) ys[i] = S_exact(xs[i]);

        /* Gaussian elimination with fractions */
        Frac A[4][4], bv[4];
        for (int i = 0; i < 4; i++) {
            bv[i] = frac_from_ll(ys[i]);
            for (int j = 0; j < 4; j++) {
                ll xp = 1;
                for (int k = 0; k < j; k++) xp *= xs[i];
                A[i][j] = frac_from_ll(xp);
            }
        }
        for (int col = 0; col < 4; col++) {
            int piv = col;
            while (A[piv][col].num == 0) piv++;
            if (piv != col) {
                for (int j = 0; j < 4; j++) { Frac t = A[col][j]; A[col][j] = A[piv][j]; A[piv][j] = t; }
                Frac t = bv[col]; bv[col] = bv[piv]; bv[piv] = t;
            }
            for (int row = 0; row < 4; row++) {
                if (row == col) continue;
                Frac f = frac_div(A[row][col], A[col][col]);
                for (int j = 0; j < 4; j++)
                    A[row][j] = frac_sub(A[row][j], frac_mul(f, A[col][j]));
                bv[row] = frac_sub(bv[row], frac_mul(f, bv[col]));
            }
        }
        for (int i = 0; i < 4; i++) {
            Frac coeff = frac_div(bv[i], A[i][i]);
            /* Multiply by 288 */
            Frac c288 = frac_mul(coeff, frac_from_ll(288));
            i288[s][i] = c288.num / c288.den;
        }
    }
}

static M11 build_T(ll aa, ll bb, ll cc, ll dd, ll e0, ll e1, ll e2, ll e3, ll p) {
    M11 T;
    memset(&T, 0, sizeof(T));
    ll d = dd % p, c = cc % p, b = bb % p, a = aa % p;

#define MUL(x,y) ((lll)(x) * (y) % p)

    /* G'^3, G'^2*H', G'*H'^2, H'^3 */
    T.a[0][0] = MUL(MUL(d,d),d);
    T.a[0][1] = 3*MUL(MUL(d,d),c) % p;
    T.a[0][2] = 3*MUL(MUL(d,c),c) % p;
    T.a[0][3] = MUL(MUL(c,c),c);

    T.a[1][0] = MUL(MUL(b,d),d);
    T.a[1][1] = (MUL(MUL(a,d),d) + 2*MUL(MUL(b,d),c)) % p;
    T.a[1][2] = (2*MUL(MUL(a,d),c) + MUL(MUL(b,c),c)) % p;
    T.a[1][3] = MUL(MUL(a,c),c);

    T.a[2][0] = MUL(MUL(d,b),b);
    T.a[2][1] = (2*MUL(MUL(d,a),b) + MUL(MUL(c,b),b)) % p;
    T.a[2][2] = (MUL(MUL(d,a),a) + 2*MUL(MUL(c,a),b)) % p;
    T.a[2][3] = MUL(MUL(c,a),a);

    T.a[3][0] = MUL(MUL(b,b),b);
    T.a[3][1] = 3*MUL(MUL(b,b),a) % p;
    T.a[3][2] = 3*MUL(MUL(b,a),a) % p;
    T.a[3][3] = MUL(MUL(a,a),a);

    /* G'^2, G'*H', H'^2 */
    T.a[4][4] = MUL(d,d);
    T.a[4][5] = 2*MUL(d,c) % p;
    T.a[4][6] = MUL(c,c);

    T.a[5][4] = MUL(b,d);
    T.a[5][5] = (MUL(a,d) + MUL(b,c)) % p;
    T.a[5][6] = MUL(a,c);

    T.a[6][4] = MUL(b,b);
    T.a[6][5] = 2*MUL(a,b) % p;
    T.a[6][6] = MUL(a,a);

    /* G', H' */
    T.a[7][7] = d; T.a[7][8] = c;
    T.a[8][7] = b; T.a[8][8] = a;

    /* constant */
    T.a[9][9] = 1;

    /* cumsum row */
    for (int j = 0; j < 4; j++)
        T.a[10][j] = (lll)e3 * T.a[0][j] % p;
    for (int j = 4; j < 7; j++)
        T.a[10][j] = (lll)e2 * T.a[4][j] % p;
    for (int j = 7; j < 9; j++)
        T.a[10][j] = (lll)e1 * T.a[7][j] % p;
    T.a[10][9] = e0 % p;
    T.a[10][10] = 1;

#undef MUL

    for (int i = 0; i < DIM; i++)
        for (int j = 0; j < DIM; j++)
            T.a[i][j] = ((T.a[i][j] % p) + p) % p;
    return T;
}

int main(void) {
    ll N_TARGET = 1234567890123LL;
    precompute();

    M2 F24 = {{{1, 1}, {1, 0}}};
    M2 A24 = m2_pow(F24, 24, WMOD);
    ll aa = A24.a[0][0], bb = A24.a[0][1];
    ll cc = A24.a[1][0], dd = A24.a[1][1];

    ll total_288 = 0;

    for (int i = 0; i < 24; i++) {
        int s = fmod24[i];
        ll m_start = (i >= 2) ? 0 : 1;
        ll m_end = (N_TARGET - i) / 24;
        if (m_end < m_start) continue;
        ll num_terms = m_end - m_start + 1;

        ll e0 = ((i288[s][0] % WMOD) + WMOD) % WMOD;
        ll e1 = ((i288[s][1] % WMOD) + WMOD) % WMOD;
        ll e2 = ((i288[s][2] % WMOD) + WMOD) % WMOD;
        ll e3 = ((i288[s][3] % WMOD) + WMOD) % WMOD;

        M11 T = build_T(aa, bb, cc, dd, e0, e1, e2, e3, WMOD);

        ll k0 = i + 24 * m_start;
        ll G0 = fib_mod(k0, WMOD);
        ll H0 = fib_mod(k0 + 1, WMOD);

        ll S0_288 = ((lll)e0 + (lll)e1 * G0 % WMOD + (lll)e2 * ((lll)G0 * G0 % WMOD) % WMOD + (lll)e3 * ((lll)G0 * G0 % WMOD * G0 % WMOD) % WMOD) % WMOD;

        ll state[DIM];
        state[0] = (lll)G0 * G0 % WMOD * G0 % WMOD;
        state[1] = (lll)G0 * G0 % WMOD * H0 % WMOD;
        state[2] = (lll)G0 * H0 % WMOD * H0 % WMOD;
        state[3] = (lll)H0 * H0 % WMOD * H0 % WMOD;
        state[4] = (lll)G0 * G0 % WMOD;
        state[5] = (lll)G0 * H0 % WMOD;
        state[6] = (lll)H0 * H0 % WMOD;
        state[7] = G0;
        state[8] = H0;
        state[9] = 1;
        state[10] = S0_288;

        if (num_terms == 1) {
            total_288 = (total_288 + S0_288) % WMOD;
            continue;
        }

        M11 Tp = m11_pow(T, num_terms - 1, WMOD);
        ll val = 0;
        for (int j = 0; j < DIM; j++)
            val = (val + (lll)Tp.a[10][j] * state[j]) % WMOD;

        total_288 = (total_288 + val) % WMOD;
    }

    printf("%lld\n", (total_288 / 288) % MOD_BASE);
    return 0;
}

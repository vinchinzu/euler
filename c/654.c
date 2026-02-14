/*
 * Project Euler 654 - Neighbourly Constraints
 * T(N, K): sequences of N positive integers with consecutive sum <= K.
 * Uses Berlekamp-Massey + Kitamasa with NTT-based polynomial arithmetic.
 * N = 10^12, K = 5000.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef long long ll;
typedef unsigned long long ull;

#define MOD 1000000007LL

static ll power_mod(ll base, ll exp, ll mod) {
    ll r = 1;
    base %= mod;
    if (base < 0) base += mod;
    while (exp > 0) {
        if (exp & 1) r = r * base % mod;
        base = base * base % mod;
        exp >>= 1;
    }
    return r;
}

/* NTT for polynomial multiplication mod MOD = 10^9+7 */
/* MOD - 1 = 2 * 500000003 (500000003 is prime), so max NTT size is 2 */
/* We can't do NTT directly with this modulus. Use 3-moduli NTT + CRT instead. */

/* Use three NTT-friendly primes */
#define P1 998244353LL
#define P2 985661441LL
#define P3 754974721LL
/* g1=3, g2=3, g3=11 are primitive roots */

static ll pw(ll base, ll exp, ll mod) {
    ll r = 1;
    base %= mod;
    while (exp > 0) {
        if (exp & 1) r = r * base % mod;
        base = base * base % mod;
        exp >>= 1;
    }
    return r;
}

static void ntt(ll *a, int n, int inv, ll mod, ll g) {
    for (int i = 1, j = 0; i < n; i++) {
        int bit = n >> 1;
        for (; j & bit; bit >>= 1) j ^= bit;
        j ^= bit;
        if (i < j) { ll t = a[i]; a[i] = a[j]; a[j] = t; }
    }
    for (int len = 2; len <= n; len <<= 1) {
        ll w = inv ? pw(g, mod - 1 - (mod - 1) / len, mod) : pw(g, (mod - 1) / len, mod);
        for (int i = 0; i < n; i += len) {
            ll wn = 1;
            for (int j = 0; j < len / 2; j++) {
                ll u = a[i + j], v = a[i + j + len / 2] * wn % mod;
                a[i + j] = (u + v) % mod;
                a[i + j + len / 2] = (u - v + mod) % mod;
                wn = wn * w % mod;
            }
        }
    }
    if (inv) {
        ll inv_n = pw(n, mod - 2, mod);
        for (int i = 0; i < n; i++) a[i] = a[i] * inv_n % mod;
    }
}

/* Multiply two polynomials mod MOD using 3-prime NTT + CRT */
static ll *poly_mul(ll *a, int na, ll *b, int nb, int *nout) {
    int nc = na + nb - 1;
    int n = 1;
    while (n < nc) n <<= 1;
    *nout = nc;

    ll *a1 = (ll *)calloc(n, sizeof(ll));
    ll *b1 = (ll *)calloc(n, sizeof(ll));
    ll *a2 = (ll *)calloc(n, sizeof(ll));
    ll *b2 = (ll *)calloc(n, sizeof(ll));
    ll *a3 = (ll *)calloc(n, sizeof(ll));
    ll *b3 = (ll *)calloc(n, sizeof(ll));

    for (int i = 0; i < na; i++) { a1[i] = a[i] % P1; a2[i] = a[i] % P2; a3[i] = a[i] % P3; }
    for (int i = 0; i < nb; i++) { b1[i] = b[i] % P1; b2[i] = b[i] % P2; b3[i] = b[i] % P3; }

    ntt(a1, n, 0, P1, 3); ntt(b1, n, 0, P1, 3);
    ntt(a2, n, 0, P2, 3); ntt(b2, n, 0, P2, 3);
    ntt(a3, n, 0, P3, 11); ntt(b3, n, 0, P3, 11);

    for (int i = 0; i < n; i++) {
        a1[i] = a1[i] * b1[i] % P1;
        a2[i] = a2[i] * b2[i] % P2;
        a3[i] = a3[i] * b3[i] % P3;
    }

    ntt(a1, n, 1, P1, 3);
    ntt(a2, n, 1, P2, 3);
    ntt(a3, n, 1, P3, 11);

    /* CRT to combine */
    ll *res = (ll *)calloc(nc, sizeof(ll));
    ll inv12 = pw(P1, P2 - 2, P2);
    ll inv13 = pw(P1 * P2 % P3, P3 - 2, P3);

    for (int i = 0; i < nc; i++) {
        ll r1 = a1[i], r2 = a2[i], r3 = a3[i];
        ll x1 = r1;
        ll x2 = (r2 - x1 % P2 + P2) % P2 * inv12 % P2;
        ll val = (x1 + x2 % P3 * (P1 % P3) % P3) % P3;
        ll x3 = (r3 - val + P3) % P3 * inv13 % P3;

        /* Result = x1 + x2*P1 + x3*P1*P2 */
        __int128 result = x1;
        result += (__int128)x2 * P1;
        result += (__int128)x3 * P1 * P2;
        res[i] = (ll)(result % MOD);
    }

    free(a1); free(b1); free(a2); free(b2); free(a3); free(b3);
    return res;
}

/* Polynomial mod: compute p mod char_poly (monic of degree d) */
static ll *poly_mod(ll *p, int np, ll *cp, int d) {
    if (np <= d) {
        ll *res = (ll *)calloc(d, sizeof(ll));
        for (int i = 0; i < np; i++) res[i] = p[i];
        return res;
    }

    /* Long division */
    ll *r = (ll *)calloc(np, sizeof(ll));
    for (int i = 0; i < np; i++) r[i] = p[i];

    for (int i = np - 1; i >= d; i--) {
        ll coeff = r[i] % MOD;
        if (coeff == 0) continue;
        for (int j = 0; j < d; j++) {
            r[i - d + j] = (r[i - d + j] - coeff * cp[j] % MOD + MOD) % MOD;
        }
        r[i] = 0;
    }

    ll *res = (ll *)calloc(d, sizeof(ll));
    for (int i = 0; i < d; i++) res[i] = r[i];
    free(r);
    return res;
}

/* Berlekamp-Massey */
static int berlekamp_massey(ll *s, int len, ll **out_C) {
    ll *C = (ll *)calloc(len + 2, sizeof(ll));
    ll *B = (ll *)calloc(len + 2, sizeof(ll));
    C[0] = 1;
    B[0] = 1;
    int Clen = 1, Blen = 1;
    int L = 0, m = 1;
    ll b = 1;

    for (int n = 0; n < len; n++) {
        ll d = s[n];
        for (int j = 1; j <= L; j++) {
            d = (d + C[j] * s[n - j]) % MOD;
        }
        d = (d % MOD + MOD) % MOD;

        if (d == 0) {
            m++;
        } else if (2 * L <= n) {
            ll *T = (ll *)calloc(Clen, sizeof(ll));
            for (int i = 0; i < Clen; i++) T[i] = C[i];
            int Tlen = Clen;

            ll coef = d * power_mod(b, MOD - 2, MOD) % MOD;
            int new_len = Blen + m;
            if (new_len > Clen) {
                C = (ll *)realloc(C, (new_len + 1) * sizeof(ll));
                for (int i = Clen; i <= new_len; i++) C[i] = 0;
                Clen = new_len;
            }
            for (int i = 0; i < Blen; i++) {
                C[i + m] = (C[i + m] - coef * B[i] % MOD + MOD) % MOD;
            }

            L = n + 1 - L;
            free(B);
            B = T;
            Blen = Tlen;
            b = d;
            m = 1;
        } else {
            ll coef = d * power_mod(b, MOD - 2, MOD) % MOD;
            int new_len = Blen + m;
            if (new_len > Clen) {
                C = (ll *)realloc(C, (new_len + 1) * sizeof(ll));
                for (int i = Clen; i <= new_len; i++) C[i] = 0;
                Clen = new_len;
            }
            for (int i = 0; i < Blen; i++) {
                C[i + m] = (C[i + m] - coef * B[i] % MOD + MOD) % MOD;
            }
            if (new_len > Clen) Clen = new_len;
            m++;
        }
    }

    *out_C = C;
    free(B);
    return L;
}

int main(void) {
    ll N_val = 1000000000000LL;
    int K = 5000;

    /* Compute T[1..2K-1] */
    ll *dp = (ll *)calloc(K, sizeof(ll));
    for (int i = 1; i < K; i++) dp[i] = 1;

    int seq_len = 2 * K - 1;
    ll *seq = (ll *)malloc(seq_len * sizeof(ll));

    for (int iter = 0; iter < seq_len; iter++) {
        ll ti = 0;
        for (int j = 0; j < K; j++) ti = (ti + dp[j]) % MOD;
        seq[iter] = ti;

        ll *new_dp = (ll *)calloc(K, sizeof(ll));
        new_dp[1] = ti;
        /* offsets = cumsum(dp[K-1..1]) */
        ll cum = 0;
        for (int j = K - 1; j >= 1; j--) {
            cum = (cum + dp[j]) % MOD;
            int idx = K - 1 - j; /* 0-based index into offsets */
            if (idx < K - 2) {
                new_dp[idx + 2] = (ti - cum + MOD) % MOD;
            }
        }

        free(dp);
        dp = new_dp;
    }
    free(dp);

    /* Berlekamp-Massey to find recurrence */
    ll *C_poly;
    int L = berlekamp_massey(seq, seq_len, &C_poly);
    int d = L;

    /* Build characteristic polynomial (monic) */
    /* char_poly[d] = 1, char_poly[d-1-i] = -recurrence[i] */
    /* recurrence[i] = -C_poly[i+1] */
    ll *char_poly = (ll *)calloc(d + 1, sizeof(ll));
    char_poly[d] = 1;
    for (int i = 0; i < d; i++) {
        char_poly[d - 1 - i] = (C_poly[i + 1]) % MOD; /* -(-C[i+1]) = C[i+1] */
    }

    /* cp_trunc = char_poly[0..d-1] (without leading 1) */
    ll *cp_trunc = (ll *)calloc(d, sizeof(ll));
    for (int i = 0; i < d; i++) cp_trunc[i] = char_poly[i];

    /* Kitamasa: compute x^(N-1) mod char_poly using binary exponentiation */
    ll *result = (ll *)calloc(d, sizeof(ll));
    result[0] = 1;
    ll *base = (ll *)calloc(d, sizeof(ll));
    base[1] = 1;

    ll exp = N_val - 1;
    while (exp > 0) {
        if (exp & 1) {
            int nout;
            ll *prod = poly_mul(result, d, base, d, &nout);
            ll *rem = poly_mod(prod, nout, cp_trunc, d);
            free(prod);
            free(result);
            result = rem;
        }
        {
            int nout;
            ll *prod = poly_mul(base, d, base, d, &nout);
            ll *rem = poly_mod(prod, nout, cp_trunc, d);
            free(prod);
            free(base);
            base = rem;
        }
        exp >>= 1;
    }

    ll ans = 0;
    for (int i = 0; i < d; i++) {
        ans = (ans + result[i] * seq[i]) % MOD;
    }

    printf("%lld\n", ans);

    free(result);
    free(base);
    free(char_poly);
    free(cp_trunc);
    free(C_poly);
    free(seq);
    return 0;
}

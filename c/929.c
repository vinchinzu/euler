/* Project Euler Problem 929 - Compositions with Odd-Length Runs
 * F(10^5) mod 1111124111
 * Uses NTT (3 primes + CRT) for polynomial inversion.
 * h[m] = sum_{d|m} (-1)^(d-1) * F_d (Fibonacci)
 * P(x) = 1 - H(x), answer = [x^N] P(x)^{-1}
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef long long ll;
typedef unsigned long long ull;
typedef __int128 lll;

#define FINAL_MOD 1111124111LL
#define P1 998244353LL
#define P2 1004535809LL
#define P3 469762049LL
#define G 3LL

ll power(ll base, ll exp, ll mod) {
    ll res = 1;
    base %= mod;
    while (exp > 0) {
        if (exp & 1) res = (lll)res * base % mod;
        base = (lll)base * base % mod;
        exp >>= 1;
    }
    return res;
}

ll modInverse(ll n, ll mod) {
    return power(n, mod - 2, mod);
}

void ntt(ll *a, int n, int invert, ll mod, ll root) {
    for (int i = 1, j = 0; i < n; i++) {
        int bit = n >> 1;
        for (; j & bit; bit >>= 1)
            j ^= bit;
        j ^= bit;
        if (i < j) { ll tmp = a[i]; a[i] = a[j]; a[j] = tmp; }
    }
    for (int len = 2; len <= n; len <<= 1) {
        ll wlen = power(root, (mod - 1) / len, mod);
        if (invert) wlen = modInverse(wlen, mod);
        for (int i = 0; i < n; i += len) {
            ll w = 1;
            for (int j = 0; j < len / 2; j++) {
                ll u = a[i + j], v = (lll)a[i + j + len / 2] * w % mod;
                a[i + j] = (u + v < mod ? u + v : u + v - mod);
                a[i + j + len / 2] = (u - v >= 0 ? u - v : u - v + mod);
                w = (lll)w * wlen % mod;
            }
        }
    }
    if (invert) {
        ll n_inv = modInverse(n, mod);
        for (int i = 0; i < n; i++)
            a[i] = (lll)a[i] * n_inv % mod;
    }
}

ll crt(ll r1, ll r2, ll r3) {
    ll inv_m1_m2 = modInverse(P1 % P2, P2);
    ll x1 = r1;
    ll k1 = (lll)((r2 - x1 + P2) % P2) * inv_m1_m2 % P2;
    lll M1 = P1;
    lll x12 = x1 + M1 * k1;

    lll M1M2 = M1 * P2;
    ll inv_M1M2_m3 = modInverse((ll)(M1M2 % P3), P3);
    ll k2 = (lll)((r3 - (ll)(x12 % P3) + P3) % P3) * inv_M1M2_m3 % P3;

    ll res = ((ll)(x12 % FINAL_MOD) + (ll)(M1M2 % FINAL_MOD) * k2 % FINAL_MOD) % FINAL_MOD;
    return res;
}

/* Multiply two polynomials using 3-prime NTT + CRT */
ll *multiply(ll *a, int na, ll *b, int nb, int *out_n) {
    int n = 1;
    while (n < na + nb) n <<= 1;
    *out_n = n;

    ll *fa1 = (ll *)calloc(n, sizeof(ll));
    ll *fb1 = (ll *)calloc(n, sizeof(ll));
    ll *c1 = (ll *)calloc(n, sizeof(ll));
    memcpy(fa1, a, na * sizeof(ll));
    memcpy(fb1, b, nb * sizeof(ll));
    ntt(fa1, n, 0, P1, G); ntt(fb1, n, 0, P1, G);
    for (int i = 0; i < n; i++) c1[i] = (lll)fa1[i] * fb1[i] % P1;
    ntt(c1, n, 1, P1, G);
    free(fa1); free(fb1);

    ll *fa2 = (ll *)calloc(n, sizeof(ll));
    ll *fb2 = (ll *)calloc(n, sizeof(ll));
    ll *c2 = (ll *)calloc(n, sizeof(ll));
    memcpy(fa2, a, na * sizeof(ll));
    memcpy(fb2, b, nb * sizeof(ll));
    ntt(fa2, n, 0, P2, G); ntt(fb2, n, 0, P2, G);
    for (int i = 0; i < n; i++) c2[i] = (lll)fa2[i] * fb2[i] % P2;
    ntt(c2, n, 1, P2, G);
    free(fa2); free(fb2);

    ll *fa3 = (ll *)calloc(n, sizeof(ll));
    ll *fb3 = (ll *)calloc(n, sizeof(ll));
    ll *c3 = (ll *)calloc(n, sizeof(ll));
    memcpy(fa3, a, na * sizeof(ll));
    memcpy(fb3, b, nb * sizeof(ll));
    ntt(fa3, n, 0, P3, G); ntt(fb3, n, 0, P3, G);
    for (int i = 0; i < n; i++) c3[i] = (lll)fa3[i] * fb3[i] % P3;
    ntt(c3, n, 1, P3, G);
    free(fa3); free(fb3);

    ll *res = (ll *)calloc(n, sizeof(ll));
    for (int i = 0; i < n; i++)
        res[i] = crt(c1[i], c2[i], c3[i]);
    free(c1); free(c2); free(c3);
    return res;
}

/* Polynomial inverse: given P[0..n-1], compute Q s.t. P*Q = 1 mod x^n */
ll *poly_inv(ll *P, int n) {
    if (n == 1) {
        ll *q = (ll *)calloc(1, sizeof(ll));
        q[0] = modInverse(P[0], FINAL_MOD);
        return q;
    }

    int half_n = (n + 1) / 2;
    ll *Q = poly_inv(P, half_n);

    /* T = P_trunc * Q (length n) */
    int tn;
    ll *P_trunc = (ll *)calloc(n, sizeof(ll));
    memcpy(P_trunc, P, n * sizeof(ll));
    ll *T = multiply(P_trunc, n, Q, half_n, &tn);
    free(P_trunc);

    /* R[0] = 2 - T[0], R[i] = -T[i] for i>0 */
    ll *R = (ll *)calloc(n, sizeof(ll));
    for (int i = 0; i < n; i++) {
        ll ti = (i < tn) ? T[i] : 0;
        if (i == 0)
            R[i] = (2 - ti + FINAL_MOD) % FINAL_MOD;
        else
            R[i] = (FINAL_MOD - ti) % FINAL_MOD;
    }
    free(T);

    int rn;
    ll *Res = multiply(Q, half_n, R, n, &rn);
    free(Q); free(R);

    /* Truncate to n terms */
    ll *result = (ll *)calloc(n, sizeof(ll));
    for (int i = 0; i < n && i < rn; i++)
        result[i] = Res[i];
    free(Res);
    return result;
}

int main(void) {
    int N = 100000;

    /* Compute Fibonacci mod FINAL_MOD */
    ll *F = (ll *)calloc(N + 1, sizeof(ll));
    F[1] = 1;
    if (N >= 2) F[2] = 1;
    for (int i = 3; i <= N; i++)
        F[i] = (F[i - 1] + F[i - 2]) % FINAL_MOD;

    /* Compute h[m] = sum_{d|m} (-1)^{d-1} * F[d] */
    ll *h = (ll *)calloc(N + 1, sizeof(ll));
    for (int d = 1; d <= N; d++) {
        ll val = F[d];
        if ((d - 1) % 2 == 1)
            val = (FINAL_MOD - val) % FINAL_MOD;
        for (int m = d; m <= N; m += d)
            h[m] = (h[m] + val) % FINAL_MOD;
    }
    free(F);

    /* P[0] = 1, P[i] = -h[i] for i >= 1 */
    ll *P = (ll *)calloc(N + 1, sizeof(ll));
    P[0] = 1;
    for (int i = 1; i <= N; i++)
        P[i] = (FINAL_MOD - h[i]) % FINAL_MOD;
    free(h);

    /* Compute Q = P^{-1} mod x^{N+1} */
    ll *Q = poly_inv(P, N + 1);
    free(P);

    printf("%lld\n", Q[N]);
    free(Q);
    return 0;
}

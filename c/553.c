/*
 * Project Euler Problem 553: Power Sets of Power Sets.
 * C(n, k) counts elements of R(n) whose intersection graph has exactly
 * k connected components. Find C(10^4, 10) mod 10^9+7.
 *
 * Uses EGF approach with polynomial operations.
 * Extracted from embedded C in Python solution.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef long long ll;
static const ll MOD = 1000000007;

ll power(ll base, ll exp, ll mod) {
    ll result = 1;
    base %= mod;
    if (base < 0) base += mod;
    while (exp > 0) {
        if (exp & 1) result = result * base % mod;
        base = base * base % mod;
        exp >>= 1;
    }
    return result;
}

ll inv(ll a, ll mod) { return power(a, mod - 2, mod); }

static ll fact[10001], inv_fact[10001];
static ll a_coeff[10001];
static ll e_neg[10001];
static ll e_pos[10001];

void precompute(int N) {
    fact[0] = 1;
    for (int i = 1; i <= N; i++) fact[i] = fact[i-1] * i % MOD;
    inv_fact[N] = inv(fact[N], MOD);
    for (int i = N-1; i >= 0; i--) inv_fact[i] = inv_fact[i+1] * (i+1) % MOD;

    for (int i = 0; i <= N; i++) {
        ll exp_val = power(2, i, MOD - 1);
        exp_val = (exp_val - 1 + MOD - 1) % (MOD - 1);
        a_coeff[i] = power(2, exp_val, MOD) % MOD * inv_fact[i] % MOD;
    }

    for (int i = 0; i <= N; i++) {
        e_neg[i] = (i % 2 == 0) ? inv_fact[i] : (MOD - inv_fact[i]) % MOD;
        e_pos[i] = inv_fact[i];
    }
}

/* Polynomial multiply truncated to degree N, O(N^2) */
void poly_mul(ll *a, ll *b, ll *out, int N) {
    memset(out, 0, (N+1) * sizeof(ll));
    for (int i = 0; i <= N; i++) {
        if (a[i] == 0) continue;
        ll ai = a[i];
        for (int j = 0; j <= N - i; j++) {
            out[i+j] = (out[i+j] + ai * b[j]) % MOD;
        }
    }
}

/* Polynomial inverse: given f with f[0]!=0, find g s.t. f*g = 1 mod x^(N+1) */
void poly_inv(ll *f, ll *g, int N) {
    memset(g, 0, (N+1) * sizeof(ll));
    g[0] = inv(f[0], MOD);
    for (int i = 1; i <= N; i++) {
        ll s = 0;
        for (int j = 1; j <= i; j++) {
            if (j < N+1 && f[j] != 0)
                s = (s + f[j] * g[i-j]) % MOD;
        }
        g[i] = (MOD - g[0] % MOD * s % MOD) % MOD;
    }
}

void poly_deriv(ll *f, ll *df, int N) {
    for (int i = 0; i < N; i++) {
        df[i] = f[i+1] * (i+1) % MOD;
    }
    df[N] = 0;
}

void poly_integ(ll *f, ll *intf, int N) {
    intf[0] = 0;
    for (int i = 0; i < N; i++) {
        intf[i+1] = f[i] % MOD * inv(i+1, MOD) % MOD;
    }
}

void poly_log(ll *f, ll *logf, int N) {
    ll *df = (ll*)calloc(N+1, sizeof(ll));
    ll *finv = (ll*)calloc(N+1, sizeof(ll));
    ll *quot = (ll*)calloc(N+1, sizeof(ll));

    poly_deriv(f, df, N);
    poly_inv(f, finv, N);
    poly_mul(df, finv, quot, N);
    poly_integ(quot, logf, N);

    free(df); free(finv); free(quot);
}

/* Precompute all inverses 1..N */
static ll inv_table[10001];
void precompute_inv(int N) {
    inv_table[1] = 1;
    for (int i = 2; i <= N; i++) {
        inv_table[i] = (MOD - MOD / i) * inv_table[MOD % i] % MOD;
    }
}

/* Faster polynomial exp using precomputed inverses */
void poly_exp_fast(ll *f, ll *g, int N) {
    memset(g, 0, (N+1) * sizeof(ll));
    g[0] = 1;
    ll *kf = (ll*)calloc(N+1, sizeof(ll));
    for (int k = 1; k <= N; k++) kf[k] = (ll)k % MOD * f[k] % MOD;

    for (int n = 1; n <= N; n++) {
        ll s = 0;
        for (int k = 1; k <= n; k++) {
            s = (s + kf[k] * g[n-k]) % MOD;
        }
        g[n] = s % MOD * inv_table[n] % MOD;
    }
    free(kf);
}

int main(void) {
    const int N = 10000;
    const int K = 10;

    precompute(N);
    precompute_inv(N);

    ll *p = (ll*)calloc(N+1, sizeof(ll));
    poly_mul(a_coeff, e_neg, p, N);

    ll *logp = (ll*)calloc(N+1, sizeof(ll));
    poly_log(p, logp, N);

    /* h = logp shifted down by 1 */
    ll *h = (ll*)calloc(N+1, sizeof(ll));
    for (int i = 0; i <= N-1; i++) h[i] = logp[i+1];

    ll h0 = h[0];
    ll h0_inv = inv(h0, MOD);
    ll *h_norm = (ll*)calloc(N+1, sizeof(ll));
    for (int i = 0; i <= N-K; i++) h_norm[i] = h[i] * h0_inv % MOD;

    ll *log_h = (ll*)calloc(N+1, sizeof(ll));
    poly_log(h_norm, log_h, N-K);

    for (int i = 0; i <= N-K; i++) log_h[i] = log_h[i] * K % MOD;

    ll *h_pow = (ll*)calloc(N+1, sizeof(ll));
    poly_exp_fast(log_h, h_pow, N-K);

    ll h0K = power(h0, K, MOD);
    for (int i = 0; i <= N-K; i++) h_pow[i] = h_pow[i] * h0K % MOD;

    ll *logpK = (ll*)calloc(N+1, sizeof(ll));
    for (int i = K; i <= N; i++) logpK[i] = h_pow[i-K];

    ll *result = (ll*)calloc(N+1, sizeof(ll));
    poly_mul(logpK, e_pos, result, N);

    ll ans = result[N] * fact[N] % MOD * inv_fact[K] % MOD;
    printf("%lld\n", (long long)ans);

    free(p); free(logp); free(h); free(h_norm); free(log_h);
    free(h_pow); free(logpK); free(result);
    return 0;
}

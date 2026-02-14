/*
 * Project Euler Problem 554: Centaurs on a chessboard.
 * C(n) = 8*C(2n,n) - 3*(n-1)^2 - 8n - 4
 * Find sum_{i=2}^{90} C(F_i) mod (10^8+7).
 * Uses Lucas' theorem for binomial coefficients mod prime.
 *
 * Extracted from embedded C in Python solution.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef long long ll;
typedef unsigned long long ull;

static const ll M = 100000007LL;  /* 10^8 + 7, prime */

static ll *fact = NULL;
static ll *inv_fact = NULL;

static ll power(ll base, ll exp, ll mod) {
    ll result = 1;
    base %= mod;
    while (exp > 0) {
        if (exp & 1) result = result * base % mod;
        base = base * base % mod;
        exp >>= 1;
    }
    return result;
}

static void precompute(void) {
    fact = (ll *)malloc(M * sizeof(ll));
    inv_fact = (ll *)malloc(M * sizeof(ll));
    if (!fact || !inv_fact) { fprintf(stderr, "malloc failed\n"); exit(1); }

    fact[0] = 1;
    for (ll i = 1; i < M; i++)
        fact[i] = fact[i-1] * i % M;

    inv_fact[M-1] = power(fact[M-1], M-2, M);
    for (ll i = M-2; i >= 0; i--)
        inv_fact[i] = inv_fact[i+1] * (i+1) % M;
}

static ll nCr_small(ll a, ll b) {
    if (b < 0 || b > a || a < 0) return 0;
    if (a >= M) return 0;
    return fact[a] % M * inv_fact[b] % M * inv_fact[a-b] % M;
}

/* Lucas theorem: C(n, r) mod p */
static ll nCr_lucas(ull n, ull r) {
    if (r > n) return 0;
    ll result = 1;
    while (n > 0 || r > 0) {
        ll ni = (ll)(n % M);
        ll ri = (ll)(r % M);
        if (ri > ni) return 0;
        result = result * nCr_small(ni, ri) % M;
        n /= M;
        r /= M;
    }
    return result;
}

int main(void) {
    precompute();

    ull fibs[91];
    fibs[0] = 0; fibs[1] = 1;
    for (int i = 2; i <= 90; i++)
        fibs[i] = fibs[i-1] + fibs[i-2];

    ll ans = 0;
    for (int i = 2; i <= 90; i++) {
        ull n = fibs[i];
        ll n_mod = (ll)(n % M);
        ll c2n_n = nCr_lucas(2ULL * n, n);

        ll sq = (n_mod - 1 + M) % M;
        sq = sq * sq % M;
        ll val = 8LL * c2n_n % M;
        val = (val - 3LL * sq % M + M) % M;
        val = (val - 8LL * n_mod % M + M) % M;
        val = (val - 4 + M) % M;

        ans = (ans + val) % M;
    }

    printf("%lld\n", ans);

    free(fact);
    free(inv_fact);
    return 0;
}

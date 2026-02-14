/*
 * Project Euler 781 - Feynman Diagrams
 *
 * F(n) = number of non-isomorphic connected graphs with n degree-3 vertices.
 * Using EGF convolution: f(m) = t(m) - sum_{j=1}^{m} s(j) * f(m-j)
 * where t(j) = B(2j) / (2^j * j!), s(j) = D(2j) / (2^j * j!)
 * B(n) = (n+1)*D(n) + n*D(n-1), D = derangements.
 */
#include <stdio.h>
#include <stdlib.h>

#define MOD 1000000007LL

typedef long long ll;

static ll mod(ll x) {
    x %= MOD;
    if (x < 0) x += MOD;
    return x;
}

static ll powmod(ll base, ll exp, ll m) {
    ll result = 1;
    base %= m;
    while (exp > 0) {
        if (exp & 1) result = result * base % m;
        base = base * base % m;
        exp >>= 1;
    }
    return result;
}

int main(void) {
    int n = 50000;
    int m = n / 2;

    /* Derangements D[k] for k=0..n */
    ll *D = (ll *)calloc(n + 1, sizeof(ll));
    D[0] = 1;
    D[1] = 0;
    for (int k = 2; k <= n; k++) {
        D[k] = ((ll)k * D[k - 1] % MOD + (k % 2 == 0 ? 1 : MOD - 1)) % MOD;
    }

    /* Factorials and inverse factorials mod MOD */
    ll *fact = (ll *)malloc((m + 1) * sizeof(ll));
    ll *inv_fact = (ll *)malloc((m + 1) * sizeof(ll));
    fact[0] = 1;
    for (int i = 1; i <= m; i++)
        fact[i] = fact[i - 1] * i % MOD;
    inv_fact[m] = powmod(fact[m], MOD - 2, MOD);
    for (int i = m - 1; i >= 0; i--)
        inv_fact[i] = inv_fact[i + 1] * (i + 1) % MOD;

    /* Powers of inv(2) */
    ll inv2 = powmod(2, MOD - 2, MOD);
    ll *inv_pow2 = (ll *)malloc((m + 1) * sizeof(ll));
    inv_pow2[0] = 1;
    for (int i = 1; i <= m; i++)
        inv_pow2[i] = inv_pow2[i - 1] * inv2 % MOD;

    /* Compute t[j] and s[j] */
    ll *t = (ll *)calloc(m + 1, sizeof(ll));
    ll *s = (ll *)calloc(m + 1, sizeof(ll));
    for (int j = 0; j <= m; j++) {
        int idx = 2 * j;
        ll B_val;
        if (idx == 0)
            B_val = 1;
        else
            B_val = ((ll)(idx + 1) * D[idx] % MOD + (ll)idx * D[idx - 1] % MOD) % MOD;
        ll coeff = inv_pow2[j] * inv_fact[j] % MOD;
        t[j] = B_val * coeff % MOD;
        s[j] = D[idx] * coeff % MOD;
    }

    /* Compute f[j] = t[j] - sum_{k=1}^{j} s[k] * f[j-k] */
    ll *f = (ll *)calloc(m + 1, sizeof(ll));
    for (int j = 0; j <= m; j++) {
        ll val = t[j];
        for (int k = 1; k <= j; k++) {
            val = mod(val - s[k] * f[j - k] % MOD);
        }
        f[j] = val;
    }

    printf("%lld\n", f[m] % MOD);

    free(D); free(fact); free(inv_fact); free(inv_pow2);
    free(t); free(s); free(f);
    return 0;
}

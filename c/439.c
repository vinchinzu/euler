/*
 * Project Euler 439 - Sum of sum of divisors
 *
 * S(N) = sum_{i=1}^N sum_{j=1}^N d(i*j), mod 10^9.
 * Extracted from embedded C in python/439.py.
 */
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>
#include <math.h>

typedef long long ll;
typedef unsigned long long ull;
typedef __int128 lll;

#define N 100000000000LL
#define MOD 1000000000LL

static inline ll mod(ll x) {
    x %= MOD;
    if (x < 0) x += MOD;
    return x;
}

int L;
signed char *mobius;
ll *sigma_prefix;
ll *n_mu_prefix;

#define CACHE_SIZE 400000
ll sigma_cache[CACHE_SIZE];
ll n_mu_cache[CACHE_SIZE];
char sigma_cached[CACHE_SIZE];
char n_mu_cached[CACHE_SIZE];

void sieve(int limit) {
    mobius = (signed char*)malloc((limit + 1) * sizeof(signed char));
    int *spf = (int*)malloc((limit + 1) * sizeof(int));

    for (int i = 0; i <= limit; i++) {
        spf[i] = i;
        mobius[i] = 1;
    }

    for (int i = 2; i <= limit; i++) {
        if (spf[i] == i) {
            mobius[i] = -1;
            for (ll j = (ll)i * i; j <= limit; j += i) {
                if (spf[j] == j) spf[j] = i;
            }
        } else {
            int p = spf[i];
            int q = i / p;
            if (q % p == 0) {
                mobius[i] = 0;
            } else {
                mobius[i] = -mobius[q];
            }
        }
    }
    free(spf);
}

void compute_prefix_sums(int limit) {
    ll *sigma = (ll*)calloc(limit + 1, sizeof(ll));
    for (int d = 1; d <= limit; d++) {
        for (int m = d; m <= limit; m += d) {
            sigma[m] += d;
        }
    }

    sigma_prefix = (ll*)malloc((limit + 1) * sizeof(ll));
    sigma_prefix[0] = 0;
    for (int i = 1; i <= limit; i++) {
        sigma_prefix[i] = mod(sigma_prefix[i-1] + sigma[i]);
    }
    free(sigma);

    n_mu_prefix = (ll*)malloc((limit + 1) * sizeof(ll));
    n_mu_prefix[0] = 0;
    for (int i = 1; i <= limit; i++) {
        n_mu_prefix[i] = mod(n_mu_prefix[i-1] + (ll)i * mobius[i]);
    }
}

static inline ll isqrt(ll n) {
    ll x = (ll)sqrtl((long double)n);
    while (x > 0 && x * x > n) x--;
    while ((x + 1) * (x + 1) <= n) x++;
    return x;
}

static inline ll tr(ll n) {
    n %= (2 * MOD);
    return (n * (n + 1) / 2) % MOD;
}

static inline ll sum_range(ll a, ll b) {
    return mod(tr(b) - tr(a - 1));
}

ll sigma_sum(ll n) {
    if (n <= L) return sigma_prefix[n];

    ll idx = N / n;
    if (idx < CACHE_SIZE && sigma_cached[idx]) return sigma_cache[idx];

    ll result = 0;
    ll sqrt_n = isqrt(n);

    for (ll d = 1; d <= sqrt_n; d++) {
        result = mod(result + (d % MOD) * ((n / d) % MOD));
    }

    for (ll k = 1; k <= sqrt_n; k++) {
        ll d_hi = n / k;
        if (d_hi > sqrt_n) {
            ll d_lo = n / (k + 1) + 1;
            result = mod(result + sum_range(d_lo, d_hi) * (k % MOD));
        }
    }

    if (idx < CACHE_SIZE) {
        sigma_cache[idx] = result;
        sigma_cached[idx] = 1;
    }

    return result;
}

ll n_mu_sum(ll n) {
    if (n <= L) return n_mu_prefix[n];

    ll idx = N / n;
    if (idx < CACHE_SIZE && n_mu_cached[idx]) return n_mu_cache[idx];

    ll result = 1;
    ll sqrt_n = isqrt(n);

    for (ll d = 2; d <= sqrt_n; d++) {
        result = mod(result - n_mu_sum(n / d) * (d % MOD));
    }

    for (ll k = 1; k <= sqrt_n; k++) {
        ll d_hi = n / k;
        ll d_lo = n / (k + 1);
        if (d_hi > sqrt_n && d_lo >= sqrt_n) {
            result = mod(result - n_mu_sum(k) * sum_range(d_lo + 1, d_hi));
        }
    }

    if (idx < CACHE_SIZE) {
        n_mu_cache[idx] = result;
        n_mu_cached[idx] = 1;
    }

    return result;
}

int main() {
    L = (int)sqrtl((long double)N) + 10;

    sieve(L);
    compute_prefix_sums(L);

    memset(sigma_cached, 0, sizeof(sigma_cached));
    memset(n_mu_cached, 0, sizeof(n_mu_cached));

    ll ans = 0;

    for (int g = 1; g <= L; g++) {
        if (mobius[g] != 0) {
            ll ss = sigma_sum(N / g);
            ll term = (lll)mobius[g] * g % MOD * ss % MOD * ss % MOD;
            ans = mod(ans + term);
        }
    }

    for (ll q = 1; q < N / L; q++) {
        ll g_hi = N / q;
        ll g_lo = N / (q + 1);
        if (g_lo < L) g_lo = L;
        if (g_hi > g_lo) {
            ll ss = sigma_sum(q);
            ll mu_diff = mod(n_mu_sum(g_hi) - n_mu_sum(g_lo));
            ll term = (lll)mu_diff * ss % MOD * ss % MOD;
            ans = mod(ans + term);
        }
    }

    printf("%lld\n", ans);

    free(mobius);
    free(sigma_prefix);
    free(n_mu_prefix);

    return 0;
}

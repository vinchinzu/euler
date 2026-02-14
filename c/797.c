/*
 * Project Euler 797 - Cyclotomic Polynomials
 *
 * Let F_n(x) be the nth cyclotomic polynomial. We compute F_n(2) by sieve division.
 * G_n(2) = product of (F_i(2)+1) for all i|n is the sum of all factor-subsets of x^n-1 evaluated at 2.
 * P_n(x) = sum of polynomials dividing x^n-1 but not any smaller x^k-1.
 * By Mobius inversion, total = sum_{n=1}^N Mertens(N/n) * G_n(2).
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef long long ll;

#define N 10000000
#define MOD 1000000007LL

static ll F[N + 1];
static ll G[N + 1];
static int mu[N + 1];
static ll mertens[N + 1];
static char is_prime[N + 1];

ll pow_mod(ll base, ll exp, ll mod) {
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

ll mod_inv(ll a, ll m) {
    return pow_mod(a, m - 2, m);
}

int main(void) {
    /* Compute Mobius function */
    memset(is_prime, 1, sizeof(is_prime));
    is_prime[0] = is_prime[1] = 0;
    for (int i = 0; i <= N; i++) mu[i] = 1;

    for (int i = 2; i <= N; i++) {
        if (is_prime[i]) {
            for (int j = i; j <= N; j += i) {
                if (j > i) is_prime[j] = 0;
                if ((ll)(j / i) % i == 0) {
                    mu[j] = 0;
                } else {
                    mu[j] = -mu[j];
                }
            }
        }
    }

    /* Mertens function */
    mertens[0] = 0;
    for (int i = 1; i <= N; i++) {
        mertens[i] = mertens[i - 1] + mu[i];
    }

    /* Compute F_n(2) = cyclotomic polynomial evaluated at 2 */
    /* Start with F[n] = 2^n - 1, then divide by F[d] for each d|n, d<n */
    for (int i = 0; i <= N; i++) {
        F[i] = (pow_mod(2, i, MOD) - 1 + MOD) % MOD;
    }

    for (int i = 1; i <= N; i++) {
        ll inv = mod_inv(F[i], MOD);
        for (ll j = 2 * i; j <= N; j += i) {
            F[j] = F[j] * inv % MOD;
        }
    }

    /* Compute G_n(2) = product of (F_d(2)+1) for all d|n */
    for (int i = 0; i <= N; i++) G[i] = 1;
    for (int i = 1; i <= N; i++) {
        ll factor = (F[i] + 1) % MOD;
        for (ll j = i; j <= N; j += i) {
            G[j] = G[j] * factor % MOD;
        }
    }

    /* Sum contributions: ans = sum_{n=1}^N Mertens(N/n) * G_n(2) */
    ll ans = 0;
    for (int i = 1; i <= N; i++) {
        ans = (ans + (mertens[N / i] % MOD + MOD) % MOD * G[i]) % MOD;
    }

    printf("%lld\n", ans);
    return 0;
}

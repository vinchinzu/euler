/*
 * Project Euler 840: Arithmetic Derivative Partition Sums
 *
 * D(n) = arithmetic derivative with D(1) = 1.
 * B[k] = sum_{d|k} d * D(d)^(k/d)
 * g[0]=1; g[n] = (1/n) * sum_{k=1}^n B[k] * g[n-k]
 * S(N) = sum_{n=1}^N G(n) = sum g[1..N]
 *
 * N = 50000, mod 999676999
 */
#include <stdio.h>
#include <stdlib.h>

typedef long long ll;
typedef __int128 i128;

#define NMAX 50000
#define MOD 999676999LL

static int spf[NMAX + 1];
static ll D_arr[NMAX + 1];
static ll B_arr[NMAX + 1];
static ll g[NMAX + 1];
static ll inv_arr[NMAX + 1];

int main(void) {
    /* SPF sieve */
    for (int i = 0; i <= NMAX; i++) spf[i] = i;
    for (int i = 2; (ll)i * i <= NMAX; i++) {
        if (spf[i] == i) {
            for (int j = i * i; j <= NMAX; j += i) {
                if (spf[j] == j) spf[j] = i;
            }
        }
    }

    /* Arithmetic derivative */
    D_arr[1] = 1;
    for (int n = 2; n <= NMAX; n++) {
        int m = n;
        ll deriv = 0;
        while (m > 1) {
            int p = spf[m];
            int e = 0;
            while (m % p == 0) { m /= p; e++; }
            deriv = (deriv + (ll)e * (n / p)) % MOD;
        }
        D_arr[n] = deriv;
    }

    /* Build B[k] = sum_{d|k} d * D(d)^(k/d) */
    for (int d = 1; d <= NMAX; d++) {
        ll y = D_arr[d] % MOD;
        ll pow_y = y;
        for (int k = d; k <= NMAX; k += d) {
            B_arr[k] = (B_arr[k] + (i128)d * pow_y) % MOD;
            pow_y = (i128)pow_y * y % MOD;
        }
    }

    /* Precompute modular inverses */
    inv_arr[1] = 1;
    for (int n = 2; n <= NMAX; n++) {
        inv_arr[n] = (MOD - (MOD / n) * inv_arr[MOD % n] % MOD) % MOD;
    }

    /* Compute g[n] */
    g[0] = 1;
    for (int n = 1; n <= NMAX; n++) {
        ll s = 0;
        for (int k = 1; k <= n; k++) {
            s = (s + (i128)B_arr[k] * g[n - k]) % MOD;
        }
        g[n] = s % MOD * inv_arr[n] % MOD;
    }

    /* S(N) = sum g[1..N] */
    ll S = 0;
    for (int n = 1; n <= NMAX; n++) {
        S = (S + g[n]) % MOD;
    }

    printf("%lld\n", S);
    return 0;
}

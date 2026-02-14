/* Project Euler 956 - D(1000*, 1000) mod 999999001
 * Uses DFT-based approach with roots of unity.
 */
#include <stdio.h>
#include <stdlib.h>
#include <math.h>

typedef long long ll;
typedef __int128 lll;
#define MOD 999999001LL

ll mod_pow(ll base, ll exp, ll mod) {
    ll result = 1;
    base %= mod;
    if (base < 0) base += mod;
    while (exp > 0) {
        if (exp & 1) result = (lll)result * base % mod;
        base = (lll)base * base % mod;
        exp >>= 1;
    }
    return result;
}

ll mod_inv(ll a, ll mod) {
    return mod_pow(a, mod - 2, mod);
}

int sieve_primes(int limit, int *primes) {
    char *is_p = (char *)calloc(limit + 1, 1);
    for (int i = 2; i <= limit; i++) is_p[i] = 1;
    for (int i = 2; (ll)i*i <= limit; i++) {
        if (is_p[i]) {
            for (int j = i*i; j <= limit; j += i) is_p[j] = 0;
        }
    }
    int cnt = 0;
    for (int i = 2; i <= limit; i++) {
        if (is_p[i]) primes[cnt++] = i;
    }
    free(is_p);
    return cnt;
}

int is_primitive_root(ll g, ll mod, int *factors, int nf) {
    ll phi = mod - 1;
    for (int i = 0; i < nf; i++) {
        if (mod_pow(g, phi / factors[i], mod) == 1) return 0;
    }
    return 1;
}

int main(void) {
    int n = 1000, m = 1000;
    int phi_factors[] = {2, 3, 5, 7, 11, 13, 37};
    int nf = 7;

    /* Find primitive root of MOD */
    ll g = 2;
    while (!is_primitive_root(g, MOD, phi_factors, nf)) g++;

    /* Get primes up to n */
    int primes[200];
    int num_primes = sieve_primes(n, primes);

    /* Compute exponents of each prime in 1000-bigstar */
    ll *exponents = (ll *)malloc(num_primes * sizeof(ll));
    for (int idx = 0; idx < num_primes; idx++) {
        int pp = primes[idx];
        ll e = 0;
        for (int k = 1; k <= n; k++) {
            ll vk = 0;
            ll pk = pp;
            while (pk <= k) {
                vk += k / pk;
                pk *= pp;
            }
            e += (ll)(n - k + 1) * vk;
        }
        exponents[idx] = e;
    }

    ll phi = MOD - 1;
    ll omega = mod_pow(g, phi / m, MOD);
    ll inv_m = mod_inv(m, MOD);

    ll sum_S = 0;
    for (int j = 0; j < m; j++) {
        ll y = mod_pow(omega, j, MOD);
        ll prod = 1;
        for (int idx = 0; idx < num_primes; idx++) {
            int pp = primes[idx];
            ll e = exponents[idx];
            ll r = (lll)pp * y % MOD;
            ll geo;
            if (r == 1) {
                geo = (e + 1) % MOD;
            } else {
                ll exp_mod = (e + 1) % phi;
                ll re_plus1 = mod_pow(r, exp_mod, MOD);
                ll num = (1 - re_plus1 + MOD + MOD) % MOD;
                ll den = (1 - r + MOD + MOD) % MOD;
                ll den_inv = mod_inv(den, MOD);
                geo = (lll)num * den_inv % MOD;
            }
            prod = (lll)prod * geo % MOD;
        }
        sum_S = (sum_S + prod) % MOD;
    }

    ll result = (lll)sum_S * inv_m % MOD;
    printf("%lld\n", result);

    free(exponents);
    return 0;
}

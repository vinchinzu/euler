/*
 * Project Euler 650 - Divisors of Binomial Product
 * B(n) = product C(n,k). D(n) = sigma(B(n)). S(N) = sum D(n) mod 10^9+7.
 * Extracted from embedded C, made standalone (no stdin).
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef long long ll;
#define MAXN 20001
#define N_VAL 20000
#define MOD_VAL 1000000007LL

ll power(ll base, ll exp, ll mod) {
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

int spf[MAXN];
int primes_arr[MAXN];
int nprimes;
int pidx[MAXN];

int B_exp[MAXN];
int fact_exp[MAXN];

int main(void) {
    ll MOD = MOD_VAL;
    int N = N_VAL;

    /* Sieve SPF */
    for (int i = 2; i <= N; i++) spf[i] = 0;
    nprimes = 0;
    for (int i = 2; i <= N; i++) {
        if (spf[i] == 0) {
            spf[i] = i;
            primes_arr[nprimes] = i;
            pidx[i] = nprimes;
            nprimes++;
        }
        for (int j = 0; j < nprimes && primes_arr[j] <= spf[i] && (ll)i * primes_arr[j] <= N; j++) {
            spf[i * primes_arr[j]] = primes_arr[j];
        }
    }

    ll *inv_pm1 = (ll *)malloc(nprimes * sizeof(ll));
    for (int i = 0; i < nprimes; i++) {
        inv_pm1[i] = power(primes_arr[i] - 1, MOD - 2, MOD);
    }

    memset(B_exp, 0, sizeof(B_exp));
    memset(fact_exp, 0, sizeof(fact_exp));

    ll *D_factor = (ll *)malloc(nprimes * sizeof(ll));
    for (int i = 0; i < nprimes; i++) D_factor[i] = 1;

    ll answer = 0;

    for (int n = 1; n <= N; n++) {
        for (int i = 0; i < nprimes && primes_arr[i] < n; i++) {
            if (fact_exp[i] > 0) {
                B_exp[i] -= fact_exp[i];
                if (B_exp[i] > 0) {
                    D_factor[i] = (power(primes_arr[i], B_exp[i] + 1, MOD) - 1 + MOD) % MOD * inv_pm1[i] % MOD;
                } else {
                    D_factor[i] = 1;
                }
            }
        }

        {
            int m = n;
            while (m > 1) {
                int p = spf[m];
                int e = 0;
                while (m % p == 0) { m /= p; e++; }
                int pi = pidx[p];
                B_exp[pi] += (n - 1) * e;
                fact_exp[pi] += e;
                if (B_exp[pi] > 0) {
                    D_factor[pi] = (power(p, B_exp[pi] + 1, MOD) - 1 + MOD) % MOD * inv_pm1[pi] % MOD;
                } else {
                    D_factor[pi] = 1;
                }
            }
        }

        ll D = 1;
        for (int i = 0; i < nprimes && primes_arr[i] <= n; i++) {
            D = D * D_factor[i] % MOD;
        }

        answer = (answer + D) % MOD;
    }

    printf("%lld\n", answer);

    free(inv_pm1);
    free(D_factor);
    return 0;
}

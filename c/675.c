/*
 * Project Euler 675 - 2^omega(n!)
 *
 * S(n) = prod_e (1 + 2e) for exponents in prime factorization.
 * Iteratively compute S(i!) by tracking prime exponents.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef long long ll;
typedef __int128 lll;

#define N 10000000
#define MOD 1000000087LL

static int ff[N + 1];      /* smallest prime factor */
static ll invs[2 * N + 1]; /* modular inverses */
static int exponents[N + 1];

int main() {
    /* Smallest prime factor sieve */
    for (int i = 0; i <= N; i++) ff[i] = i;
    for (int i = 2; (ll)i * i <= N; i++) {
        if (ff[i] == i) {
            for (int j = i * i; j <= N; j += i) {
                if (ff[j] == j) ff[j] = i;
            }
        }
    }

    /* Precompute modular inverses for 1..2*N */
    invs[1] = 1;
    for (int i = 2; i <= 2 * N; i++) {
        invs[i] = (MOD - (MOD / i) * invs[MOD % i] % MOD) % MOD;
    }

    memset(exponents, 0, sizeof(exponents));
    ll S = 1;
    ll ans = 0;

    for (int i = 2; i <= N; i++) {
        int ii = i;
        while (ii > 1) {
            int p = ff[ii];
            int e = 0;
            while (ii % p == 0) {
                ii /= p;
                e++;
            }

            S = (lll)S * invs[1 + 2 * exponents[p]] % MOD;
            exponents[p] += e;
            S = (lll)S * (1 + 2 * exponents[p]) % MOD;
        }
        ans = (ans + S) % MOD;
    }

    printf("%lld\n", ans);
    return 0;
}

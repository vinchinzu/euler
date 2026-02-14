#include <stdio.h>
#include <stdlib.h>

typedef long long ll;
#define MOD 1234567891LL

int main(void) {
    int N = 10000000;
    int LIMIT = N / 2 + 2;

    /* Precompute modular inverses */
    ll *inv = (ll *)calloc((size_t)(LIMIT + 1), sizeof(ll));
    if (!inv) { fprintf(stderr, "malloc failed\n"); return 1; }
    inv[1] = 1;
    for (int i = 2; i <= LIMIT; i++) {
        inv[i] = (MOD - MOD / i) * inv[MOD % i] % MOD;
    }

    int M = N / 2;
    ll comb = 1;
    ll total_sum = 0;
    ll inv2 = inv[2];

    for (int m = 1; m <= M; m++) {
        /* Update comb to binom(2m, m) */
        comb = comb * 2 % MOD * (2 * m - 1) % MOD * inv[m] % MOD;

        ll Tm = comb * comb % MOD;

        /* Add D(2m) = Tm / 2 */
        ll term_even = Tm * inv2 % MOD;
        total_sum = (total_sum + term_even) % MOD;

        /* Add D(2m+1) if within range */
        if (2 * m + 1 <= N) {
            ll term_odd = Tm * 2 % MOD * m % MOD * inv[m + 1] % MOD;
            total_sum = (total_sum + term_odd) % MOD;
        }
    }

    printf("%lld\n", total_sum);
    free(inv);
    return 0;
}

#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>

#define N 12344321
#define MOD 135707531

static uint64_t pow_mod(uint64_t base, uint64_t exp) {
    uint64_t result = 1 % MOD;
    base %= MOD;
    while (exp) {
        if (exp & 1) result = (result * base) % MOD;
        base = (base * base) % MOD;
        exp >>= 1;
    }
    return result;
}

int main(void) {
    uint64_t *fact = malloc((N + 1) * sizeof(uint64_t));
    uint64_t *inv_fact = malloc((N + 1) * sizeof(uint64_t));
    if (!fact || !inv_fact) {
        fprintf(stderr, "Allocation failed\n");
        return 1;
    }

    fact[0] = 1;
    for (uint64_t i = 1; i <= N; i++) {
        fact[i] = (fact[i - 1] * i) % MOD;
    }

    inv_fact[N] = pow_mod(fact[N], MOD - 2);
    for (uint64_t i = N; i > 0; i--) {
        inv_fact[i - 1] = (inv_fact[i] * i) % MOD;
    }

    uint64_t ans = (uint64_t)N * (N - 1) % MOD;
    ans = ans * pow_mod(N - 2, N - 1) % MOD;

    for (uint64_t l = 2; l < N; l++) {
        uint64_t ncr = fact[N] * inv_fact[l] % MOD * inv_fact[N - l] % MOD;
        uint64_t term = ncr * fact[l - 1] % MOD;
        term = term * pow_mod(N - l - 1, N - l) % MOD;
        ans += term;
        if (ans >= MOD) ans -= MOD;
    }

    printf("%llu\n", (unsigned long long)(ans % MOD));
    free(fact);
    free(inv_fact);
    return 0;
}

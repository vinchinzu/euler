/*
 * Project Euler Problem 250: 250250
 *
 * Find the number of nonempty subsets of {1^1, 2^2, ..., 250250^250250}
 * whose sum is divisible by 250, modulo 10^16.
 */
#include <stdio.h>
#include <string.h>

#define N 250250
#define K 250
#define MOD 10000000000000000ULL

typedef unsigned long long ull;

static ull dp[K];
static ull new_dp[K];

/* Modular exponentiation: base^exp mod mod */
static int pow_mod(int base, int exp, int mod) {
    long long result = 1, b = base % mod;
    while (exp > 0) {
        if (exp & 1) result = result * b % mod;
        b = b * b % mod;
        exp >>= 1;
    }
    return (int)result;
}

int main(void) {
    memset(dp, 0, sizeof(dp));
    dp[0] = 1;

    for (int k = 1; k <= N; k++) {
        int val = pow_mod(k, k, K);
        memcpy(new_dp, dp, sizeof(dp));
        for (int i = 0; i < K; i++) {
            int new_i = (i + val) % K;
            new_dp[new_i] = (dp[i] + new_dp[new_i]) % MOD;
        }
        memcpy(dp, new_dp, sizeof(dp));
    }

    ull ans = (dp[0] + MOD - 1) % MOD;
    printf("%llu\n", ans);
    return 0;
}

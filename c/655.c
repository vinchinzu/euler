/*
 * Project Euler 655 - Divisible Palindromes
 * Count palindromes with up to N=32 digits divisible by K=10^7+19.
 * Extracted from embedded C.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define B 10
#define NN 32

static long long pow_mod(long long base, long long exp, long long mod) {
    long long result = 1;
    base %= mod;
    while (exp > 0) {
        if (exp & 1) result = result * base % mod;
        base = base * base % mod;
        exp >>= 1;
    }
    return result;
}

static long long *dp, *new_dp;

static long long num_palindromes(int num_digits, long long K) {
    memset(dp, 0, K * sizeof(long long));
    dp[0] = 1;

    int half = (num_digits + 1) / 2;
    for (int i = 0; i < half; i++) {
        long long mult;
        if (2 * i + 1 == num_digits) {
            mult = pow_mod(B, i, K);
        } else {
            mult = (pow_mod(B, i, K) + pow_mod(B, num_digits - 1 - i, K)) % K;
        }

        memset(new_dp, 0, K * sizeof(long long));

        for (int d = 0; d < B; d++) {
            long long shift = (mult * d) % K;
            for (long long j = 0; j < K; j++) {
                if (dp[j]) {
                    long long nj = (j + shift) % K;
                    new_dp[nj] += dp[j];
                }
            }
        }

        long long *tmp = dp;
        dp = new_dp;
        new_dp = tmp;
    }

    return dp[0] - 1;
}

int main(void) {
    long long K = 10000019LL;

    dp = (long long *)calloc(K, sizeof(long long));
    new_dp = (long long *)calloc(K, sizeof(long long));

    if (!dp || !new_dp) {
        fprintf(stderr, "Memory allocation failed\n");
        return 1;
    }

    long long ans = 0;
    ans += num_palindromes(NN - 1, K);
    ans += num_palindromes(NN, K);

    printf("%lld\n", ans);

    free(dp);
    free(new_dp);
    return 0;
}

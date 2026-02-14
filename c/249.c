/*
 * Project Euler Problem 249: Prime Subset Sums
 *
 * Find the number of subsets of primes < 5000 that have a prime sum,
 * modulo 10^16.
 *
 * Extracted from embedded C in Python solution.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define N 5000
#define M 10000000000000000ULL

int main(void) {
    /* Generate primes up to N using sieve */
    char is_prime_n[N + 1];
    memset(is_prime_n, 1, sizeof(is_prime_n));
    is_prime_n[0] = is_prime_n[1] = 0;
    for (int i = 2; i * i <= N; i++)
        if (is_prime_n[i])
            for (int j = i * i; j <= N; j += i)
                is_prime_n[j] = 0;

    int primes[700];
    int num_primes = 0;
    int total_sum = 0;
    for (int i = 2; i <= N; i++)
        if (is_prime_n[i]) {
            primes[num_primes++] = i;
            total_sum += i;
        }

    /* Allocate DP array */
    unsigned long long *dp = calloc(total_sum + 1, sizeof(unsigned long long));
    dp[0] = 1;

    int current_sum = 0;
    for (int pi = 0; pi < num_primes; pi++) {
        int p = primes[pi];
        current_sum += p;
        for (int i = current_sum; i >= p; i--) {
            dp[i] += dp[i - p];
            if (dp[i] >= M)
                dp[i] -= M;
        }
    }

    /* Generate primes up to total_sum for checking */
    char *is_prime_sum = calloc(total_sum + 1, 1);
    memset(is_prime_sum, 1, total_sum + 1);
    is_prime_sum[0] = is_prime_sum[1] = 0;
    for (int i = 2; (long)i * i <= total_sum; i++)
        if (is_prime_sum[i])
            for (int j = i * i; j <= total_sum; j += i)
                is_prime_sum[j] = 0;

    /* Sum dp values at prime indices */
    unsigned long long ans = 0;
    for (int i = 2; i <= total_sum; i++) {
        if (is_prime_sum[i]) {
            ans += dp[i];
            if (ans >= M)
                ans -= M;
        }
    }

    printf("%llu\n", ans);

    free(dp);
    free(is_prime_sum);
    return 0;
}

/*
 * Project Euler Problem 386: Antichain Counting
 *
 * N(n) is the max antichain size in divisor lattice of n.
 * Sum of N(n) for n=1 to 10^8.
 * Extracted from external C helper.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define N 100000000
#define MAX_PRIMES 6000000
#define MAX_EXPONENTS 30

int primes[MAX_PRIMES];
int prime_count = 0;

void sieve(void) {
    char *is_prime = calloc(N + 1, 1);
    for (int i = 2; i <= N; i++) is_prime[i] = 1;

    for (int i = 2; (long long)i * i <= N; i++) {
        if (is_prime[i]) {
            for (int j = i * i; j <= N; j += i) {
                is_prime[j] = 0;
            }
        }
    }

    for (int i = 2; i <= N; i++) {
        if (is_prime[i]) {
            primes[prime_count++] = i;
        }
    }
    free(is_prime);
}

long count_combinations(int *exps, int len) {
    if (len == 0) return 1;

    int total = 0;
    for (int i = 0; i < len; i++) total += exps[i];
    int target = total / 2;

    long *dp = calloc(target + 1, sizeof(long));
    dp[0] = 1;

    for (int i = 0; i < len; i++) {
        long *new_dp = calloc(target + 1, sizeof(long));
        for (int s = 0; s <= target; s++) {
            if (dp[s] > 0) {
                int max_k = exps[i];
                if (target - s < max_k) max_k = target - s;
                for (int k = 0; k <= max_k; k++) {
                    new_dp[s + k] += dp[s];
                }
            }
        }
        free(dp);
        dp = new_dp;
    }

    long result = dp[target];
    free(dp);
    return result;
}

long ans = 0;

void helper(int min_index, int *exponents, int exp_len, long n) {
    ans += count_combinations(exponents, exp_len);

    for (int index = min_index; index < prime_count; index++) {
        int p = primes[index];
        if (n * (long)p > N) break;

        long prod = 1;
        int e = 1;
        while (1) {
            prod *= p;
            if (n * prod > N) break;
            exponents[exp_len] = e;
            helper(index + 1, exponents, exp_len + 1, n * prod);
            e++;
        }
    }
}

int main(void) {
    sieve();
    int exponents[MAX_EXPONENTS];
    helper(0, exponents, 0, 1);
    printf("%ld\n", ans);
    return 0;
}

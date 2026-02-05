#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>
#include <math.h>

#define N 20000000
#define MOD 1000000000U

int main(void) {
    int limit = N + 1;
    uint8_t *is_prime = malloc((size_t)limit + 1);
    if (!is_prime) {
        fprintf(stderr, "Allocation failed for is_prime\n");
        return 1;
    }
    memset(is_prime, 1, (size_t)limit + 1);
    is_prime[0] = 0;
    is_prime[1] = 0;
    int sqrt_limit = (int)sqrt((double)limit);
    for (int i = 2; i <= sqrt_limit; i++) {
        if (is_prime[i]) {
            for (int j = i * i; j <= limit; j += i) {
                is_prime[j] = 0;
            }
        }
    }

    double *logs = calloc((size_t)N, sizeof(double));
    uint32_t *mods = malloc((size_t)N * sizeof(uint32_t));
    if (!logs || !mods) {
        fprintf(stderr, "Allocation failed for arrays\n");
        free(is_prime);
        free(logs);
        free(mods);
        return 1;
    }
    for (int i = 0; i < N; i++) {
        mods[i] = 1U;
    }

    // Handle p = 2 separately.
    double log2v = log(2.0);
    for (int n = 1; n < N; n++) {
        logs[n] += log2v;
        mods[n] = (uint32_t)(((uint64_t)mods[n] * 2U) % MOD);
    }
    for (int n = 2; n < N; n += 2) {
        logs[n] += log2v;
        mods[n] = (uint32_t)(((uint64_t)mods[n] * 2U) % MOD);
    }
    for (int pe = 2; pe < N; pe <<= 1) {
        for (int n = pe; n < N; n += pe) {
            logs[n] += log2v;
            mods[n] = (uint32_t)(((uint64_t)mods[n] * 2U) % MOD);
        }
    }

    // Odd primes.
    for (int p = 3; p <= N; p += 2) {
        if (!is_prime[p]) continue;
        int d = p - 1;
        double logp = log((double)p);

        for (int n = d; n < N; n += d) {
            logs[n] += logp;
            mods[n] = (uint32_t)(((uint64_t)mods[n] * (uint64_t)p) % MOD);
        }

        long long pe = p;
        while ((long long)d * pe < N) {
            long long step = (long long)d * pe;
            for (long long n = step; n < N; n += step) {
                logs[(int)n] += logp;
                mods[(int)n] = (uint32_t)(((uint64_t)mods[(int)n] * (uint64_t)p) % MOD);
            }
            if (pe > (long long)(N - 1) / p) break;
            pe *= p;
        }
    }

    double best_log = -1.0;
    uint32_t best_mod = 0;
    for (int n = 1; n < N; n++) {
        if (logs[n] > best_log) {
            best_log = logs[n];
            best_mod = mods[n];
        }
    }

    uint64_t answer = (uint64_t)best_mod + 1U;
    printf("%llu\n", (unsigned long long)answer);

    free(is_prime);
    free(logs);
    free(mods);
    return 0;
}

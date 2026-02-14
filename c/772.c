/*
 * Project Euler 772 - Balanceable Partitions
 *
 * The answer is 2 * LCM(1..N) mod (10^9+7).
 * Uses sieve of Eratosthenes to find primes, then computes
 * the product of p^floor(log_p(N)) for each prime p <= N.
 * Extracted from embedded C in Python solution.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>

typedef __int128 int128;

static int64_t pow_mod(int64_t base, int64_t exp, int64_t mod) {
    int64_t result = 1;
    base %= mod;
    while (exp > 0) {
        if (exp & 1)
            result = (int128)result * base % mod;
        base = (int128)base * base % mod;
        exp >>= 1;
    }
    return result;
}

int main() {
    int64_t N = 100000000LL;  /* 10^8 */
    int64_t M = 1000000007LL;

    /* Sieve of Eratosthenes */
    unsigned char *is_prime = (unsigned char *)malloc((size_t)(N + 1));
    if (!is_prime) {
        fprintf(stderr, "alloc fail\n");
        return 1;
    }
    memset(is_prime, 1, (size_t)(N + 1));
    is_prime[0] = 0;
    is_prime[1] = 0;

    int64_t sq = 1;
    while ((sq + 1) * (sq + 1) <= N) sq++;

    for (int64_t i = 2; i <= sq; i++) {
        if (is_prime[i]) {
            for (int64_t j = i * i; j <= N; j += i)
                is_prime[j] = 0;
        }
    }

    /* Compute 2 * LCM(1..N) mod M */
    int64_t ans = 2;

    for (int64_t p = 2; p <= N; p++) {
        if (!is_prime[p]) continue;

        int64_t pe = p;
        int exp = 1;
        while (pe <= N / p) {
            pe *= p;
            exp++;
        }

        ans = (int128)ans * pow_mod(p, exp, M) % M;
    }

    printf("%lld\n", ans);
    free(is_prime);
    return 0;
}

/*
 * Project Euler 787 - Bezout's Game
 *
 * Uses Mertens function with hash-based memoization and sum_odd.
 * Extracted from embedded C in Python solution.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <math.h>

typedef int64_t i64;
typedef __int128 i128;

#define N 1000000000LL

int SIEVE_LIMIT;
signed char *mobius;
i64 *mertens_prefix;
i64 *sum_odd_prefix;

void sieve(int limit) {
    SIEVE_LIMIT = limit;
    mobius = calloc(limit + 1, 1);
    mertens_prefix = calloc(limit + 1, sizeof(i64));
    sum_odd_prefix = calloc(limit + 1, sizeof(i64));

    for (int i = 0; i <= limit; i++) mobius[i] = 1;
    char *is_prime = calloc(limit + 2, 1);
    memset(is_prime, 1, limit + 2);
    is_prime[0] = is_prime[1] = 0;

    for (int i = 2; i <= limit; i++) {
        if (is_prime[i]) {
            for (int j = i; j <= limit; j += i) is_prime[j] = 0;
            for (i64 j = (i64)i * i; j <= limit; j += (i64)i * i) mobius[j] = 0;
            for (int j = i; j <= limit; j += i) mobius[j] = -mobius[j];
        }
    }
    free(is_prime);

    mertens_prefix[0] = 0;
    sum_odd_prefix[0] = 0;
    for (int i = 1; i <= limit; i++) {
        mertens_prefix[i] = mertens_prefix[i-1] + mobius[i];
        sum_odd_prefix[i] = sum_odd_prefix[i-1] + ((i & 1) ? mobius[i] : 0);
    }
}

#define HASH_SIZE 2000003
i64 mertens_keys[HASH_SIZE];
i64 mertens_vals[HASH_SIZE];
char mertens_used[HASH_SIZE];

i64 sum_odd_keys[HASH_SIZE];
i64 sum_odd_vals[HASH_SIZE];
char sum_odd_used[HASH_SIZE];

i64 mertens(i64 n);
i64 sum_odd(i64 n);

i64 mertens_cached(i64 n) {
    if (n <= SIEVE_LIMIT) return mertens_prefix[n];

    unsigned int idx = (unsigned int)(((unsigned long long)n * 2654435761ULL) % HASH_SIZE);
    while (mertens_used[idx] && mertens_keys[idx] != n) idx = (idx + 1) % HASH_SIZE;
    if (mertens_used[idx]) return mertens_vals[idx];

    i64 val = mertens(n);
    mertens_keys[idx] = n;
    mertens_vals[idx] = val;
    mertens_used[idx] = 1;
    return val;
}

i64 mertens(i64 n) {
    if (n <= SIEVE_LIMIT) return mertens_prefix[n];

    i64 sqrtn = (i64)sqrtl((long double)n);
    i64 result = 1;

    for (i64 k = 2; k <= n / (sqrtn + 1); k++) {
        result -= mertens_cached(n / k);
    }

    for (i64 q = 1; q <= sqrtn; q++) {
        i64 kmin = n / (q + 1) + 1;
        i64 kmax = n / q;
        if (kmin <= n / (sqrtn + 1)) kmin = n / (sqrtn + 1) + 1;
        if (kmax >= kmin) {
            result -= (kmax - kmin + 1) * mertens_cached(q);
        }
    }

    return result;
}

i64 sum_odd_cached(i64 n) {
    if (n <= SIEVE_LIMIT) return sum_odd_prefix[n];

    unsigned int idx = (unsigned int)(((unsigned long long)n * 2654435769ULL) % HASH_SIZE);
    while (sum_odd_used[idx] && sum_odd_keys[idx] != n) idx = (idx + 1) % HASH_SIZE;
    if (sum_odd_used[idx]) return sum_odd_vals[idx];

    i64 val = sum_odd(n);
    sum_odd_keys[idx] = n;
    sum_odd_vals[idx] = val;
    sum_odd_used[idx] = 1;
    return val;
}

i64 sum_odd(i64 n) {
    if (n <= SIEVE_LIMIT) return sum_odd_prefix[n];
    return mertens_cached(n) + sum_odd_cached(n / 2);
}

i64 tr(i64 n) {
    return n * (n + 1) / 2;
}

int main() {
    int L = (int)sqrtl((long double)N) + 1;
    sieve(L + 100);
    memset(mertens_used, 0, sizeof(mertens_used));
    memset(sum_odd_used, 0, sizeof(sum_odd_used));

    i128 ans = 0;

    /* Direct: g from 1 to N/L (odd g only), quotient t = N/g >= L */
    for (i64 g = 1; g <= N / L; g += 2) {
        i64 t = N / g;
        ans += (i128)mobius[g] * (tr(t) / 2);
    }

    /* Batch: for quotient t from 1 to L-1 */
    for (i64 t = 1; t < L; t++) {
        i64 upper = N / t;
        i64 lower = N / (t + 1);
        i64 coeff = sum_odd_cached(upper) - sum_odd_cached(lower);
        ans += (i128)coeff * (tr(t) / 2);
    }

    printf("%lld\n", (long long)ans);
    return 0;
}

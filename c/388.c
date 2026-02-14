/*
 * Project Euler Problem 388: Distinct Lines through lattice points
 *
 * D(N) = sum_{d=1}^N mu(d) * (floor(N/d) + 1)^3 - M(N)
 * where M(N) is the Mertens function.
 *
 * Uses O(N^{2/3}) Mertens via Lucy DP.
 * Output: first 9 digits concatenated with last 9 digits.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

#define N_VAL 10000000000LL  /* 10^10 */

static int *mu;
static long long *mu_prefix;
static int *is_composite;
static int *primes;
static int nprime;

/* Mertens cache for large values */
#define MC_SIZE (1 << 18)
#define MC_MASK (MC_SIZE - 1)
typedef struct { long long key; long long val; int used; } MCEntry;
static MCEntry mc[MC_SIZE];

static void mc_init(void) {
    for (int i = 0; i < MC_SIZE; i++) mc[i].used = 0;
}

static int mc_get(long long key, long long *val) {
    unsigned long long h = (unsigned long long)key;
    h = (h ^ (h >> 30)) * 0xbf58476d1ce4e5b9ULL;
    int idx = (int)((h ^ (h >> 27)) & MC_MASK);
    for (int i = 0; i < 64; i++) {
        int slot = (idx + i) & MC_MASK;
        if (!mc[slot].used) return 0;
        if (mc[slot].key == key) { *val = mc[slot].val; return 1; }
    }
    return 0;
}

static void mc_put(long long key, long long val) {
    unsigned long long h = (unsigned long long)key;
    h = (h ^ (h >> 30)) * 0xbf58476d1ce4e5b9ULL;
    int idx = (int)((h ^ (h >> 27)) & MC_MASK);
    for (int i = 0; i < 64; i++) {
        int slot = (idx + i) & MC_MASK;
        if (!mc[slot].used || mc[slot].key == key) {
            mc[slot].key = key;
            mc[slot].val = val;
            mc[slot].used = 1;
            return;
        }
    }
    mc[idx].key = key;
    mc[idx].val = val;
    mc[idx].used = 1;
}

static int sieve_limit;

static long long mertens(long long n) {
    if (n <= sieve_limit) return mu_prefix[n];
    long long cached;
    if (mc_get(n, &cached)) return cached;

    long long s = 0;
    long long d = 2;
    while (d <= n) {
        long long q = n / d;
        long long d_max = n / q;
        s += (d_max - d + 1) * mertens(q);
        d = d_max + 1;
    }

    long long result = 1 - s;
    mc_put(n, result);
    return result;
}

int main(void) {
    long long N = N_VAL;

    /* Compute sieve limit as max(cbrt(N)^2, sqrt(N)+1) */
    int cbrt_n = (int)cbrt((double)N);
    while ((long long)(cbrt_n + 1) * (cbrt_n + 1) * (cbrt_n + 1) <= N) cbrt_n++;
    while ((long long)cbrt_n * cbrt_n * cbrt_n > N) cbrt_n--;

    long long sqrt_n = (long long)sqrt((double)N);
    while (sqrt_n * sqrt_n > N) sqrt_n--;
    while ((sqrt_n + 1) * (sqrt_n + 1) <= N) sqrt_n++;

    sieve_limit = (int)(cbrt_n * cbrt_n);
    if (sieve_limit < sqrt_n + 1) sieve_limit = (int)(sqrt_n + 1);

    /* Linear sieve for mu */
    mu = calloc(sieve_limit + 1, sizeof(int));
    is_composite = calloc(sieve_limit + 1, sizeof(int));
    primes = calloc(sieve_limit / 2, sizeof(int));
    mu_prefix = calloc(sieve_limit + 1, sizeof(long long));

    mu[1] = 1;
    nprime = 0;
    for (int i = 2; i <= sieve_limit; i++) {
        if (!is_composite[i]) {
            primes[nprime++] = i;
            mu[i] = -1;
        }
        for (int j = 0; j < nprime; j++) {
            long long v = (long long)primes[j] * i;
            if (v > sieve_limit) break;
            is_composite[(int)v] = 1;
            if (i % primes[j] == 0) {
                mu[(int)v] = 0;
                break;
            }
            mu[(int)v] = -mu[i];
        }
    }

    for (int i = 1; i <= sieve_limit; i++) {
        mu_prefix[i] = mu_prefix[i - 1] + mu[i];
    }

    mc_init();

    /* Compute M(N) first */
    long long M_N = mertens(N);

    /* Main sum using __int128 to avoid overflow */
    __int128 main_sum = 0;
    long long d_start = 1;
    while (d_start <= N) {
        long long q = N / d_start;
        long long d_max = N / q;

        long long mu_range;
        if (d_max <= sieve_limit) {
            mu_range = mu_prefix[d_max] - mu_prefix[d_start - 1];
        } else if (d_start <= sieve_limit) {
            mu_range = mu_prefix[sieve_limit] - mu_prefix[d_start - 1];
            mu_range += mertens(d_max) - mu_prefix[sieve_limit];
        } else {
            mu_range = mertens(d_max) - mertens(d_start - 1);
        }

        __int128 cube = (__int128)(q + 1) * (q + 1) * (q + 1);
        main_sum += mu_range * cube;
        d_start = d_max + 1;
    }

    __int128 result = main_sum - M_N;

    /* Convert to string */
    char buf[60];
    int neg = 0;
    __int128 r = result;
    if (r < 0) { neg = 1; r = -r; }
    int pos = 59;
    buf[pos] = 0;
    if (r == 0) { buf[--pos] = '0'; }
    while (r > 0) {
        buf[--pos] = '0' + (int)(r % 10);
        r /= 10;
    }
    if (neg) buf[--pos] = '-';
    char *s = buf + pos;

    /* Print first 9 and last 9 digits */
    int len = strlen(s);
    if (len <= 18) {
        printf("%s\n", s);
    } else {
        /* First 9 digits */
        char first9[10];
        strncpy(first9, s, 9);
        first9[9] = 0;
        /* Last 9 digits */
        char *last9 = s + len - 9;
        printf("%s%s\n", first9, last9);
    }

    free(mu);
    free(is_composite);
    free(primes);
    free(mu_prefix);
    return 0;
}

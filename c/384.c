/*
 * Project Euler Problem 384: Rudin-Shapiro sequence
 *
 * s(n) = summatory Rudin-Shapiro function.
 * count(n, val) = occurrences of val in s(0)..s(n).
 * g(t, c) = index where t appears for c-th time.
 * Answer = sum_{t=2}^{45} g(F(t+1), F(t)).
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* ---- s_cache: maps long long -> long long ---- */
#define S_SIZE (1 << 20)
#define S_MASK (S_SIZE - 1)

typedef struct { long long key; long long val; char used; } SEntry;
static SEntry s_cache[S_SIZE];

static unsigned int shash(long long k) {
    unsigned long long h = (unsigned long long)k;
    h ^= h >> 33;
    h *= 0xff51afd7ed558ccdULL;
    h ^= h >> 33;
    h *= 0xc4ceb9fe1a85ec53ULL;
    h ^= h >> 33;
    return (unsigned int)(h & S_MASK);
}

static int s_get(long long key, long long *val) {
    unsigned int h = shash(key);
    for (;;) {
        if (!s_cache[h].used) return 0;
        if (s_cache[h].key == key) { *val = s_cache[h].val; return 1; }
        h = (h + 1) & S_MASK;
    }
}

static void s_put(long long key, long long val) {
    unsigned int h = shash(key);
    for (;;) {
        if (!s_cache[h].used) {
            s_cache[h].key = key; s_cache[h].val = val; s_cache[h].used = 1;
            return;
        }
        if (s_cache[h].key == key) { s_cache[h].val = val; return; }
        h = (h + 1) & S_MASK;
    }
}

static int bit_length(long long n) {
    int b = 0;
    while (n > 0) { b++; n >>= 1; }
    return b;
}

static long long s_func(long long n) {
    if (n <= 1) return n + 1;
    long long cached;
    if (s_get(n, &cached)) return cached;

    long long x = 1LL << (bit_length(n) - 1);
    long long result;
    if (n >= x + x / 2) {
        result = s_func(x - 1) + 2 * s_func(x / 2 - 1) - s_func(n - x);
    } else {
        result = s_func(x - 1) + s_func(n - x);
    }
    s_put(n, result);
    return result;
}

/* ---- count cache: maps (n, val) -> long long ---- */
/* Use two fields for the key to avoid overflow */
#define C_SIZE (1 << 22)
#define C_MASK (C_SIZE - 1)

typedef struct { long long n; long long v; long long result; char used; } CEntry;
static CEntry *c_cache;

static unsigned int chash(long long n, long long v) {
    unsigned long long h = (unsigned long long)n;
    h ^= h >> 33;
    h *= 0xff51afd7ed558ccdULL;
    h ^= h >> 33;
    h ^= (unsigned long long)v * 0x9e3779b97f4a7c15ULL;
    h ^= h >> 33;
    h *= 0xc4ceb9fe1a85ec53ULL;
    h ^= h >> 33;
    return (unsigned int)(h & C_MASK);
}

static int c_get(long long n, long long val, long long *result) {
    unsigned int h = chash(n, val);
    for (;;) {
        if (!c_cache[h].used) return 0;
        if (c_cache[h].n == n && c_cache[h].v == val) {
            *result = c_cache[h].result;
            return 1;
        }
        h = (h + 1) & C_MASK;
    }
}

static void c_put(long long n, long long val, long long result) {
    unsigned int h = chash(n, val);
    for (;;) {
        if (!c_cache[h].used) {
            c_cache[h].n = n; c_cache[h].v = val;
            c_cache[h].result = result; c_cache[h].used = 1;
            return;
        }
        if (c_cache[h].n == n && c_cache[h].v == val) {
            c_cache[h].result = result;
            return;
        }
        h = (h + 1) & C_MASK;
    }
}

static long long isqrt_ll(long long n) {
    if (n <= 0) return 0;
    long long x = 1;
    while (x * x <= n && x < (1LL << 31)) x *= 2;
    long long lo = x / 2, hi = x;
    while (lo < hi) {
        long long mid = lo + (hi - lo) / 2;
        if (mid <= n / mid) lo = mid + 1;
        else hi = mid;
    }
    return lo - 1;
}

static long long count_func(long long n, long long val) {
    if (val < 0 || (n >= 0 && val > 6 * isqrt_ll(n + 1))) return 0;
    if (n == -1) return (val == 0) ? 1 : 0;

    long long cached;
    if (c_get(n, val, &cached)) return cached;

    long long x = 1LL << (bit_length(n + 1) - 1);
    long long c = count_func(x - 2, val);

    if (n >= x + x / 2) {
        long long s_x = s_func(x - 1);
        long long s_mid = s_func(x + x / 2 - 1);
        long long mirror_val = 2 * s_mid - s_x - val;
        c += count_func(x / 2 - 2, val - s_x)
           + count_func(n - x, mirror_val)
           - count_func(x / 2 - 2, mirror_val);
    } else {
        c += count_func(n - x, val - s_func(x - 1));
    }

    c_put(n, val, c);
    return c;
}

static long long g_func(long long t, long long c_target) {
    /* Find upper bound */
    long long hi = 1;
    while (count_func(hi, t) < c_target) {
        if (hi > (1LL << 62)) break;
        hi *= 2;
    }

    long long lo = 0;
    while (lo + 1 < hi) {
        long long mid = lo + (hi - lo) / 2;
        if (count_func(mid, t) < c_target) {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    return hi;
}

int main(void) {
    c_cache = calloc(C_SIZE, sizeof(CEntry));
    memset(s_cache, 0, sizeof(s_cache));

    /* Fibonacci */
    long long fib[50];
    fib[0] = 0; fib[1] = 1;
    for (int i = 2; i < 50; i++) fib[i] = fib[i-1] + fib[i-2];

    long long total = 0;
    for (int t = 2; t <= 45; t++) {
        /* Clear count cache between iterations to avoid overflow */
        memset(c_cache, 0, C_SIZE * sizeof(CEntry));
        memset(s_cache, 0, sizeof(s_cache));
        long long result = g_func(fib[t + 1], fib[t]);
        total += result;
    }
    printf("%lld\n", total);
    free(c_cache);
    return 0;
}

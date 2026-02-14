/*
 * Project Euler Problem 297: Zeckendorf Representation
 *
 * Z(n) = sum of Zeckendorf representation lengths for 0 < k < n.
 * Recursive with memoization using Fibonacci decomposition.
 */
#include <stdio.h>
#include <string.h>

#define MAX_FIBS 90

static long long fibs[MAX_FIBS];
static int nfibs;

/* Memoization: cache Z(fib[i]) for each Fibonacci number */
/* Also need general cache - use array since values are Fibonacci-indexed */

/* Actually, the recursion only ever calls Z on values that are
 * sums of Fibonacci numbers, which means the cache won't be too large.
 * But for 10^17 we need a hash map approach. */

#define CACHE_SIZE (1 << 20)
#define CACHE_MASK (CACHE_SIZE - 1)

typedef struct {
    long long key;
    long long val;
    int used;
} CacheEntry;

static CacheEntry cache[CACHE_SIZE];

static void cache_put(long long key, long long val) {
    unsigned long long h = (unsigned long long)key;
    h = (h ^ (h >> 30)) * 0xbf58476d1ce4e5b9ULL;
    h = (h ^ (h >> 27)) * 0x94d049bb133111ebULL;
    h = h ^ (h >> 31);
    int idx = (int)(h & CACHE_MASK);
    /* Linear probing */
    for (int i = 0; i < 100; i++) {
        int slot = (idx + i) & CACHE_MASK;
        if (!cache[slot].used || cache[slot].key == key) {
            cache[slot].key = key;
            cache[slot].val = val;
            cache[slot].used = 1;
            return;
        }
    }
    /* Evict */
    cache[idx].key = key;
    cache[idx].val = val;
    cache[idx].used = 1;
}

static int cache_get(long long key, long long *val) {
    unsigned long long h = (unsigned long long)key;
    h = (h ^ (h >> 30)) * 0xbf58476d1ce4e5b9ULL;
    h = (h ^ (h >> 27)) * 0x94d049bb133111ebULL;
    h = h ^ (h >> 31);
    int idx = (int)(h & CACHE_MASK);
    for (int i = 0; i < 100; i++) {
        int slot = (idx + i) & CACHE_MASK;
        if (!cache[slot].used) return 0;
        if (cache[slot].key == key) {
            *val = cache[slot].val;
            return 1;
        }
    }
    return 0;
}

static long long Z(long long n) {
    if (n <= 1) return 0;

    long long val;
    if (cache_get(n, &val)) return val;

    /* Find largest Fibonacci number < n */
    long long a = 1;
    for (int i = 0; i < nfibs; i++) {
        if (fibs[i] < n) a = fibs[i];
        else break;
    }

    long long result = Z(a) + (n - a) + Z(n - a);
    cache_put(n, result);
    return result;
}

int main(void) {
    long long N = 100000000000000000LL; /* 10^17 */

    /* Generate Fibonacci sequence */
    fibs[0] = 1;
    fibs[1] = 2;
    nfibs = 2;
    while (fibs[nfibs - 1] < N) {
        fibs[nfibs] = fibs[nfibs - 1] + fibs[nfibs - 2];
        nfibs++;
    }

    memset(cache, 0, sizeof(cache));

    printf("%lld\n", Z(N));
    return 0;
}

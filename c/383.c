/*
 * Project Euler Problem 383: Divisibility comparison between factorials
 *
 * Find count of i <= 10^18 where v5((2i-1)!) < 2*v5(i!).
 * Digit recursion in base 5 with memoization.
 */
#include <stdio.h>
#include <string.h>

#define BASE 5

/* Cache: key = (n, diff, total_diff)
 * n can be up to 10^18 / 5^k, diff in small range, total_diff in small range.
 * The recursion reduces n by factor 5 each step, so depth is ~26.
 * At each level, diff and total_diff are bounded.
 */

/* Use hash map */
#define HT_SIZE (1 << 22)
#define HT_MASK (HT_SIZE - 1)

typedef struct {
    long long n;
    int diff;
    int total_diff;
    long long result;
    int used;
} CEntry;

static CEntry cache[HT_SIZE];

static unsigned int hash3(long long n, int diff, int td) {
    unsigned long long h = (unsigned long long)n;
    h = h * 2654435761ULL + (unsigned)(diff + 200);
    h = h * 2654435761ULL + (unsigned)(td + 2000);
    h = (h ^ (h >> 30)) * 0xbf58476d1ce4e5b9ULL;
    return (unsigned int)((h ^ (h >> 27))) & HT_MASK;
}

static int cache_get(long long n, int diff, int td, long long *result) {
    unsigned int h = hash3(n, diff, td);
    for (int i = 0; i < 32; i++) {
        int slot = (h + i) & HT_MASK;
        if (!cache[slot].used) return 0;
        if (cache[slot].n == n && cache[slot].diff == diff && cache[slot].total_diff == td) {
            *result = cache[slot].result;
            return 1;
        }
    }
    return 0;
}

static void cache_put(long long n, int diff, int td, long long result) {
    unsigned int h = hash3(n, diff, td);
    for (int i = 0; i < 32; i++) {
        int slot = (h + i) & HT_MASK;
        if (!cache[slot].used || (cache[slot].n == n && cache[slot].diff == diff && cache[slot].total_diff == td)) {
            cache[slot].n = n;
            cache[slot].diff = diff;
            cache[slot].total_diff = td;
            cache[slot].result = result;
            cache[slot].used = 1;
            return;
        }
    }
    /* Evict */
    cache[h].n = n;
    cache[h].diff = diff;
    cache[h].total_diff = td;
    cache[h].result = result;
    cache[h].used = 1;
}

/* floor division toward negative infinity */
static int floor_div(int a, int b) {
    /* b > 0 always (b = BASE = 5) */
    if (a >= 0) return a / b;
    return (a - b + 1) / b;
}

static long long T(long long n, int diff, int total_diff) {
    if (n <= 0) {
        return (n == 0 && total_diff > 0) ? 1 : 0;
    }

    long long cached;
    if (cache_get(n, diff, total_diff, &cached)) return cached;

    long long result = 0;
    int n_mod = (int)(n % BASE);

    for (int r = 0; r < BASE; r++) {
        int new_diff = -floor_div(2 * r - diff, BASE);
        long long n_next = n / BASE - (r > n_mod ? 1 : 0);
        result += T(n_next, new_diff, total_diff + new_diff);
    }

    cache_put(n, diff, total_diff, result);
    return result;
}

int main(void) {
    long long N = 1000000000000000000LL; /* 10^18 */
    memset(cache, 0, sizeof(cache));

    long long ans = T(N, 1, 0) - 1;
    printf("%lld\n", ans);
    return 0;
}

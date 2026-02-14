/*
 * Project Euler 776 - Digit Sum Division
 *
 * Find sum_{n=1}^N n/d(n) where d(n) = digit sum of n.
 * N = 1234567890123456789
 *
 * Uses digit DP: f(n, s) returns (count, sum) of numbers <= n with digit sum s.
 * The recursion processes digits from most significant to least.
 *
 * Uses memoization with a hash table keyed on (n, s).
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>

/* We need to return both count and sum. Sum can be very large (up to ~N^2/2),
 * so we use long double for the sum (since the answer is a float anyway). */

typedef struct {
    int64_t count;
    long double total_sum;
} Result;

/* Memoization hash table */
/* Key: (n, sum_digits) where n can be up to ~10^18 / 10^k */
#define MEMO_SIZE (1 << 20)
#define MEMO_MASK (MEMO_SIZE - 1)

typedef struct MemoEntry {
    int64_t n;
    int s;
    Result val;
    int next;
    int valid;
} MemoEntry;

static MemoEntry *memo_entries;
static int *memo_buckets;
static int memo_count;

static void memo_init(void) {
    memo_entries = (MemoEntry *)malloc(MEMO_SIZE * 2 * sizeof(MemoEntry));
    memo_buckets = (int *)malloc(MEMO_SIZE * sizeof(int));
    memset(memo_buckets, -1, MEMO_SIZE * sizeof(int));
    memo_count = 0;
}

static uint32_t memo_hash(int64_t n, int s) {
    uint64_t h = (uint64_t)n * 2654435761ULL + (uint64_t)s * 40503ULL;
    return (uint32_t)(h & MEMO_MASK);
}

static int memo_find(int64_t n, int s, Result *out) {
    uint32_t bucket = memo_hash(n, s);
    for (int idx = memo_buckets[bucket]; idx != -1; idx = memo_entries[idx].next) {
        if (memo_entries[idx].n == n && memo_entries[idx].s == s) {
            *out = memo_entries[idx].val;
            return 1;
        }
    }
    return 0;
}

static void memo_insert(int64_t n, int s, Result val) {
    uint32_t bucket = memo_hash(n, s);
    int idx = memo_count++;
    memo_entries[idx].n = n;
    memo_entries[idx].s = s;
    memo_entries[idx].val = val;
    memo_entries[idx].next = memo_buckets[bucket];
    memo_buckets[bucket] = idx;
}

static Result f(int64_t n, int s) {
    Result r = {0, 0.0L};
    if (n == 0 && s == 0)
        return (Result){1, 0.0L};
    if (n <= 0 || s < 0)
        return r;

    Result cached;
    if (memo_find(n, s, &cached))
        return cached;

    for (int d = 0; d < 10; d++) {
        int64_t n_prime = n / 10 - (d > n % 10 ? 1 : 0);
        if (s - d < 0) continue;
        Result sub = f(n_prime, s - d);
        r.count += sub.count;
        r.total_sum += sub.total_sum * 10.0L + (long double)d * sub.count;
    }

    memo_insert(n, s, r);
    return r;
}

int main(void) {
    int64_t N = 1234567890123456789LL;
    int max_digit_sum = 9 * 19; /* 19 digits * 9 = 171 */

    memo_init();

    long double ans = 0.0L;
    for (int s = 1; s < max_digit_sum; s++) {
        Result val = f(N, s);
        if (val.count > 0)
            ans += val.total_sum / (long double)s;
    }

    /* Format as scientific notation matching expected output */
    /* Expected: 9.627509725002e33 */
    char buf[100];
    sprintf(buf, "%.12Le", ans);
    /* Remove '+' from exponent */
    char out[100];
    int j = 0;
    for (int i = 0; buf[i]; i++) {
        if (buf[i] == '+') continue;
        out[j++] = buf[i];
    }
    out[j] = '\0';
    printf("%s\n", out);

    free(memo_entries);
    free(memo_buckets);
    return 0;
}

/* Project Euler Problem 755: Not Zeckendorf.
 * Translated from python/755.py
 *
 * Count ways to express numbers up to N as sums of distinct Fibonacci numbers.
 * Uses memoized recursion over Fibonacci index and remaining budget.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef long long ll;

#define MAX_FIBS 80

static ll fibs[MAX_FIBS];
static int num_fibs;

/* Generate Fibonacci numbers */
void gen_fibs(void) {
    fibs[0] = fibs[1] = 1;
    num_fibs = 2;
    while (fibs[num_fibs - 1] < 1000000000000000LL) { /* 10^15 */
        fibs[num_fibs] = fibs[num_fibs - 1] + fibs[num_fibs - 2];
        num_fibs++;
    }
}

/*
 * We need to memoize helper(index, n).
 * index ranges 0..num_fibs-1 (~70), n can be up to 10^13.
 * The key insight: for a given index, n only takes O(sqrt(N)) distinct values
 * due to the recursive structure. We use a hash map.
 */

typedef struct {
    ll key;
    ll val;
    int used;
} HashEntry;

#define HASH_SIZE (1 << 20)
#define HASH_MASK (HASH_SIZE - 1)

typedef struct {
    HashEntry entries[HASH_SIZE];
} HashMap;

static HashMap *maps;

void hm_init(HashMap *hm) {
    memset(hm->entries, 0, sizeof(hm->entries));
}

int hm_get(HashMap *hm, ll key, ll *val) {
    unsigned h = (unsigned)((key ^ (key >> 17)) * 0x9e3779b97f4a7c15ULL) & HASH_MASK;
    for (int i = 0; i < 32; i++) {
        int idx = (h + i) & HASH_MASK;
        if (!hm->entries[idx].used) return 0;
        if (hm->entries[idx].key == key) {
            *val = hm->entries[idx].val;
            return 1;
        }
    }
    return 0;
}

void hm_put(HashMap *hm, ll key, ll val) {
    unsigned h = (unsigned)((key ^ (key >> 17)) * 0x9e3779b97f4a7c15ULL) & HASH_MASK;
    for (int i = 0; i < 32; i++) {
        int idx = (h + i) & HASH_MASK;
        if (!hm->entries[idx].used) {
            hm->entries[idx].key = key;
            hm->entries[idx].val = val;
            hm->entries[idx].used = 1;
            return;
        }
    }
    /* Should not happen if hash table is large enough */
}

ll helper(int index, ll n) {
    if (n < 0) return 0;
    if (index < 0) return 1;

    ll cached;
    if (hm_get(&maps[index], n, &cached))
        return cached;

    /* If sum of all remaining fibs (fibs[0]..fibs[index]) <= n+1,
     * then any subset works: 2^(index+1) subsets
     * fibs[index+2] - 1 is the sum of fibs[0]..fibs[index] (Zeckendorf identity)
     * Actually sum = fibs[index+2] - 1
     */
    if (index + 2 < num_fibs && fibs[index + 2] <= n + 2) {
        ll result = 1LL << (index + 1);
        /* But wait: 2^(index+1) might overflow for index~70.
         * Actually the answer is ~2.8*10^15, so this won't be called for large index. */
        hm_put(&maps[index], n, result);
        return result;
    }

    ll result = helper(index - 1, n) + helper(index - 1, n - fibs[index]);
    hm_put(&maps[index], n, result);
    return result;
}

int main() {
    ll N = 10000000000000LL; /* 10^13 */
    gen_fibs();

    maps = (HashMap *)calloc(num_fibs, sizeof(HashMap));
    for (int i = 0; i < num_fibs; i++)
        hm_init(&maps[i]);

    /* The answer is helper(len(fibs)-3, N) */
    ll result = helper(num_fibs - 3, N);
    printf("%lld\n", result);

    free(maps);
    return 0;
}

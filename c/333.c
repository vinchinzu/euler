/*
 * Project Euler Problem 333 - Special partitions
 *
 * Partitions into parts 2^i * 3^j (no part divides another).
 * Sum of primes q < 1000000 with exactly 1 special partition.
 *
 * Algorithm: generate terms, sort by (exp2 asc, exp3 desc).
 * Predecessors: term j can follow term i if exp2[i] < exp2[j] and exp3[i] > exp3[j].
 * DP using counts array: for each term, track reachable sums via predecessors.
 *
 * Uses a flat array DP instead of linked lists to avoid excessive memory.
 * For each term, maintain a boolean array of which sums are reachable (and their counts).
 * Since predecessors form a DAG, process in order and combine.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define LIMIT 1000000

typedef struct {
    int value;
    int exp2;
    int exp3;
} Term;

int term_cmp(const void *a, const void *b) {
    const Term *ta = (const Term *)a;
    const Term *tb = (const Term *)b;
    if (ta->exp2 != tb->exp2) return ta->exp2 - tb->exp2;
    return tb->exp3 - ta->exp3;
}

static char sieve[LIMIT];
static int counts[LIMIT + 1];

int main(void) {
    /* Sieve primes */
    memset(sieve, 1, LIMIT);
    sieve[0] = sieve[1] = 0;
    for (int i = 2; (long long)i * i < LIMIT; i++)
        if (sieve[i])
            for (int j = i * i; j < LIMIT; j += i)
                sieve[j] = 0;

    /* Generate terms */
    Term terms[200];
    int nterms = 0;
    long long v2 = 1;
    int e2 = 0;
    while (v2 <= LIMIT) {
        long long v3 = v2;
        int e3 = 0;
        while (v3 <= LIMIT) {
            if (v3 > 1) {
                terms[nterms].value = (int)v3;
                terms[nterms].exp2 = e2;
                terms[nterms].exp3 = e3;
                nterms++;
            }
            v3 *= 3;
            e3++;
        }
        v2 *= 2;
        e2++;
    }
    qsort(terms, nterms, sizeof(Term), term_cmp);

    /* Build predecessors */
    int preds[200][200];
    int npreds[200];
    memset(npreds, 0, sizeof(npreds));
    for (int j = 0; j < nterms; j++)
        for (int i = 0; i < j; i++)
            if (terms[i].exp2 < terms[j].exp2 && terms[i].exp3 > terms[j].exp3)
                preds[j][npreds[j]++] = i;

    /* DP using per-term count arrays.
     * dp[idx] is a dynamically allocated array of size (LIMIT+1) storing counts.
     * To save memory, only allocate for terms that have predecessors or where it's needed.
     * Actually, the Python uses dicts. In C, use sparse representation via sorted lists.
     */

    /* Use a simpler approach: for each term, maintain a list of (sum, count) pairs.
     * Use a dynamic array (growing as needed). */
    typedef struct { int sum; int count; } Pair;

    /* Pool for pairs */
    int pool_cap = 20000000;
    Pair *pool = (Pair *)malloc(pool_cap * sizeof(Pair));
    if (!pool) { fprintf(stderr, "malloc failed\n"); return 1; }
    int pool_used = 0;

    /* For each term: start index and count in pool */
    int dp_start[200];
    int dp_count[200];
    memset(dp_count, 0, sizeof(dp_count));

    memset(counts, 0, sizeof(counts));

    for (int idx = 0; idx < nterms; idx++) {
        int value = terms[idx].value;

        /* Temporary buffer: use a hash map approach with the counts array as workspace.
         * We'll use a separate temp array for this term's results. */

        /* Collect all (sum, count) pairs for this term */
        /* Start with just {value: 1} */
        /* Then merge in all predecessor entries + value */

        /* Use a temporary array to accumulate */
        /* Strategy: mark sums in a temporary array, then extract */
        int *temp = (int *)calloc(LIMIT + 1, sizeof(int));
        if (!temp) { fprintf(stderr, "calloc failed\n"); return 1; }

        if (value <= LIMIT)
            temp[value] = 1;

        for (int pi = 0; pi < npreds[idx]; pi++) {
            int pred = preds[idx][pi];
            for (int k = dp_start[pred]; k < dp_start[pred] + dp_count[pred]; k++) {
                int new_sum = pool[k].sum + value;
                if (new_sum <= LIMIT) {
                    temp[new_sum] += pool[k].count;
                }
            }
        }

        /* Store results in pool */
        dp_start[idx] = pool_used;
        dp_count[idx] = 0;
        for (int s = 1; s <= LIMIT; s++) {
            if (temp[s] > 0) {
                if (pool_used >= pool_cap) {
                    pool_cap *= 2;
                    pool = (Pair *)realloc(pool, pool_cap * sizeof(Pair));
                    if (!pool) { fprintf(stderr, "realloc failed\n"); return 1; }
                }
                pool[pool_used].sum = s;
                pool[pool_used].count = temp[s];
                pool_used++;
                dp_count[idx]++;
                counts[s] += temp[s];
            }
        }

        free(temp);
    }

    /* Sum primes with exactly 1 special partition */
    long long total = 0;
    for (int p = 2; p < LIMIT; p++)
        if (sieve[p] && counts[p] == 1)
            total += p;

    printf("%lld\n", total);
    free(pool);
    return 0;
}

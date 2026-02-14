/*
 * Project Euler Problem 333 - Special partitions
 *
 * Partitions into parts 2^i * 3^j. A partition is "special" if no part
 * divides another. Sum of primes q < 1000000 with exactly 1 special partition.
 *
 * Generate all terms 2^i * 3^j <= LIMIT (excluding 1).
 * Sort by (exp2 ascending, exp3 descending).
 * Predecessors: term j can follow term i if exp2[i] < exp2[j] and exp3[i] > exp3[j].
 * DP: for each term, accumulate sums from predecessors.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

#define LIMIT 1000000

/* Sieve primes */
static char sieve[LIMIT];

void make_sieve(void) {
    memset(sieve, 1, LIMIT);
    sieve[0] = sieve[1] = 0;
    for (int i = 2; (long long)i * i < LIMIT; i++)
        if (sieve[i])
            for (int j = i * i; j < LIMIT; j += i)
                sieve[j] = 0;
}

/* Terms: 2^e2 * 3^e3, excluding 1 */
typedef struct {
    int value;
    int exp2;
    int exp3;
} Term;

int term_cmp(const void *a, const void *b) {
    const Term *ta = (const Term *)a;
    const Term *tb = (const Term *)b;
    if (ta->exp2 != tb->exp2) return ta->exp2 - tb->exp2;
    return tb->exp3 - ta->exp3; /* descending exp3 */
}

/* For DP: each term has a dictionary of (sum -> count).
 * Use a simple linked list of (sum, count) pairs. */
typedef struct Entry {
    int sum;
    int count;
    struct Entry *next;
} Entry;

/* Pool allocator for entries */
#define POOL_SIZE 40000000
static Entry pool[POOL_SIZE];
static int pool_idx = 0;

Entry *alloc_entry(void) {
    return &pool[pool_idx++];
}

/* Counts array for accumulating */
static int counts[LIMIT + 1];

int main(void) {
    make_sieve();

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

    for (int j = 0; j < nterms; j++) {
        for (int i = 0; i < j; i++) {
            if (terms[i].exp2 < terms[j].exp2 && terms[i].exp3 > terms[j].exp3) {
                preds[j][npreds[j]++] = i;
            }
        }
    }

    /* DP using linked lists of (sum, count) */
    Entry *dp_heads[200];
    memset(dp_heads, 0, sizeof(dp_heads));
    memset(counts, 0, sizeof(counts));

    for (int idx = 0; idx < nterms; idx++) {
        int value = terms[idx].value;

        /* Start new partition with just this term */
        if (value <= LIMIT) {
            Entry *e = alloc_entry();
            e->sum = value;
            e->count = 1;
            e->next = dp_heads[idx];
            dp_heads[idx] = e;
            counts[value]++;
        }

        /* Extend from predecessors */
        for (int pi = 0; pi < npreds[idx]; pi++) {
            int pred = preds[idx][pi];
            for (Entry *e = dp_heads[pred]; e; e = e->next) {
                int new_sum = e->sum + value;
                if (new_sum > LIMIT) continue;
                Entry *ne = alloc_entry();
                ne->sum = new_sum;
                ne->count = e->count;
                ne->next = dp_heads[idx];
                dp_heads[idx] = ne;
                counts[new_sum] += e->count;
            }
        }
    }

    /* Sum primes with exactly 1 special partition */
    long long total = 0;
    for (int p = 2; p < LIMIT; p++) {
        if (sieve[p] && counts[p] == 1)
            total += p;
    }

    printf("%lld\n", total);
    return 0;
}

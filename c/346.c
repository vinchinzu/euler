/* Project Euler 346 - Strong Repunits
 *
 * A strong repunit is a positive integer that is a repunit in at least two
 * bases greater than 1. Find the sum of all strong repunits below 10^12.
 *
 * Every n >= 7 is 11 in base (n-1), so we only need to find numbers that
 * are repunits of length >= 3 in some base >= 2.
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define LIMIT 1000000000000LL
#define MAX_BASE 1000000

/* Hash set for storing found repunits */
#define HASH_SIZE (1 << 22)  /* ~4M buckets */
#define HASH_MASK (HASH_SIZE - 1)

typedef struct Node {
    long long val;
    struct Node *next;
} Node;

static Node *table[HASH_SIZE];
static Node pool[20000000];
static int pool_idx = 0;

int hash_contains(long long v) {
    unsigned int h = (unsigned int)((v ^ (v >> 32)) * 2654435761U) & HASH_MASK;
    for (Node *n = table[h]; n; n = n->next)
        if (n->val == v) return 1;
    return 0;
}

void hash_insert(long long v) {
    unsigned int h = (unsigned int)((v ^ (v >> 32)) * 2654435761U) & HASH_MASK;
    for (Node *n = table[h]; n; n = n->next)
        if (n->val == v) return;
    Node *n = &pool[pool_idx++];
    n->val = v;
    n->next = table[h];
    table[h] = n;
}

int main(void) {
    memset(table, 0, sizeof(table));

    /* For each base b >= 2, generate repunits of length >= 3 */
    for (long long b = 2; b <= MAX_BASE; b++) {
        /* rep = 1 + b + b^2 + ... = (b^k - 1)/(b-1) */
        /* Start with length 3: 1 + b + b^2 */
        long long rep = 1 + b + b * b;
        int len = 3;
        while (rep < LIMIT) {
            hash_insert(rep);
            rep = rep * b + 1;
            len++;
        }
    }

    /* Sum all unique repunits plus 1 (which is a strong repunit by convention) */
    long long total = 1; /* include 1 */
    for (int i = 0; i < HASH_SIZE; i++) {
        for (Node *n = table[i]; n; n = n->next) {
            total += n->val;
        }
    }

    printf("%lld\n", total);
    return 0;
}

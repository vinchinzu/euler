/*
 * Project Euler 269: Polynomials with at least one integer root
 *
 * DP over Horner evaluation values at 0..9 for polynomials of degree 15
 * with coefficients in [0,9].
 *
 * State: tuple of 10 values (one per evaluation point), where None
 * (represented as INT_MIN) means the value can never reach 0.
 *
 * Uses hash map for state compression.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <limits.h>

#define B 10
#define NSTEP 16
#define NONE INT_MIN

typedef struct {
    int vals[B];
} Key;

typedef struct Node {
    Key key;
    long long count;
    struct Node *next;
} Node;

#define HASH_SIZE (1 << 20)
#define HASH_MASK (HASH_SIZE - 1)

static Node *table1[HASH_SIZE];
static Node *table2[HASH_SIZE];
static Node pool1[8000000];
static Node pool2[8000000];
static int pool1_idx = 0;
static int pool2_idx = 0;

static unsigned hash_key(Key *k) {
    unsigned h = 0;
    for (int i = 0; i < B; i++) {
        h = h * 1000003 + (unsigned)(k->vals[i] + 1000000);
    }
    return h & HASH_MASK;
}

static int key_eq(Key *a, Key *b) {
    return memcmp(a->vals, b->vals, sizeof(a->vals)) == 0;
}

static void table_add(Node **tbl, Node *pool, int *pidx, Key *key, long long count) {
    unsigned h = hash_key(key);
    for (Node *n = tbl[h]; n; n = n->next) {
        if (key_eq(&n->key, key)) {
            n->count += count;
            return;
        }
    }
    Node *n = &pool[(*pidx)++];
    n->key = *key;
    n->count = count;
    n->next = tbl[h];
    tbl[h] = n;
}

int main(void) {
    /* Initial state: all values = 0 */
    Key init;
    for (int i = 0; i < B; i++) init.vals[i] = 0;

    Node **cur_tbl = table1;
    Node *cur_pool = pool1;
    int *cur_pidx = &pool1_idx;

    Node **new_tbl = table2;
    Node *new_pool = pool2;
    int *new_pidx = &pool2_idx;

    table_add(cur_tbl, cur_pool, cur_pidx, &init, 1);

    for (int step = 1; step <= NSTEP; step++) {
        memset(new_tbl, 0, HASH_SIZE * sizeof(Node *));
        *new_pidx = 0;

        /* Iterate over all entries in cur_tbl */
        for (int h = 0; h < HASH_SIZE; h++) {
            for (Node *node = cur_tbl[h]; node; node = node->next) {
                Key *key = &node->key;
                long long count = node->count;

                for (int d = 0; d < B; d++) {
                    Key nk;
                    for (int j = 0; j < B; j++) {
                        int v = key->vals[j];
                        if (v == NONE) {
                            nk.vals[j] = NONE;
                        } else {
                            int new_v = v * (-j) + d;
                            /* Check pruning conditions */
                            if (new_v > 0) {
                                /* (new_v * (-j) + B-1) * (-j) >= new_v */
                                int t = (new_v * (-j) + B - 1) * (-j);
                                if (t >= new_v) {
                                    nk.vals[j] = NONE;
                                } else {
                                    nk.vals[j] = new_v;
                                }
                            } else if (new_v < 0) {
                                /* (new_v * (-j) * (-j) + B-1) <= new_v */
                                int t = new_v * j * j + B - 1;
                                if (t <= new_v) {
                                    nk.vals[j] = NONE;
                                } else {
                                    nk.vals[j] = new_v;
                                }
                            } else {
                                nk.vals[j] = 0;
                            }
                        }
                    }
                    table_add(new_tbl, new_pool, new_pidx, &nk, count);
                }
            }
        }

        /* Swap tables */
        Node **tmp_tbl = cur_tbl;
        cur_tbl = new_tbl;
        new_tbl = tmp_tbl;

        Node *tmp_pool = cur_pool;
        cur_pool = new_pool;
        new_pool = tmp_pool;

        int *tmp_pidx = cur_pidx;
        cur_pidx = new_pidx;
        new_pidx = tmp_pidx;
    }

    /* Sum counts where at least one value is 0 */
    long long ans = 0;
    for (int h = 0; h < HASH_SIZE; h++) {
        for (Node *node = cur_tbl[h]; node; node = node->next) {
            int has_zero = 0;
            for (int j = 0; j < B; j++) {
                if (node->key.vals[j] == 0) { has_zero = 1; break; }
            }
            if (has_zero) ans += node->count;
        }
    }

    printf("%lld\n", ans);
    return 0;
}

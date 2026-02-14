/* Project Euler Problem 930 - Bowls and Balls
 * G(N, M) = sum F(n, m) for n=2..N, m=2..M
 * Uses DP over cosine-based state representation.
 * Output: scientific format with 12 significant digits.
 */
#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <string.h>

#define N_LIM 12
#define M_LIM 12
#define PI 3.14159265358979323846

/* Hash map for DP states */
/* State: (counts[num_unique], k_sum) -> ways (double) */
/* We'll use a different approach: array-based DP for small state spaces */

/* For each n, compute unique cosines and map k to idx */
/* max unique cosines for n<=12 is 7 (for n=12) */

typedef struct {
    int counts[13]; /* up to 13 unique cosine values */
    int k_sum;
} State;

/* Simple hash for states */
#define DP_HASH_SIZE (1 << 20)
#define DP_HASH_MASK (DP_HASH_SIZE - 1)

typedef struct DPEntry {
    State state;
    double ways;
    int used;
    struct DPEntry *next;
} DPEntry;

/* Use two DP tables */
static DPEntry *dp_pool = NULL;
static int dp_pool_idx = 0;
static int dp_pool_size = 0;

static DPEntry **dp_table = NULL;

void dp_init(void) {
    dp_pool_size = 1 << 20;
    dp_pool = (DPEntry *)calloc(dp_pool_size, sizeof(DPEntry));
    dp_pool_idx = 0;
    dp_table = (DPEntry **)calloc(DP_HASH_SIZE, sizeof(DPEntry *));
}

void dp_clear(void) {
    memset(dp_table, 0, DP_HASH_SIZE * sizeof(DPEntry *));
    dp_pool_idx = 0;
}

unsigned int state_hash(State *s, int num_unique, int n) {
    unsigned int h = s->k_sum * 31;
    for (int i = 0; i < num_unique; i++)
        h = h * 37 + s->counts[i];
    return h & DP_HASH_MASK;
}

int state_eq(State *a, State *b, int num_unique) {
    if (a->k_sum != b->k_sum) return 0;
    for (int i = 0; i < num_unique; i++)
        if (a->counts[i] != b->counts[i]) return 0;
    return 1;
}

void dp_add(State *s, double ways, int num_unique, int n) {
    unsigned int h = state_hash(s, num_unique, n);
    DPEntry *e = dp_table[h];
    while (e) {
        if (state_eq(&e->state, s, num_unique)) {
            e->ways += ways;
            return;
        }
        e = e->next;
    }
    /* New entry */
    if (dp_pool_idx >= dp_pool_size) {
        /* Grow pool */
        dp_pool_size *= 2;
        dp_pool = (DPEntry *)realloc(dp_pool, dp_pool_size * sizeof(DPEntry));
    }
    e = &dp_pool[dp_pool_idx++];
    e->state = *s;
    e->ways = ways;
    e->used = 1;
    e->next = dp_table[h];
    dp_table[h] = e;
}

/* Iterate over all entries */
typedef struct { State state; double ways; } DPItem;
static DPItem *dp_items = NULL;
static int dp_items_size = 0;

int dp_collect(void) {
    int cnt = 0;
    for (int i = 0; i < DP_HASH_SIZE; i++) {
        DPEntry *e = dp_table[i];
        while (e) {
            if (cnt >= dp_items_size) {
                dp_items_size = dp_items_size ? dp_items_size * 2 : (1 << 16);
                dp_items = (DPItem *)realloc(dp_items, dp_items_size * sizeof(DPItem));
            }
            dp_items[cnt].state = e->state;
            dp_items[cnt].ways = e->ways;
            cnt++;
            e = e->next;
        }
    }
    return cnt;
}

int main(void) {
    dp_init();
    double total_G = 0.0;

    for (int n = 2; n <= N_LIM; n++) {
        /* Compute unique cosines for this n */
        double cos_vals[13];
        int k_to_idx[13];
        int num_unique = 0;

        double unique_vals[13];
        for (int k = 0; k < n; k++) {
            double val = cos(2.0 * PI * k / n);
            /* Round to 13 decimals for comparison */
            double rounded = round(val * 1e13) / 1e13;
            cos_vals[k] = rounded;
        }

        /* Find unique values */
        for (int k = 0; k < n; k++) {
            int found = 0;
            for (int u = 0; u < num_unique; u++) {
                if (fabs(cos_vals[k] - unique_vals[u]) < 1e-12) {
                    k_to_idx[k] = u;
                    found = 1;
                    break;
                }
            }
            if (!found) {
                k_to_idx[k] = num_unique;
                unique_vals[num_unique] = cos_vals[k];
                num_unique++;
            }
        }

        /* Exact cos values for computation */
        double idx_to_val[13];
        for (int k = 0; k < n; k++)
            idx_to_val[k_to_idx[k]] = cos(2.0 * PI * k / n);

        /* DP: start with empty state */
        dp_clear();
        State init;
        memset(&init, 0, sizeof(init));
        init.k_sum = 0;
        dp_add(&init, 1.0, num_unique, n);

        for (int m = 2; m <= M_LIM; m++) {
            /* Transition: add one variable (ball) */
            int old_count = dp_collect();

            /* Build new DP */
            DPEntry **old_table = dp_table;
            DPEntry *old_pool = dp_pool;
            int old_pool_idx = dp_pool_idx;

            /* Allocate new dp */
            dp_pool_idx = 0;
            memset(dp_table, 0, DP_HASH_SIZE * sizeof(DPEntry *));

            for (int it = 0; it < old_count; it++) {
                State *s = &dp_items[it].state;
                double ways = dp_items[it].ways;

                for (int k = 0; k < n; k++) {
                    State ns = *s;
                    ns.counts[k_to_idx[k]]++;
                    ns.k_sum = (s->k_sum + k) % n;
                    dp_add(&ns, ways, num_unique, n);
                }
            }

            /* Compute F(n, m) */
            int cur_count = dp_collect();
            double f_val = 0.0;
            for (int it = 0; it < cur_count; it++) {
                State *s = &dp_items[it].state;
                double ways = dp_items[it].ways;

                double sum_cos = 0.0;
                for (int i = 0; i < num_unique; i++)
                    sum_cos += s->counts[i] * idx_to_val[i];

                double cos_sum_k = cos(2.0 * PI * s->k_sum / n);
                double Lambda = (sum_cos + cos_sum_k) / m;

                if (fabs(1.0 - Lambda) < 1e-9) continue;

                f_val += ways / (1.0 - Lambda);
            }

            total_G += f_val;
        }
    }

    /* Format: scientific with 12 significant digits after decimal point */
    printf("%.12e\n", total_G);
    return 0;
}

/*
 * Project Euler Problem 701: Random Connected Area.
 *
 * Find the expected maximum area of an orthogonally contiguous group of black
 * cells, if each cell in an N x N square is randomly colored white or black.
 *
 * Uses state-based DP: for each of the previous N cells, track which are
 * connected (profile), the areas of the contiguous regions, and the total
 * maximum area. We canonicalize profiles to reduce the state space.
 *
 * State: profile[N] (canonicalized group IDs), areas[N], max_area
 * We use a hash map of states -> counts.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define NN 7
#define MAX_AREA 50  /* max possible area per group */
/* Hash table size - must be power of 2 */
#define HT_SIZE (1 << 22)
#define HT_MASK (HT_SIZE - 1)

typedef struct {
    unsigned char profile[NN];
    unsigned char areas[NN];
    unsigned char max_area;
} State;

typedef struct Entry {
    State state;
    long long count;
    struct Entry *next;
} Entry;

static Entry *table[HT_SIZE];
static Entry *new_table[HT_SIZE];

/* Pool allocator for entries */
#define POOL_BLOCK 1000000
static Entry *pool = NULL;
static int pool_idx = POOL_BLOCK;

Entry *alloc_entry(void) {
    if (pool_idx >= POOL_BLOCK) {
        pool = (Entry *)malloc(POOL_BLOCK * sizeof(Entry));
        pool_idx = 0;
    }
    return &pool[pool_idx++];
}

unsigned int hash_state(const State *s) {
    unsigned int h = 0;
    for (int i = 0; i < NN; i++) {
        h = h * 31 + s->profile[i];
        h = h * 31 + s->areas[i];
    }
    h = h * 31 + s->max_area;
    return h;
}

int state_eq(const State *a, const State *b) {
    return memcmp(a, b, sizeof(State)) == 0;
}

void ht_add(Entry **ht, const State *s, long long count) {
    unsigned int h = hash_state(s) & HT_MASK;
    Entry *e = ht[h];
    while (e) {
        if (state_eq(&e->state, s)) {
            e->count += count;
            return;
        }
        e = e->next;
    }
    e = alloc_entry();
    e->state = *s;
    e->count = count;
    e->next = ht[h];
    ht[h] = e;
}

void canonicalize(unsigned char *profile, unsigned char *out) {
    unsigned char mapping[NN + 1];
    memset(mapping, 0, sizeof(mapping));
    int idx = 0;
    for (int i = 0; i < NN; i++) {
        int num = profile[i];
        if (num > 0 && mapping[num] == 0) {
            idx++;
            mapping[num] = idx;
        }
        out[i] = mapping[num];
    }
}

int main() {
    memset(table, 0, sizeof(table));

    State init;
    memset(&init, 0, sizeof(init));
    ht_add(table, &init, 1);

    for (int row = 0; row < NN; row++) {
        for (int col = 0; col < NN; col++) {
            memset(new_table, 0, sizeof(new_table));
            /* Reset pool for new iteration */
            pool = NULL;
            pool_idx = POOL_BLOCK;

            for (int h = 0; h < HT_SIZE; h++) {
                Entry *e = table[h];
                while (e) {
                    State *st = &e->state;
                    long long count = e->count;

                    /* Handle white cell */
                    {
                        State ns;
                        unsigned char new_profile[NN], new_areas[NN];
                        for (int i = 0; i < NN - 1; i++) {
                            new_profile[i] = st->profile[i + 1];
                            new_areas[i] = st->areas[i + 1];
                        }
                        new_profile[NN - 1] = 0;
                        new_areas[NN - 1] = 0;
                        canonicalize(new_profile, ns.profile);
                        memcpy(ns.areas, new_areas, NN);
                        ns.max_area = st->max_area;
                        ht_add(new_table, &ns, count);
                    }

                    /* Handle black cell */
                    {
                        State ns;
                        unsigned char new_profile[NN], new_areas[NN];
                        for (int i = 0; i < NN - 1; i++) {
                            new_profile[i] = st->profile[i + 1];
                            new_areas[i] = st->areas[i + 1];
                        }
                        new_profile[NN - 1] = 0;
                        new_areas[NN - 1] = 0;

                        int above_group = st->profile[0];
                        int left_group = (col > 0) ? st->profile[NN - 1] : 0;

                        int new_area = 1 + st->areas[0];
                        if (above_group != left_group && col > 0) {
                            new_area += st->areas[NN - 1];
                        }

                        for (int i = 0; i < NN; i++) {
                            if ((new_profile[i] > 0 &&
                                 (new_profile[i] == above_group ||
                                  (new_profile[i] == left_group && col > 0)))
                                || i == NN - 1) {
                                new_profile[i] = NN;
                                new_areas[i] = new_area;
                            }
                        }

                        canonicalize(new_profile, ns.profile);
                        memcpy(ns.areas, new_areas, NN);
                        ns.max_area = st->max_area;
                        if (new_area > ns.max_area) ns.max_area = new_area;
                        ht_add(new_table, &ns, count);
                    }

                    e = e->next;
                }
            }

            /* Swap tables */
            memcpy(table, new_table, sizeof(table));
        }
    }

    /* Compute answer */
    double ans = 0.0;
    for (int h = 0; h < HT_SIZE; h++) {
        Entry *e = table[h];
        while (e) {
            ans += (double)e->count * e->state.max_area;
            e = e->next;
        }
    }

    /* Divide by 2^(N*N) */
    double total = 1.0;
    for (int i = 0; i < NN * NN; i++) total *= 2.0;
    ans /= total;

    printf("%.8f\n", ans);
    return 0;
}

/*
 * Project Euler Problem 298: Selective Amnesia
 *
 * Simulate Larry and Robin's memory game using state-based DP.
 * State: (larry_memory[5], robin_memory[5], diff) -> probability
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

#define TURNS 50
#define K_VAL 10
#define L_VAL 5
#define MAX_DIFF (TURNS + 1)
#define DIFF_OFFSET TURNS  /* diff ranges from -TURNS to +TURNS */
#define DIFF_SIZE (2 * TURNS + 1)

/* Encode Larry's and Robin's memories as integers.
 * Each memory is a tuple of L_VAL values in [0..K_VAL].
 * We normalize by mapping values to consecutive integers. */

/* State: encoded (larry, robin) -> probability array over diffs */
#define HT_SIZE (1 << 20)
#define HT_MASK (HT_SIZE - 1)

typedef struct {
    unsigned long long key;  /* encoded (larry, robin) */
    double prob[DIFF_SIZE];
    int used;
} HTEntry;

static HTEntry *ht_a, *ht_b;

static unsigned long long encode_mems(int *larry, int *robin) {
    /* Normalize: map values to consecutive integers starting from 0 */
    int mapping[K_VAL + 2];
    memset(mapping, -1, sizeof(mapping));
    mapping[0] = 0;
    int next_id = 1;

    int nl[L_VAL], nr[L_VAL];
    for (int i = 0; i < L_VAL; i++) {
        if (mapping[larry[i]] == -1)
            mapping[larry[i]] = next_id++;
        nl[i] = mapping[larry[i]];
    }
    for (int i = 0; i < L_VAL; i++) {
        if (mapping[robin[i]] == -1)
            mapping[robin[i]] = next_id++;
        nr[i] = mapping[robin[i]];
    }

    /* Encode: each value needs 4 bits (max 11 values) */
    unsigned long long key = 0;
    for (int i = 0; i < L_VAL; i++)
        key = key * 12 + nl[i];
    for (int i = 0; i < L_VAL; i++)
        key = key * 12 + nr[i];
    return key;
}

static void ht_init(HTEntry *ht) {
    for (int i = 0; i < HT_SIZE; i++) ht[i].used = 0;
}

static HTEntry *ht_find(HTEntry *ht, unsigned long long key) {
    unsigned int h = (unsigned int)((key ^ (key >> 30)) * 0xbf58476d1ce4e5b9ULL);
    h = (h ^ (h >> 16)) & HT_MASK;
    while (1) {
        if (!ht[h].used) return &ht[h];
        if (ht[h].key == key) return &ht[h];
        h = (h + 1) & HT_MASK;
    }
}

int main(void) {
    ht_a = calloc(HT_SIZE, sizeof(HTEntry));
    ht_b = calloc(HT_SIZE, sizeof(HTEntry));
    if (!ht_a || !ht_b) return 1;

    ht_init(ht_a);
    ht_init(ht_b);

    /* Initial state: both memories are [0, 0, 0, 0, 0] */
    int init_larry[L_VAL] = {0, 0, 0, 0, 0};
    int init_robin[L_VAL] = {0, 0, 0, 0, 0};
    unsigned long long init_key = encode_mems(init_larry, init_robin);
    HTEntry *e = ht_find(ht_a, init_key);
    e->key = init_key;
    e->used = 1;
    memset(e->prob, 0, sizeof(e->prob));
    e->prob[DIFF_OFFSET] = 1.0;

    HTEntry *cur_ht = ht_a;
    HTEntry *nxt_ht = ht_b;

    for (int turn = 0; turn < TURNS; turn++) {
        ht_init(nxt_ht);

        for (int idx = 0; idx < HT_SIZE; idx++) {
            if (!cur_ht[idx].used) continue;

            /* Decode the state */
            unsigned long long key = cur_ht[idx].key;
            int larry[L_VAL], robin[L_VAL];

            /* Decode from key */
            unsigned long long tmp = key;
            for (int i = L_VAL - 1; i >= 0; i--) {
                robin[i] = (int)(tmp % 12);
                tmp /= 12;
            }
            for (int i = L_VAL - 1; i >= 0; i--) {
                larry[i] = (int)(tmp % 12);
                tmp /= 12;
            }

            for (int called = 1; called <= K_VAL; called++) {
                int diff_change = 0;

                /* Update Larry's memory */
                int new_larry[L_VAL];
                int found_l = -1;
                for (int i = 0; i < L_VAL; i++) {
                    if (larry[i] == called && found_l == -1) found_l = i;
                }
                if (found_l >= 0) {
                    diff_change += 1;
                    /* Remove from position found_l, shift left, append called */
                    int j = 0;
                    for (int i = 0; i < L_VAL; i++) {
                        if (i == found_l) continue;
                        new_larry[j++] = larry[i];
                    }
                    new_larry[L_VAL - 1] = called;
                } else {
                    /* Remove oldest (index 0), shift left, append called */
                    for (int i = 0; i < L_VAL - 1; i++)
                        new_larry[i] = larry[i + 1];
                    new_larry[L_VAL - 1] = called;
                }

                /* Update Robin's memory */
                int new_robin[L_VAL];
                int found_r = -1;
                for (int i = 0; i < L_VAL; i++) {
                    if (robin[i] == called && found_r == -1) found_r = i;
                }
                if (found_r >= 0) {
                    diff_change -= 1;
                    memcpy(new_robin, robin, sizeof(new_robin));
                } else {
                    /* Remove oldest, append called */
                    for (int i = 0; i < L_VAL - 1; i++)
                        new_robin[i] = robin[i + 1];
                    new_robin[L_VAL - 1] = called;
                }

                unsigned long long nk = encode_mems(new_larry, new_robin);
                HTEntry *ne = ht_find(nxt_ht, nk);
                if (!ne->used) {
                    ne->key = nk;
                    ne->used = 1;
                    memset(ne->prob, 0, sizeof(ne->prob));
                }

                for (int d = 0; d < DIFF_SIZE; d++) {
                    if (cur_ht[idx].prob[d] == 0.0) continue;
                    int nd = d + diff_change;
                    if (nd >= 0 && nd < DIFF_SIZE) {
                        ne->prob[nd] += cur_ht[idx].prob[d] / K_VAL;
                    }
                }
            }
        }

        /* Swap */
        HTEntry *tmp_ht = cur_ht;
        cur_ht = nxt_ht;
        nxt_ht = tmp_ht;
    }

    /* Sum expected |diff| */
    double ans = 0.0;
    for (int idx = 0; idx < HT_SIZE; idx++) {
        if (!cur_ht[idx].used) continue;
        for (int d = 0; d < DIFF_SIZE; d++) {
            if (cur_ht[idx].prob[d] != 0.0) {
                ans += fabs((double)(d - DIFF_OFFSET)) * cur_ht[idx].prob[d];
            }
        }
    }

    printf("%.8f\n", ans);

    free(ht_a);
    free(ht_b);
    return 0;
}

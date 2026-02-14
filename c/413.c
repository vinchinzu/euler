#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>

#define MAXD 20
#define BASE 10

/* Hash map for DP states */
#define HM_SIZE (1 << 22)  /* 4M buckets */
#define HM_MASK (HM_SIZE - 1)

typedef struct {
    uint64_t key;
    long long val;
} HMEntry;

typedef struct {
    HMEntry *entries;
    int *next;
    int *bucket;
    int count;
    int cap;
} HashMap;

void hm_init(HashMap *hm, int cap) {
    hm->entries = (HMEntry *)malloc(cap * sizeof(HMEntry));
    hm->next = (int *)malloc(cap * sizeof(int));
    hm->bucket = (int *)malloc(HM_SIZE * sizeof(int));
    memset(hm->bucket, -1, HM_SIZE * sizeof(int));
    hm->count = 0;
    hm->cap = cap;
}

void hm_clear(HashMap *hm) {
    memset(hm->bucket, -1, HM_SIZE * sizeof(int));
    hm->count = 0;
}

void hm_free(HashMap *hm) {
    free(hm->entries);
    free(hm->next);
    free(hm->bucket);
}

void hm_add(HashMap *hm, uint64_t key, long long val) {
    uint32_t h = (uint32_t)(key * 2654435761ULL) & HM_MASK;
    int idx = hm->bucket[h];
    while (idx != -1) {
        if (hm->entries[idx].key == key) {
            hm->entries[idx].val += val;
            return;
        }
        idx = hm->next[idx];
    }
    int i = hm->count++;
    hm->entries[i].key = key;
    hm->entries[i].val = val;
    hm->next[i] = hm->bucket[h];
    hm->bucket[h] = i;
}

static inline uint64_t encode_state(int *counts, int d, int hits) {
    uint64_t key = 0;
    for (int r = 0; r < d; r++) {
        key |= ((uint64_t)(counts[r] > 2 ? 2 : counts[r])) << (2 * r);
    }
    key |= ((uint64_t)hits) << 62;
    return key;
}

static inline void decode_state(uint64_t key, int *counts, int d, int *hits) {
    for (int r = 0; r < d; r++) {
        counts[r] = (key >> (2 * r)) & 3;
    }
    *hits = (key >> 62) & 1;
}

int gcd(int a, int b) {
    while (b) { int t = b; b = a % b; a = t; }
    return a;
}

int main() {
    int N = 19;
    long long total_ans = 0;

    HashMap dp1, dp2;
    hm_init(&dp1, 1 << 21);
    hm_init(&dp2, 1 << 21);

    for (int d = 1; d <= N; d++) {
        int g = gcd(d, BASE);
        int cap = (g == 1) ? 1 : 2;

        hm_clear(&dp1);

        int counts[MAXD];
        memset(counts, 0, sizeof(counts));
        uint64_t init_key = encode_state(counts, d, 0);
        hm_add(&dp1, init_key, 1);

        for (int pos = 0; pos < d; pos++) {
            hm_clear(&dp2);

            for (int ei = 0; ei < dp1.count; ei++) {
                uint64_t key = dp1.entries[ei].key;
                long long val = dp1.entries[ei].val;
                if (val == 0) continue;

                int old_counts[MAXD];
                int old_hits;
                decode_state(key, old_counts, d, &old_hits);

                int start_digit = (pos == 0) ? 1 : 0;
                for (int digit = start_digit; digit < BASE; digit++) {
                    int new_counts[MAXD];
                    memset(new_counts, 0, sizeof(new_counts));

                    for (int r = 0; r < d; r++) {
                        if (old_counts[r] > 0) {
                            int new_r = (r * 10 + digit) % d;
                            new_counts[new_r] += old_counts[r];
                        }
                    }

                    new_counts[digit % d]++;

                    int new_hits = old_hits;
                    new_hits += new_counts[0];

                    if (new_hits > 1) continue;

                    for (int r = 0; r < d; r++) {
                        if (new_counts[r] > cap) new_counts[r] = cap;
                    }

                    uint64_t new_key = encode_state(new_counts, d, new_hits);
                    hm_add(&dp2, new_key, val);
                }
            }

            HashMap tmp = dp1;
            dp1 = dp2;
            dp2 = tmp;
        }

        long long d_ans = 0;
        for (int ei = 0; ei < dp1.count; ei++) {
            uint64_t key = dp1.entries[ei].key;
            int hits = (key >> 62) & 1;
            if (hits == 1) {
                d_ans += dp1.entries[ei].val;
            }
        }
        total_ans += d_ans;
    }

    printf("%lld\n", total_ans);

    hm_free(&dp1);
    hm_free(&dp2);
    return 0;
}

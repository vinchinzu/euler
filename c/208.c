/*
 * Project Euler 208: Robot Walks
 *
 * Robot moves in 1/5 circle arcs (72 degrees), left or right.
 * Count paths of 70 arcs that return to start.
 *
 * State: (left_counts[5], right_counts[5], direction) where counts track
 * how many arcs in each direction slot. Use memoized DP with hash map.
 *
 * Key insight: we only need counts per direction (0..14 each, since 70/5=14).
 * State: 5 left counts + direction. Right counts are determined by the total
 * arcs taken, so we track only left counts and direction.
 *
 * Actually, use the half-and-combine approach from the Python code:
 * simulate first 35 steps, then match with reversed second half.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define K 5
#define N 70
#define HALF (N / 2)
#define GOAL (N / K)  /* 14 */

/* State: left[0..4], right[0..4], direction (0..4) */
/* Encode compactly: each count 0..14 needs 4 bits, 5 counts = 20 bits per side + 3 bits dir */
/* Total: 43 bits - use 64-bit key */

typedef unsigned long long u64;

static u64 encode_key(int left[K], int right[K], int dir) {
    u64 key = 0;
    for (int i = 0; i < K; i++)
        key |= ((u64)left[i]) << (i * 4);
    for (int i = 0; i < K; i++)
        key |= ((u64)right[i]) << (20 + i * 4);
    key |= ((u64)dir) << 40;
    return key;
}

/* Simple hash map */
#define HASH_SIZE (1 << 22)
#define HASH_MASK (HASH_SIZE - 1)

typedef struct Entry {
    u64 key;
    long long val;
    struct Entry *next;
} Entry;

typedef struct {
    Entry *buckets[HASH_SIZE];
    Entry *pool;
    int pool_size;
    int pool_cap;
} HashMap;

static void hm_init(HashMap *hm, int cap) {
    memset(hm->buckets, 0, sizeof(hm->buckets));
    hm->pool = malloc(cap * sizeof(Entry));
    hm->pool_size = 0;
    hm->pool_cap = cap;
}

static void hm_free(HashMap *hm) {
    free(hm->pool);
}

static void hm_clear(HashMap *hm) {
    memset(hm->buckets, 0, sizeof(hm->buckets));
    hm->pool_size = 0;
}

static void hm_add(HashMap *hm, u64 key, long long val) {
    unsigned h = (unsigned)(key ^ (key >> 22)) & HASH_MASK;
    for (Entry *e = hm->buckets[h]; e; e = e->next) {
        if (e->key == key) {
            e->val += val;
            return;
        }
    }
    Entry *e = &hm->pool[hm->pool_size++];
    e->key = key;
    e->val = val;
    e->next = hm->buckets[h];
    hm->buckets[h] = e;
}

static long long hm_get(HashMap *hm, u64 key) {
    unsigned h = (unsigned)(key ^ (key >> 22)) & HASH_MASK;
    for (Entry *e = hm->buckets[h]; e; e = e->next) {
        if (e->key == key) return e->val;
    }
    return 0;
}

/* Iterator */
typedef struct {
    HashMap *hm;
    int bucket;
    Entry *entry;
} HMIter;

static void hm_iter_init(HMIter *it, HashMap *hm) {
    it->hm = hm;
    it->bucket = -1;
    it->entry = NULL;
}

static int hm_iter_next(HMIter *it, u64 *key, long long *val) {
    while (1) {
        if (it->entry) {
            *key = it->entry->key;
            *val = it->entry->val;
            it->entry = it->entry->next;
            return 1;
        }
        it->bucket++;
        if (it->bucket >= HASH_SIZE) return 0;
        it->entry = it->hm->buckets[it->bucket];
    }
}

static void decode_key(u64 key, int left[K], int right[K], int *dir) {
    for (int i = 0; i < K; i++)
        left[i] = (int)((key >> (i * 4)) & 0xF);
    for (int i = 0; i < K; i++)
        right[i] = (int)((key >> (20 + i * 4)) & 0xF);
    *dir = (int)((key >> 40) & 0x7);
}

int main(void) {
    HashMap *cur = malloc(sizeof(HashMap));
    HashMap *nxt = malloc(sizeof(HashMap));
    hm_init(cur, 2000000);
    hm_init(nxt, 2000000);

    int left0[K] = {0}, right0[K] = {0};
    hm_add(cur, encode_key(left0, right0, 0), 1);

    for (int step = 0; step < HALF; step++) {
        hm_clear(nxt);
        HMIter it;
        hm_iter_init(&it, cur);
        u64 key; long long val;
        while (hm_iter_next(&it, &key, &val)) {
            int left[K], right[K], dir;
            decode_key(key, left, right, &dir);

            /* Counterclockwise: left[dir]++, dir = (dir+1)%K */
            left[dir]++;
            int nd1 = (dir + 1) % K;
            hm_add(nxt, encode_key(left, right, nd1), val);
            left[dir]--;

            /* Clockwise: right[dir]++, dir = (dir+K-1)%K */
            right[dir]++;
            int nd2 = (dir + K - 1) % K;
            hm_add(nxt, encode_key(left, right, nd2), val);
            right[dir]--;
        }
        /* Swap cur and nxt */
        HashMap *tmp = cur;
        cur = nxt;
        nxt = tmp;
    }

    /* Now compute answer by matching halves */
    long long ans = 0;
    HMIter it;
    hm_iter_init(&it, cur);
    u64 key; long long val;
    while (hm_iter_next(&it, &key, &val)) {
        int left[K], right[K], dir;
        decode_key(key, left, right, &dir);

        for (int goal = 0; goal <= GOAL; goal++) {
            int rem_left[K], rem_right[K];
            int valid = 1;
            for (int i = 0; i < K; i++) {
                rem_left[i] = goal - left[(i + dir) % K];
                rem_right[i] = GOAL - goal - right[(i + dir) % K];
                if (rem_left[i] < 0 || rem_right[i] < 0) { valid = 0; break; }
            }
            if (!valid) continue;

            int rem_dir = (K - dir) % K;
            u64 rem_key = encode_key(rem_left, rem_right, rem_dir);
            long long match = hm_get(cur, rem_key);
            if (match > 0) {
                ans += val * match;
            }
        }
    }

    printf("%lld\n", ans);

    hm_free(cur); hm_free(nxt);
    free(cur); free(nxt);
    return 0;
}

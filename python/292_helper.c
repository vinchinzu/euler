
#include <stdlib.h>
#include <string.h>

#define HASH_SIZE (1 << 22)
#define HASH_MASK (HASH_SIZE - 1)

typedef struct Entry {
    long long key;
    long long val;
    struct Entry *next;
} Entry;

static Entry *buckets[HASH_SIZE];
static Entry pool[2000000];
static int pool_idx = 0;

static unsigned int hash_key(long long k) {
    unsigned long long u = (unsigned long long)k;
    u = (u ^ (u >> 30)) * 0xbf58476d1ce4e5b9ULL;
    u = (u ^ (u >> 27)) * 0x94d049bb133111ebULL;
    return (unsigned int)(u ^ (u >> 31)) & HASH_MASK;
}

static void hm_clear(void) {
    memset(buckets, 0, sizeof(buckets));
    pool_idx = 0;
}

static void hm_add(long long key, long long val) {
    unsigned int h = hash_key(key);
    Entry *e = buckets[h];
    while (e) {
        if (e->key == key) { e->val += val; return; }
        e = e->next;
    }
    Entry *ne = &pool[pool_idx++];
    ne->key = key;
    ne->val = val;
    ne->next = buckets[h];
    buckets[h] = ne;
}

static long long encode(int sx, int sy, int peri, int ne) {
    return (long long)((sx + 120) * 241 + (sy + 120)) * (121 * 4) + peri * 4 + ne;
}

static void decode(long long key, int *sx, int *sy, int *peri, int *ne) {
    *ne = (int)(key % 4); key /= 4;
    *peri = (int)(key % 121); key /= 121;
    *sy = (int)(key % 241) - 120;
    *sx = (int)(key / 241) - 120;
}

#define MAX_DIRS 200
#define MAX_OPTS 130
static int dir_dx[MAX_DIRS][MAX_OPTS];
static int dir_dy[MAX_DIRS][MAX_OPTS];
static int dir_dl[MAX_DIRS][MAX_OPTS];
static int dir_nopts[MAX_DIRS];

void set_direction(int idx, int opt_idx, int dx, int dy, int dl) {
    dir_dx[idx][opt_idx] = dx;
    dir_dy[idx][opt_idx] = dy;
    dir_dl[idx][opt_idx] = dl;
}

void set_direction_count(int idx, int count) {
    dir_nopts[idx] = count;
}

int compute_half(int start_dir, int end_dir, int N,
                 long long *out_keys, long long *out_vals, int max_out) {
    hm_clear();
    hm_add(encode(0, 0, 0, 0), 1);

    static long long iter_keys[500000];
    static long long iter_vals[500000];

    for (int didx = start_dir; didx < end_dir; didx++) {
        int nopts = dir_nopts[didx];
        if (nopts == 0) continue;

        int nstates = 0;
        for (int b = 0; b < HASH_SIZE; b++) {
            Entry *e = buckets[b];
            while (e) {
                iter_keys[nstates] = e->key;
                iter_vals[nstates] = e->val;
                nstates++;
                e = e->next;
            }
        }

        hm_clear();

        for (int s = 0; s < nstates; s++) {
            long long key = iter_keys[s];
            long long cnt = iter_vals[s];
            int sx, sy, peri, ne;
            decode(key, &sx, &sy, &peri, &ne);

            hm_add(key, cnt);

            for (int o = 0; o < nopts; o++) {
                int new_peri = peri + dir_dl[didx][o];
                if (new_peri <= N) {
                    int new_ne = ne < 3 ? ne + 1 : 3;
                    long long nk = encode(sx + dir_dx[didx][o],
                                          sy + dir_dy[didx][o],
                                          new_peri, new_ne);
                    hm_add(nk, cnt);
                }
            }
        }
    }

    int nstates = 0;
    for (int b = 0; b < HASH_SIZE; b++) {
        Entry *e = buckets[b];
        while (e) {
            if (nstates < max_out) {
                out_keys[nstates] = e->key;
                out_vals[nstates] = e->val;
            }
            nstates++;
            e = e->next;
        }
    }
    return nstates;
}

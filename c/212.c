/*
 * Project Euler 212: Combined Volume of Cuboids
 *
 * 50000 cuboids generated via lagged Fibonacci sequence.
 * Compute the total volume of their union using spatial sectioning
 * and inclusion-exclusion.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define N_CUBOIDS 50000
#define L 130

typedef struct {
    int x, y, z, dx, dy, dz;
} Cuboid;

static int s_seq[300001];  /* lagged fibonacci sequence */
static Cuboid cuboids[N_CUBOIDS];

/* Section hash map */
typedef struct SEntry {
    int sx, sy, sz;
    int *indices;
    int count;
    int cap;
    struct SEntry *next;
} SEntry;

#define SEC_HASH_SIZE (1 << 18)
#define SEC_HASH_MASK (SEC_HASH_SIZE - 1)
static SEntry *sec_buckets[SEC_HASH_SIZE];

static unsigned sec_hash(int sx, int sy, int sz) {
    unsigned h = (unsigned)(sx * 73856093 ^ sy * 19349663 ^ sz * 83492791);
    return h & SEC_HASH_MASK;
}

static SEntry *sec_pool;
static int sec_pool_idx;

static void sec_add(int sx, int sy, int sz, int idx) {
    unsigned h = sec_hash(sx, sy, sz);
    for (SEntry *e = sec_buckets[h]; e; e = e->next) {
        if (e->sx == sx && e->sy == sy && e->sz == sz) {
            if (e->count >= e->cap) {
                e->cap *= 2;
                e->indices = realloc(e->indices, e->cap * sizeof(int));
            }
            e->indices[e->count++] = idx;
            return;
        }
    }
    SEntry *e = &sec_pool[sec_pool_idx++];
    e->sx = sx; e->sy = sy; e->sz = sz;
    e->cap = 8;
    e->indices = malloc(e->cap * sizeof(int));
    e->indices[0] = idx;
    e->count = 1;
    e->next = sec_buckets[h];
    sec_buckets[h] = e;
}

static int iround_down(int n, int k) {
    int r = n % k;
    if (r < 0) r += k;
    return n - r;
}

static inline int max_i(int a, int b) { return a > b ? a : b; }
static inline int min_i(int a, int b) { return a < b ? a : b; }

static long long helper(int *indices, int n_indices, int idx,
                        int min_x, int min_y, int min_z,
                        int max_x, int max_y, int max_z,
                        int num_cuboids) {
    if (min_x >= max_x || min_y >= max_y || min_z >= max_z) return 0;
    if (idx == n_indices) {
        if (num_cuboids == 0) return 0;
        long long vol = (long long)(max_x - min_x) * (max_y - min_y) * (max_z - min_z);
        return (num_cuboids % 2 == 0) ? vol : -vol;
    }

    Cuboid *c = &cuboids[indices[idx]];
    long long result = helper(indices, n_indices, idx + 1,
                             min_x, min_y, min_z, max_x, max_y, max_z, num_cuboids);
    result += helper(indices, n_indices, idx + 1,
                     max_i(min_x, c->x), max_i(min_y, c->y), max_i(min_z, c->z),
                     min_i(max_x, c->x + c->dx), min_i(max_y, c->y + c->dy),
                     min_i(max_z, c->z + c->dz), num_cuboids + 1);
    return result;
}

int main(void) {
    /* Generate lagged Fibonacci sequence */
    for (int k = 1; k <= 55; k++) {
        long long kk = k;
        s_seq[k - 1] = (int)((100003 - 200003 * kk + 300007 * kk * kk * kk) % 1000000);
    }
    for (int k = 55; k < 6 * N_CUBOIDS; k++) {
        s_seq[k] = (s_seq[k - 24] + s_seq[k - 55]) % 1000000;
    }

    for (int i = 0; i < N_CUBOIDS; i++) {
        int idx = 6 * i;
        cuboids[i].x  = s_seq[idx] % 10000;
        cuboids[i].y  = s_seq[idx + 1] % 10000;
        cuboids[i].z  = s_seq[idx + 2] % 10000;
        cuboids[i].dx = s_seq[idx + 3] % 399 + 1;
        cuboids[i].dy = s_seq[idx + 4] % 399 + 1;
        cuboids[i].dz = s_seq[idx + 5] % 399 + 1;
    }

    /* Assign cuboids to sections */
    memset(sec_buckets, 0, sizeof(sec_buckets));
    sec_pool = malloc(2000000 * sizeof(SEntry));
    sec_pool_idx = 0;

    for (int i = 0; i < N_CUBOIDS; i++) {
        Cuboid *c = &cuboids[i];
        for (int dx = 0; dx < c->dx + L; dx += L) {
            for (int dy = 0; dy < c->dy + L; dy += L) {
                for (int dz = 0; dz < c->dz + L; dz += L) {
                    int sx = iround_down(c->x + dx, L);
                    int sy = iround_down(c->y + dy, L);
                    int sz = iround_down(c->z + dz, L);
                    sec_add(sx, sy, sz, i);
                }
            }
        }
    }

    /* Compute volume */
    long long ans = 0;
    for (int b = 0; b < SEC_HASH_SIZE; b++) {
        for (SEntry *e = sec_buckets[b]; e; e = e->next) {
            ans -= helper(e->indices, e->count, 0,
                         e->sx, e->sy, e->sz,
                         e->sx + L, e->sy + L, e->sz + L, 0);
        }
    }

    printf("%lld\n", ans);

    /* Cleanup */
    for (int i = 0; i < sec_pool_idx; i++)
        free(sec_pool[i].indices);
    free(sec_pool);
    return 0;
}

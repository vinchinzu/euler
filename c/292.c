/*
 * Project Euler Problem 292: Pythagorean Polygons
 *
 * Meet-in-the-middle DP over primitive Pythagorean directions.
 * Extracted from embedded C code + Python logic.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

#define PERIM 120

/* Hash map */
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
    return (long long)((sx + PERIM) * 241 + (sy + PERIM)) * (121 * 4) + peri * 4 + ne;
}

/* Directions */
#define MAX_DIRS 200
#define MAX_OPTS 130

static int dir_dx[MAX_DIRS][MAX_OPTS];
static int dir_dy[MAX_DIRS][MAX_OPTS];
static int dir_dl[MAX_DIRS][MAX_OPTS];
static int dir_nopts[MAX_DIRS];
static int num_dirs;

/* Direction generation */
typedef struct { int a, b, c; double angle; } Dir;
static Dir dirs_arr[MAX_DIRS];

static int gcd(int a, int b) {
    if (a < 0) a = -a;
    if (b < 0) b = -b;
    while (b) { int t = b; b = a % b; a = t; }
    return a;
}

static int isqrt(int n) {
    int r = (int)sqrt((double)n);
    while (r * r > n) r--;
    while ((r + 1) * (r + 1) <= n) r++;
    return r;
}

static int cmp_dir(const void *a, const void *b) {
    const Dir *da = (const Dir *)a;
    const Dir *db = (const Dir *)b;
    if (da->angle < db->angle) return -1;
    if (da->angle > db->angle) return 1;
    return 0;
}

static void generate_directions(void) {
    num_dirs = 0;
    for (int a = -PERIM; a <= PERIM; a++) {
        for (int b = -PERIM; b <= PERIM; b++) {
            if (a == 0 && b == 0) continue;
            int c2 = a * a + b * b;
            int c = isqrt(c2);
            if (c * c != c2) continue;
            if (c > PERIM) continue;
            if (gcd(abs(a), abs(b)) != 1) continue;
            dirs_arr[num_dirs].a = a;
            dirs_arr[num_dirs].b = b;
            dirs_arr[num_dirs].c = c;
            dirs_arr[num_dirs].angle = atan2((double)b, (double)a);
            num_dirs++;
        }
    }
    qsort(dirs_arr, num_dirs, sizeof(Dir), cmp_dir);

    for (int i = 0; i < num_dirs; i++) {
        int nopts = 0;
        int a = dirs_arr[i].a, b = dirs_arr[i].b, c = dirs_arr[i].c;
        for (int k = 1; k * c <= PERIM; k++) {
            dir_dx[i][nopts] = k * a;
            dir_dy[i][nopts] = k * b;
            dir_dl[i][nopts] = k * c;
            nopts++;
        }
        dir_nopts[i] = nopts;
    }
}

/* Store extracted states */
static long long state_keys[500000];
static long long state_vals[500000];
static int nstates;

static void extract_states(void) {
    nstates = 0;
    for (int b = 0; b < HASH_SIZE; b++) {
        Entry *e = buckets[b];
        while (e) {
            state_keys[nstates] = e->key;
            state_vals[nstates] = e->val;
            nstates++;
            e = e->next;
        }
    }
}

int main(void) {
    generate_directions();

    int mid = num_dirs / 2;

    /* Compute first half DP */
    hm_clear();
    hm_add(encode(0, 0, 0, 0), 1);

    static long long iter_keys[500000];
    static long long iter_vals[500000];

    for (int didx = 0; didx < mid; didx++) {
        int nopts = dir_nopts[didx];
        if (nopts == 0) continue;

        int ns = 0;
        for (int b = 0; b < HASH_SIZE; b++) {
            Entry *e = buckets[b];
            while (e) {
                iter_keys[ns] = e->key;
                iter_vals[ns] = e->val;
                ns++;
                e = e->next;
            }
        }

        hm_clear();

        for (int s = 0; s < ns; s++) {
            long long key = iter_keys[s];
            long long cnt = iter_vals[s];
            int ne = (int)(key % 4); long long k2 = key / 4;
            int peri = (int)(k2 % 121); long long k3 = k2 / 121;
            int sy = (int)(k3 % 241) - PERIM;
            int sx = (int)(k3 / 241) - PERIM;

            hm_add(key, cnt);

            for (int o = 0; o < nopts; o++) {
                int new_peri = peri + dir_dl[didx][o];
                if (new_peri <= PERIM) {
                    int new_ne = ne < 3 ? ne + 1 : 3;
                    long long nk = encode(sx + dir_dx[didx][o],
                                          sy + dir_dy[didx][o],
                                          new_peri, new_ne);
                    hm_add(nk, cnt);
                }
            }
        }
    }

    extract_states();

    /* Group by (sx, sy, ne) with prefix sums over perimeter */
    /* Use a hash map: key = (sx, sy, ne) -> array of cumulative sums */
    typedef struct { int sx, sy, ne; long long cum[PERIM + 1]; } Group;

    /* Estimate max groups */
    #define MAX_GROUPS 200000
    static Group groups[MAX_GROUPS];
    int ngroups = 0;

    /* Group hash map */
    #define GH_SIZE (1 << 18)
    #define GH_MASK (GH_SIZE - 1)
    static int group_ht[GH_SIZE]; /* -1 = empty */
    memset(group_ht, -1, sizeof(group_ht));

    for (int i = 0; i < nstates; i++) {
        long long key = state_keys[i];
        long long cnt = state_vals[i];
        int ne = (int)(key % 4); long long k2 = key / 4;
        int peri = (int)(k2 % 121); long long k3 = k2 / 121;
        int sy = (int)(k3 % 241) - PERIM;
        int sx = (int)(k3 / 241) - PERIM;

        unsigned int gh = (unsigned int)(((sx + PERIM) * 241 + (sy + PERIM)) * 4 + ne);
        gh = (gh * 2654435761u) & GH_MASK;
        int gidx = -1;
        while (1) {
            if (group_ht[gh] == -1) {
                gidx = ngroups++;
                groups[gidx].sx = sx;
                groups[gidx].sy = sy;
                groups[gidx].ne = ne;
                memset(groups[gidx].cum, 0, sizeof(groups[gidx].cum));
                group_ht[gh] = gidx;
                break;
            }
            int gi = group_ht[gh];
            if (groups[gi].sx == sx && groups[gi].sy == sy && groups[gi].ne == ne) {
                gidx = gi;
                break;
            }
            gh = (gh + 1) & GH_MASK;
        }
        groups[gidx].cum[peri] += cnt;
    }

    /* Build prefix sums */
    for (int g = 0; g < ngroups; g++) {
        for (int p = 1; p <= PERIM; p++) {
            groups[g].cum[p] += groups[g].cum[p - 1];
        }
    }

    /* Combine: for each state, look up matching states */
    long long total = 0;
    for (int i = 0; i < nstates; i++) {
        long long key = state_keys[i];
        long long cnt1 = state_vals[i];
        int ne1 = (int)(key % 4); long long k2 = key / 4;
        int p1 = (int)(k2 % 121); long long k3 = k2 / 121;
        int sy1 = (int)(k3 % 241) - PERIM;
        int sx1 = (int)(k3 / 241) - PERIM;

        int remaining = PERIM - p1;
        if (remaining < 0) continue;

        int min_ne2 = 3 - ne1;
        if (min_ne2 < 0) min_ne2 = 0;

        for (int ne2 = min_ne2; ne2 < 4; ne2++) {
            unsigned int gh = (unsigned int)(((sx1 + PERIM) * 241 + (sy1 + PERIM)) * 4 + ne2);
            gh = (gh * 2654435761u) & GH_MASK;
            while (1) {
                if (group_ht[gh] == -1) break;
                int gi = group_ht[gh];
                if (groups[gi].sx == sx1 && groups[gi].sy == sy1 && groups[gi].ne == ne2) {
                    total += cnt1 * groups[gi].cum[remaining];
                    break;
                }
                gh = (gh + 1) & GH_MASK;
            }
        }
    }

    printf("%lld\n", total);
    return 0;
}

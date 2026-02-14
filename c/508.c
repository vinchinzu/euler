/*
 * Project Euler Problem 508: Integers in Base i-1.
 * Count 1s in base (i-1) representation using recursive rectangle decomposition.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>

typedef long long ll;
#define MOD 1000000007LL
#define L 128

/* Point = complex number x + yi */
typedef struct { ll x, y; } Point;

/* Rectangle */
typedef struct { ll min_x, max_x, min_y, max_y; } Rect;

Point ds[L];
Rect bounds[L + 1];

Point complex_multiply(Point a, Point b) {
    Point r;
    r.x = a.x * b.x - a.y * b.y;
    r.y = a.x * b.y + a.y * b.x;
    return r;
}

Rect rect_translate(Rect r, Point d) {
    Rect res = {r.min_x + d.x, r.max_x + d.x, r.min_y + d.y, r.max_y + d.y};
    return res;
}

Rect rect_union(Rect a, Rect b) {
    Rect r;
    r.min_x = a.min_x < b.min_x ? a.min_x : b.min_x;
    r.max_x = a.max_x > b.max_x ? a.max_x : b.max_x;
    r.min_y = a.min_y < b.min_y ? a.min_y : b.min_y;
    r.max_y = a.max_y > b.max_y ? a.max_y : b.max_y;
    return r;
}

Rect rect_intersection(Rect a, Rect b) {
    Rect r;
    r.min_x = a.min_x > b.min_x ? a.min_x : b.min_x;
    r.max_x = a.max_x < b.max_x ? a.max_x : b.max_x;
    r.min_y = a.min_y > b.min_y ? a.min_y : b.min_y;
    r.max_y = a.max_y < b.max_y ? a.max_y : b.max_y;
    return r;
}

int rect_empty(Rect r) {
    return r.min_x > r.max_x || r.min_y > r.max_y;
}

/* Hash table for memoization */
typedef struct {
    ll min_x, max_x, min_y, max_y;
    int level, count;
    ll value;
    int used;
} HashEntry;

#define HASH_SIZE 4000003
HashEntry *htable;

static inline uint64_t hash_key(ll min_x, ll max_x, ll min_y, ll max_y, int level, int count) {
    uint64_t h = (uint64_t)min_x * 100003 + (uint64_t)max_x * 10007 +
                 (uint64_t)min_y * 1009 + (uint64_t)max_y * 101 +
                 (uint64_t)level * 7 + (uint64_t)count;
    h ^= h >> 16;
    h *= 0x45d9f3b;
    h ^= h >> 16;
    return h % HASH_SIZE;
}

ll hash_get(Rect r, int level, int count, int *found) {
    uint64_t idx = hash_key(r.min_x, r.max_x, r.min_y, r.max_y, level, count);
    for (int i = 0; i < 32; i++) {
        uint64_t pos = (idx + i) % HASH_SIZE;
        if (!htable[pos].used) { *found = 0; return 0; }
        if (htable[pos].min_x == r.min_x && htable[pos].max_x == r.max_x &&
            htable[pos].min_y == r.min_y && htable[pos].max_y == r.max_y &&
            htable[pos].level == level && htable[pos].count == count) {
            *found = 1;
            return htable[pos].value;
        }
    }
    *found = 0;
    return 0;
}

void hash_put(Rect r, int level, int count, ll value) {
    uint64_t idx = hash_key(r.min_x, r.max_x, r.min_y, r.max_y, level, count);
    for (int i = 0; i < 32; i++) {
        uint64_t pos = (idx + i) % HASH_SIZE;
        if (!htable[pos].used) {
            htable[pos].min_x = r.min_x;
            htable[pos].max_x = r.max_x;
            htable[pos].min_y = r.min_y;
            htable[pos].max_y = r.max_y;
            htable[pos].level = level;
            htable[pos].count = count;
            htable[pos].value = value;
            htable[pos].used = 1;
            return;
        }
    }
}

ll B(Rect r, int k, int extra) {
    if (rect_empty(r)) return 0;
    if (k == -1) return extra;

    int found;
    ll cached = hash_get(r, k, extra, &found);
    if (found) return cached;

    Rect bound = bounds[k];
    Rect r1 = rect_intersection(r, bound);
    Point neg_dk = {-ds[k].x, -ds[k].y};
    Rect r2_shifted = rect_translate(r, neg_dk);
    Rect r2 = rect_intersection(r2_shifted, bound);

    ll result = (B(r1, k - 1, extra) + B(r2, k - 1, extra + 1)) % MOD;

    hash_put(r, k, extra, result);
    return result;
}

int main() {
    ll N = 1000000000000000LL; /* 10^15 */

    htable = (HashEntry*)calloc(HASH_SIZE, sizeof(HashEntry));

    /* Compute d_k = (i-1)^k */
    ds[0].x = 1; ds[0].y = 0;
    for (int k = 1; k < L; k++)
        ds[k] = complex_multiply(ds[k-1], (Point){-1, 1});

    /* Compute bounding rectangles */
    bounds[0] = (Rect){0, 0, 0, 0};
    for (int k = 0; k < L; k++)
        bounds[k + 1] = rect_union(bounds[k], rect_translate(bounds[k], ds[k]));

    Rect query = {-N, N, -N, N};
    ll ans = B(query, L - 1, 0);
    printf("%lld\n", ans);

    free(htable);
    return 0;
}

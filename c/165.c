/*
 * Project Euler Problem 165: Intersections
 *
 * Generate 5000 line segments from a pseudo-random sequence,
 * find all true intersection points (not at endpoints), count distinct points.
 * Store points as reduced fractions (num_x/den_x, num_y/den_y).
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define SEGMENT_COUNT 5000
#define VALUES_COUNT (SEGMENT_COUNT * 4)
#define MOD 50515093
#define SEED 290797
#define RANGE 500

typedef long long i64;

static i64 gcd_abs(i64 a, i64 b) {
    if (a < 0) a = -a;
    if (b < 0) b = -b;
    while (b) { i64 t = b; b = a % b; a = t; }
    return a;
}

typedef struct {
    i64 num_x, den_x, num_y, den_y;
} Point;

/* Hash set for points */
#define HASH_SIZE (1 << 23)
#define HASH_MASK (HASH_SIZE - 1)

typedef struct HEntry {
    Point p;
    int used;
} HEntry;

static HEntry *htable;
static int ht_count;

static unsigned int point_hash(Point *p) {
    unsigned long long h = (unsigned long long)p->num_x * 1000003ULL;
    h ^= (unsigned long long)p->den_x * 999983ULL;
    h ^= (unsigned long long)p->num_y * 998989ULL;
    h ^= (unsigned long long)p->den_y * 997003ULL;
    h ^= h >> 17;
    h *= 0xFF51AFD7ED558CCDULL;
    h ^= h >> 33;
    return (unsigned int)(h & HASH_MASK);
}

static int point_eq(Point *a, Point *b) {
    return a->num_x == b->num_x && a->den_x == b->den_x &&
           a->num_y == b->num_y && a->den_y == b->den_y;
}

static int ht_insert(Point *p) {
    unsigned int idx = point_hash(p);
    while (1) {
        if (!htable[idx].used) {
            htable[idx].p = *p;
            htable[idx].used = 1;
            ht_count++;
            return 1;
        }
        if (point_eq(&htable[idx].p, p)) return 0;
        idx = (idx + 1) & HASH_MASK;
    }
}

typedef struct {
    int x1, y1, x2, y2;
    int min_x, max_x, min_y, max_y;
    i64 dx, dy, cross;
} Segment;

static int cmp_seg(const void *a, const void *b) {
    const Segment *sa = (const Segment *)a;
    const Segment *sb = (const Segment *)b;
    return sa->min_x - sb->min_x;
}

int main(void) {
    htable = calloc(HASH_SIZE, sizeof(HEntry));
    ht_count = 0;

    int values[VALUES_COUNT];
    i64 v = SEED;
    for (int i = 0; i < VALUES_COUNT; i++) {
        v = (v * v) % MOD;
        values[i] = (int)(v % RANGE);
    }

    Segment *segs = malloc(SEGMENT_COUNT * sizeof(Segment));
    int count = 0;

    for (int idx = 0; idx < VALUES_COUNT; idx += 4) {
        int x1 = values[idx], y1 = values[idx+1];
        int x2 = values[idx+2], y2 = values[idx+3];
        if (x1 == x2 && y1 == y2) continue;

        Segment *s = &segs[count++];
        s->x1 = x1; s->y1 = y1; s->x2 = x2; s->y2 = y2;
        s->min_x = x1 < x2 ? x1 : x2;
        s->max_x = x1 > x2 ? x1 : x2;
        s->min_y = y1 < y2 ? y1 : y2;
        s->max_y = y1 > y2 ? y1 : y2;
        s->dx = x2 - x1;
        s->dy = y2 - y1;
        s->cross = (i64)x1 * y2 - (i64)y1 * x2;
    }

    qsort(segs, count, sizeof(Segment), cmp_seg);

    for (int i = 0; i < count; i++) {
        Segment *s1 = &segs[i];
        for (int j = i + 1; j < count; j++) {
            Segment *s2 = &segs[j];
            if (s2->min_x > s1->max_x) break;

            if (s1->max_y < s2->min_y || s2->max_y < s1->min_y) continue;

            i64 o1 = s1->dx * (s2->y1 - s1->y1) - s1->dy * (s2->x1 - s1->x1);
            if (o1 == 0) continue;
            i64 o2 = s1->dx * (s2->y2 - s1->y1) - s1->dy * (s2->x2 - s1->x1);
            if (o2 == 0 || (o1 > 0) == (o2 > 0)) continue;

            i64 o3 = s2->dx * (s1->y1 - s2->y1) - s2->dy * (s1->x1 - s2->x1);
            if (o3 == 0) continue;
            i64 o4 = s2->dx * (s1->y2 - s2->y1) - s2->dy * (s1->x2 - s2->x1);
            if (o4 == 0 || (o3 > 0) == (o4 > 0)) continue;

            i64 den = s1->dx * s2->dy - s1->dy * s2->dx;
            if (den == 0) continue;

            i64 num_x = s1->dx * s2->cross - s1->cross * s2->dx;
            i64 num_y = s1->dy * s2->cross - s1->cross * s2->dy;

            if (den < 0) { den = -den; num_x = -num_x; num_y = -num_y; }

            i64 g1 = gcd_abs(num_x, den);
            i64 g2 = gcd_abs(num_y, den);

            Point p;
            p.num_x = num_x / g1;
            p.den_x = den / g1;
            p.num_y = num_y / g2;
            p.den_y = den / g2;

            ht_insert(&p);
        }
    }

    printf("%d\n", ht_count);

    free(segs);
    free(htable);
    return 0;
}

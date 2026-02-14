/*
 * Project Euler Problem 252: Convex Holes
 *
 * Find the maximum area of a convex hole in a set of 500 points generated
 * by Blum Blum Shub. A convex hole is a convex polygon whose interior
 * contains no other points.
 *
 * Algorithm: Dobkin-Edelsbrunner-Overmars "Searching for Empty Convex Polygons"
 * Uses visibility graph + DP.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

#define NPTS 500

typedef struct { int x, y; } Point;

static Point pts[NPTS];

/* Comparison for sorting points (by x, then y) */
static int cmp_points(const void *a, const void *b) {
    const Point *pa = (const Point*)a, *pb = (const Point*)b;
    if (pa->x != pb->x) return pa->x - pb->x;
    return pa->y - pb->y;
}

/* Cross product of (p2-p1) x (p3-p1) */
static long long cross(Point p1, Point p2, Point p3) {
    return (long long)(p2.x - p1.x) * (p3.y - p1.y) -
           (long long)(p2.y - p1.y) * (p3.x - p1.x);
}

/* Shoelace area * 2 (absolute value) */
static long long shoestring(Point p1, Point p2, Point p3) {
    long long v = (long long)p1.x * (p2.y - p3.y) +
                  (long long)p2.x * (p3.y - p1.y) +
                  (long long)p3.x * (p1.y - p2.y);
    return v < 0 ? -v : v;
}

/* Sort indices by angle from pA */
static Point sort_center;
static Point *sort_pts_ptr;

static int cmp_angle(const void *a, const void *b) {
    int ia = *(const int*)a, ib = *(const int*)b;
    double ang_a = atan2(sort_pts_ptr[ia].y - sort_center.y,
                         sort_pts_ptr[ia].x - sort_center.x);
    double ang_b = atan2(sort_pts_ptr[ib].y - sort_center.y,
                         sort_pts_ptr[ib].x - sort_center.x);
    if (ang_a < ang_b) return -1;
    if (ang_a > ang_b) return 1;
    return 0;
}

/* Simple deque using circular buffer */
#define DEQUE_CAP 600
typedef struct {
    int data[DEQUE_CAP];
    int front, back, size;
} Deque;

static void deque_init(Deque *d) { d->front = 0; d->back = 0; d->size = 0; }

static int deque_empty(Deque *d) { return d->size == 0; }

static int deque_front(Deque *d) { return d->data[d->front]; }

static void deque_pop_front(Deque *d) {
    d->front = (d->front + 1) % DEQUE_CAP;
    d->size--;
}

static void deque_push_back(Deque *d, int v) {
    d->data[d->back] = v;
    d->back = (d->back + 1) % DEQUE_CAP;
    d->size++;
}

/*
 * Hash table for (pA_idx, p2_idx, p3_idx) -> area (double).
 * Use a flat array indexed by a hash.
 */
#define HT_SIZE (1 << 22)
#define HT_MASK (HT_SIZE - 1)

typedef struct {
    int key1, key2, key3;
    double area;
    int occupied;
} HTEntry;

static HTEntry *ht;

static void ht_init(void) {
    ht = (HTEntry*)calloc(HT_SIZE, sizeof(HTEntry));
}

static unsigned ht_hash(int a, int b, int c) {
    unsigned h = (unsigned)a * 73856093U ^ (unsigned)b * 19349663U ^ (unsigned)c * 83492791U;
    return h & HT_MASK;
}

static double ht_get(int a, int b, int c, int *found) {
    unsigned idx = ht_hash(a, b, c);
    for (int i = 0; i < 32; i++) {
        unsigned pos = (idx + i) & HT_MASK;
        if (!ht[pos].occupied) { *found = 0; return 0.0; }
        if (ht[pos].key1 == a && ht[pos].key2 == b && ht[pos].key3 == c) {
            *found = 1;
            return ht[pos].area;
        }
    }
    *found = 0;
    return 0.0;
}

static void ht_set(int a, int b, int c, double val) {
    unsigned idx = ht_hash(a, b, c);
    for (int i = 0; i < 32; i++) {
        unsigned pos = (idx + i) & HT_MASK;
        if (!ht[pos].occupied) {
            ht[pos].key1 = a;
            ht[pos].key2 = b;
            ht[pos].key3 = c;
            ht[pos].area = val;
            ht[pos].occupied = 1;
            return;
        }
        if (ht[pos].key1 == a && ht[pos].key2 == b && ht[pos].key3 == c) {
            ht[pos].area = val;
            return;
        }
    }
    /* Table too full - shouldn't happen with proper sizing */
}

/* Visibility graph edges */
#define MAX_EDGES 300000
static int out_edges[NPTS][NPTS]; /* out_edges[i][j] for each vertex */
static int out_count[NPTS];
static int in_edges[NPTS][NPTS];
static int in_count[NPTS];

static Point remaining[NPTS];
static int rem_idx[NPTS]; /* global indices of remaining points */
static int nrem;
static Deque Q[NPTS];

static void proceed(int pi_idx, int pj_idx, Point pA) {
    Point pi = remaining[pi_idx];
    Point pj = remaining[pj_idx];
    while (!deque_empty(&Q[pi_idx]) &&
           cross(remaining[deque_front(&Q[pi_idx])], pi, pj) > 0) {
        int pk_idx = deque_front(&Q[pi_idx]);
        deque_pop_front(&Q[pi_idx]);
        proceed(pk_idx, pj_idx, pA);
    }
    out_edges[pi_idx][out_count[pi_idx]++] = pj_idx;
    in_edges[pj_idx][in_count[pj_idx]++] = pi_idx;
    deque_push_back(&Q[pj_idx], pi_idx);
}

static double best_area = 0.0;

int main(void) {
    /* Generate points using Blum Blum Shub */
    long long x = 290797;
    for (int i = 0; i < NPTS; i++) {
        x = (x * x) % 50515093;
        int xv = (int)(x % 2000) - 1000;
        x = (x * x) % 50515093;
        int yv = (int)(x % 2000) - 1000;
        pts[i].x = xv;
        pts[i].y = yv;
    }

    qsort(pts, NPTS, sizeof(Point), cmp_points);

    ht_init();

    for (int k = 0; k < NPTS; k++) {
        Point pA = pts[k];
        nrem = NPTS - k - 1;
        if (nrem < 2) continue;

        /* Copy remaining points */
        for (int i = 0; i < nrem; i++) {
            remaining[i] = pts[k + 1 + i];
            rem_idx[i] = k + 1 + i;
        }

        /* Sort by angle from pA */
        int *order = (int*)malloc(nrem * sizeof(int));
        for (int i = 0; i < nrem; i++) order[i] = i;
        sort_center = pA;
        sort_pts_ptr = remaining;
        qsort(order, nrem, sizeof(int), cmp_angle);

        /* Reorder remaining according to angle sort */
        Point *tmp_pts = (Point*)malloc(nrem * sizeof(Point));
        int *tmp_idx = (int*)malloc(nrem * sizeof(int));
        for (int i = 0; i < nrem; i++) {
            tmp_pts[i] = remaining[order[i]];
            tmp_idx[i] = rem_idx[order[i]];
        }
        memcpy(remaining, tmp_pts, nrem * sizeof(Point));
        memcpy(rem_idx, tmp_idx, nrem * sizeof(int));
        free(tmp_pts);
        free(tmp_idx);
        free(order);

        /* Build visibility graph */
        for (int i = 0; i < nrem; i++) {
            out_count[i] = 0;
            in_count[i] = 0;
            deque_init(&Q[i]);
        }

        for (int i = 0; i < nrem - 1; i++) {
            proceed(i, i + 1, pA);
        }

        /* DP: for each p2, iterate over outgoing p3 */
        for (int p2_i = 0; p2_i < nrem; p2_i++) {
            int *p1s = in_edges[p2_i];
            int np1s = in_count[p2_i];
            int p1_ptr = 0;
            double max_area_local = 0.0;

            for (int oi = 0; oi < out_count[p2_i]; oi++) {
                int p3_i = out_edges[p2_i][oi];
                Point p2 = remaining[p2_i];
                Point p3 = remaining[p3_i];

                int found;
                ht_get(k, rem_idx[p2_i], rem_idx[p3_i], &found);
                if (!found) {
                    while (p1_ptr < np1s &&
                           cross(remaining[p1s[p1_ptr]], p2, p3) > 0) {
                        int p1_local_i = p1s[p1_ptr++];
                        double area;
                        ht_get(k, rem_idx[p1_local_i], rem_idx[p2_i], &found);
                        area = found ? ht_get(k, rem_idx[p1_local_i], rem_idx[p2_i], &found) : 0.0;
                        if (area > max_area_local) max_area_local = area;
                    }
                    double new_area = max_area_local + shoestring(pA, p2, p3) / 2.0;
                    ht_set(k, rem_idx[p2_i], rem_idx[p3_i], new_area);
                    if (new_area > best_area) best_area = new_area;
                }
            }
        }
    }

    printf("%.1f\n", best_area);
    free(ht);
    return 0;
}

/*
 * Project Euler 460 - An ant on the move
 *
 * Find shortest path from (0,1) to (N,1) using Dijkstra.
 * Velocity between (x0,y0) and (x1,y1) is y0 if y0==y1,
 * else (y1-y0)/(ln(y1)-ln(y0)).
 * Only lattice points near the semicircle are considered.
 */
#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <string.h>

typedef struct {
    int x, y;
} Point;

typedef struct {
    double dist;
    int idx;
} HeapEntry;

static HeapEntry *heap;
static int heap_size;

static void heap_push(double d, int idx) {
    int i = heap_size++;
    heap[i].dist = d;
    heap[i].idx = idx;
    while (i > 0) {
        int p = (i - 1) / 2;
        if (heap[p].dist <= heap[i].dist) break;
        HeapEntry tmp = heap[p]; heap[p] = heap[i]; heap[i] = tmp;
        i = p;
    }
}

static HeapEntry heap_pop(void) {
    HeapEntry top = heap[0];
    heap_size--;
    if (heap_size > 0) {
        heap[0] = heap[heap_size];
        int i = 0;
        while (1) {
            int l = 2*i+1, r = 2*i+2, m = i;
            if (l < heap_size && heap[l].dist < heap[m].dist) m = l;
            if (r < heap_size && heap[r].dist < heap[m].dist) m = r;
            if (m == i) break;
            HeapEntry tmp = heap[i]; heap[i] = heap[m]; heap[m] = tmp;
            i = m;
        }
    }
    return top;
}

/* Hash map for point -> index lookup */
#define HSIZE 2000003

typedef struct {
    int key_x, key_y;
    int val;
    int used;
} HEntry;

static HEntry htable[HSIZE];

static int h_get(int x, int y) {
    unsigned h = ((unsigned)(x * 100003 + y)) % HSIZE;
    for (int i = 0; i < HSIZE; i++) {
        unsigned idx = (h + i) % HSIZE;
        if (!htable[idx].used) return -1;
        if (htable[idx].key_x == x && htable[idx].key_y == y)
            return htable[idx].val;
    }
    return -1;
}

static void h_set(int x, int y, int val) {
    unsigned h = ((unsigned)(x * 100003 + y)) % HSIZE;
    for (int i = 0; i < HSIZE; i++) {
        unsigned idx = (h + i) % HSIZE;
        if (!htable[idx].used) {
            htable[idx].key_x = x;
            htable[idx].key_y = y;
            htable[idx].val = val;
            htable[idx].used = 1;
            return;
        }
    }
}

/* Store points grouped by x */
#define MAX_PTS 500000
static Point pts[MAX_PTS];
static int npts;

/* Points grouped by x coordinate */
#define MAX_X 10001
static int *by_x_y[MAX_X];  /* y values for each x */
static int by_x_cnt[MAX_X];
static int by_x_cap[MAX_X];

static void by_x_add(int x, int y) {
    if (by_x_cnt[x] >= by_x_cap[x]) {
        by_x_cap[x] = by_x_cap[x] ? by_x_cap[x] * 2 : 8;
        by_x_y[x] = realloc(by_x_y[x], by_x_cap[x] * sizeof(int));
    }
    by_x_y[x][by_x_cnt[x]++] = y;
}

int main(void) {
    int d = 10000;

    memset(htable, 0, sizeof(htable));
    memset(by_x_cnt, 0, sizeof(by_x_cnt));
    memset(by_x_cap, 0, sizeof(by_x_cap));
    memset(by_x_y, 0, sizeof(by_x_y));

    /* Generate points near semicircle */
    npts = 0;
    int half = d / 2;
    for (int x = 0; x <= d; x++) {
        for (int y = 1; y <= d; y++) {
            long long dx = x - half;
            long long dist = dx * dx + (long long)y * y;
            long long r_lo = (long long)(half - 1) * (half - 1);
            long long r_hi = (long long)(half + 1) * (half + 1);
            if (r_lo <= dist && dist <= r_hi) {
                h_set(x, y, npts);
                pts[npts].x = x;
                pts[npts].y = y;
                by_x_add(x, y);
                npts++;
            }
        }
    }

    /* Dijkstra */
    double *dists = (double *)malloc(npts * sizeof(double));
    char *visited = (char *)calloc(npts, 1);
    for (int i = 0; i < npts; i++) dists[i] = 1e18;

    int start = h_get(0, 1);
    if (start < 0) { printf("No start point!\n"); return 1; }
    dists[start] = 0.0;

    heap = (HeapEntry *)malloc(npts * 20 * sizeof(HeapEntry));
    heap_size = 0;
    heap_push(0.0, start);

    double result = 1e18;

    while (heap_size > 0) {
        HeapEntry cur = heap_pop();
        int i = cur.idx;
        if (visited[i]) continue;
        visited[i] = 1;

        int px = pts[i].x, py = pts[i].y;

        if (px == d && py == 1) {
            result = cur.dist;
            break;
        }

        for (int dx = 0; dx <= d - px; dx++) {
            int nx = px + dx;
            if (nx > d) break;
            int done = 1;
            for (int ki = 0; ki < by_x_cnt[nx]; ki++) {
                int y = by_x_y[nx][ki];
                int j = h_get(nx, y);
                if (j < 0 || visited[j]) continue;

                double vel;
                if (py == y) {
                    vel = (double)py;
                } else {
                    vel = (double)(y - py) / (log((double)y) - log((double)py));
                }

                double dist_sq = (double)dx * dx + (double)(y - py) * (y - py);
                double edge_dist = sqrt(dist_sq) / vel;
                double new_dist = cur.dist + edge_dist;

                if (new_dist < dists[j]) {
                    dists[j] = new_dist;
                    heap_push(new_dist, j);
                }

                if ((long long)dx * dx + (long long)(y - py) * (y - py) < d)
                    done = 0;
            }
            if (done && dx > 0) break;
        }
    }

    printf("%.9f\n", result);

    free(dists);
    free(visited);
    free(heap);
    for (int x = 0; x <= d; x++) free(by_x_y[x]);
    return 0;
}

/*
 * Project Euler Problem 247: Squares under a hyperbola
 *
 * Repeatedly place the largest square under y=1/x (x>=1) and find
 * the index n of the last square with indices (3,3).
 *
 * Uses a max-heap (priority queue) approach.
 */
#include <stdio.h>
#include <stdlib.h>
#include <math.h>

typedef struct {
    double neg_s;  /* negative side length for min-heap = max-heap */
    int ix, iy;
    double x0, y0;
} Entry;

/* Min-heap on neg_s */
static Entry *heap;
static int heap_size;
static int heap_cap;

static void heap_init(int cap) {
    heap_cap = cap;
    heap_size = 0;
    heap = (Entry*)malloc(cap * sizeof(Entry));
}

static void heap_swap(int a, int b) {
    Entry t = heap[a]; heap[a] = heap[b]; heap[b] = t;
}

static void heap_push(Entry e) {
    if (heap_size >= heap_cap) {
        heap_cap *= 2;
        heap = (Entry*)realloc(heap, heap_cap * sizeof(Entry));
    }
    heap[heap_size] = e;
    int i = heap_size++;
    while (i > 0) {
        int p = (i - 1) / 2;
        if (heap[i].neg_s < heap[p].neg_s) {
            heap_swap(i, p);
            i = p;
        } else break;
    }
}

static Entry heap_pop(void) {
    Entry top = heap[0];
    heap[0] = heap[--heap_size];
    int i = 0;
    while (1) {
        int l = 2*i+1, r = 2*i+2, best = i;
        if (l < heap_size && heap[l].neg_s < heap[best].neg_s) best = l;
        if (r < heap_size && heap[r].neg_s < heap[best].neg_s) best = r;
        if (best != i) { heap_swap(i, best); i = best; }
        else break;
    }
    return top;
}

static Entry make_entry(int ix, int iy, double x0, double y0) {
    double s = (sqrt(x0*x0 + y0*y0 - 2*x0*y0 + 4.0) - (x0 + y0)) / 2.0;
    Entry e = { -s, ix, iy, x0, y0 };
    return e;
}

/* C(n,k) for small values */
static int comb(int n, int k) {
    if (k > n) return 0;
    if (k == 0 || k == n) return 1;
    int r = 1;
    for (int i = 0; i < k; i++) {
        r = r * (n - i) / (i + 1);
    }
    return r;
}

int main(void) {
    int IX = 3, IY = 3;
    int num_at_index = comb(IX + IY, IX);

    heap_init(1000000);
    heap_push(make_entry(0, 0, 1.0, 0.0));

    int ans = 0;

    while (num_at_index > 0) {
        Entry e = heap_pop();
        double s = -e.neg_s;
        int ix = e.ix, iy = e.iy;
        double x0 = e.x0, y0 = e.y0;

        if (ix == IX && iy == IY) {
            num_at_index--;
        }

        heap_push(make_entry(ix + 1, iy, x0 + s, y0));
        heap_push(make_entry(ix, iy + 1, x0, y0 + s));
        ans++;
    }

    printf("%d\n", ans);
    free(heap);
    return 0;
}

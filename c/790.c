/*
 * Project Euler 790 - Clock Grid
 *
 * Coordinate compression + segment tree with lazy shifts.
 * Extracted from embedded C in Python solution.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define NOPS 100000
#define KK 50515093
#define T 12

static long long s[4 * NOPS];
static int q_x1[NOPS], q_x2[NOPS], q_y1[NOPS], q_y2[NOPS];

static int *xs, *ys;
static int nx, ny_intervals;

static long long *hc;
static int *shifts;
static int seg_l;

static int *ys_arr;
static int ys_count;

int bsearch_y(int val) {
    int lo = 0, hi = ys_count - 1;
    while (lo < hi) {
        int mid = (lo + hi) / 2;
        if (ys_arr[mid] < val) lo = mid + 1;
        else hi = mid;
    }
    return lo;
}

void merge(int idx) {
    int left = 2 * idx;
    int right = 2 * idx + 1;
    int sl = shifts[left];
    int sr = shifts[right];
    int base_idx = idx * T;
    int base_l = left * T;
    int base_r = right * T;
    for (int h = 0; h < T; h++) {
        hc[base_idx + h] = hc[base_l + ((h - sl) % T + T) % T]
                         + hc[base_r + ((h - sr) % T + T) % T];
    }
}

void update(int from_idx, int to_idx, int diff, int index, int low, int high) {
    if (from_idx >= high || to_idx <= low) return;
    if (from_idx <= low && to_idx >= high) {
        shifts[index] += diff;
        return;
    }
    int mid = (low + high) >> 1;
    update(from_idx, to_idx, diff, 2 * index, low, mid);
    update(from_idx, to_idx, diff, 2 * index + 1, mid, high);
    merge(index);
}

int cmp_int(const void *a, const void *b) {
    return (*(int *)a - *(int *)b);
}

int unique_sorted(int *arr, int n) {
    if (n == 0) return 0;
    int w = 1;
    for (int i = 1; i < n; i++) {
        if (arr[i] != arr[i - 1])
            arr[w++] = arr[i];
    }
    return w;
}

struct Event {
    int x;
    int yi1, yi2;
    int diff;
};

int cmp_event(const void *a, const void *b) {
    return ((struct Event *)a)->x - ((struct Event *)b)->x;
}

int main() {
    int i, t;

    s[0] = 290797;
    for (i = 1; i < 4 * NOPS; i++) {
        s[i] = s[i - 1] * s[i - 1] % KK;
    }

    int *xs_raw = (int *)malloc(2 * NOPS * sizeof(int) + 2 * sizeof(int));
    int *ys_raw = (int *)malloc(2 * NOPS * sizeof(int) + 2 * sizeof(int));
    int xc = 0, yc = 0;

    xs_raw[xc++] = 0;
    xs_raw[xc++] = KK;
    ys_raw[yc++] = 0;
    ys_raw[yc++] = KK;

    for (t = 0; t < NOPS; t++) {
        int base = 4 * t;
        int a = (int)s[base], b = (int)s[base + 1], c = (int)s[base + 2], d = (int)s[base + 3];
        if (a <= b) { q_x1[t] = a; q_x2[t] = b + 1; }
        else { q_x1[t] = b; q_x2[t] = a + 1; }
        if (c <= d) { q_y1[t] = c; q_y2[t] = d + 1; }
        else { q_y1[t] = d; q_y2[t] = c + 1; }

        xs_raw[xc++] = q_x1[t];
        xs_raw[xc++] = q_x2[t];
        ys_raw[yc++] = q_y1[t];
        ys_raw[yc++] = q_y2[t];
    }

    qsort(xs_raw, xc, sizeof(int), cmp_int);
    xc = unique_sorted(xs_raw, xc);
    qsort(ys_raw, yc, sizeof(int), cmp_int);
    yc = unique_sorted(ys_raw, yc);

    xs = xs_raw;
    nx = xc;
    ys_arr = ys_raw;
    ys_count = yc;
    ny_intervals = yc - 1;

    seg_l = 1;
    while (seg_l < ny_intervals) seg_l *= 2;
    int tree_size = 2 * seg_l;

    hc = (long long *)calloc((long long)tree_size * T, sizeof(long long));
    shifts = (int *)calloc(tree_size, sizeof(int));

    for (i = 0; i < ny_intervals; i++) {
        hc[(long long)(seg_l + i) * T] = ys_arr[i + 1] - ys_arr[i];
    }

    for (i = seg_l - 1; i >= 1; i--) {
        merge(i);
    }

    struct Event *events = (struct Event *)malloc(2 * NOPS * sizeof(struct Event));
    int ne = 0;
    for (t = 0; t < NOPS; t++) {
        int yi1 = bsearch_y(q_y1[t]);
        int yi2 = bsearch_y(q_y2[t]);
        events[ne].x = q_x1[t]; events[ne].yi1 = yi1; events[ne].yi2 = yi2; events[ne].diff = 1; ne++;
        events[ne].x = q_x2[t]; events[ne].yi1 = yi1; events[ne].yi2 = yi2; events[ne].diff = -1; ne++;
    }
    qsort(events, ne, sizeof(struct Event), cmp_event);

    int hval[T];
    for (i = 0; i < T; i++) hval[i] = i;
    hval[0] = T;

    long long ans = 0;
    int prev_x = 0;
    int ev_idx = 0;

    for (int xi = 0; xi < nx; xi++) {
        int x = xs[xi];
        long long dx = (long long)(x - prev_x);
        if (dx > 0) {
            int s1 = shifts[1];
            for (int h = 0; h < T; h++) {
                ans += (long long)hval[h] * hc[T + ((h - s1) % T + T) % T] * dx;
            }
        }

        while (ev_idx < ne && events[ev_idx].x == x) {
            update(events[ev_idx].yi1, events[ev_idx].yi2, events[ev_idx].diff, 1, 0, seg_l);
            ev_idx++;
        }

        prev_x = x;
    }

    printf("%lld\n", ans);

    free(hc);
    free(shifts);
    free(events);
    free(xs_raw);
    free(ys_raw);
    return 0;
}

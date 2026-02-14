/*
 * Project Euler 456 - Triangles containing the origin
 *
 * Generate 2M points using the given PRNG. Count triangles containing origin.
 * Group points by angle ray, sort rays, sliding window to subtract non-containing.
 *
 * Uses exact integer cross products for angle comparison.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef long long ll;

#define NPOINTS 2000000

static int px[NPOINTS], py[NPOINTS];

/* Angle comparison for sorting.
 * We define angle ordering by (quadrant, then cross product).
 * Quadrant: 0 = (+x, +y or y=0 x>0), 1 = (-x, +y or x=0 y>0),
 *           2 = (-x, -y or y=0 x<0), 3 = (+x, -y or x=0 y<0)
 */
static int quadrant(int x, int y) {
    if (y > 0) return (x >= 0) ? 0 : 1;
    if (y < 0) return (x <= 0) ? 2 : 3;
    /* y == 0 */
    return (x > 0) ? 0 : 2;
}

/* Compare angles of points a and b (by index).
 * Returns <0 if a < b, 0 if same angle, >0 if a > b.
 */
static int *sort_px, *sort_py;

static int cmp_angle(const void *a, const void *b) {
    int ia = *(const int *)a;
    int ib = *(const int *)b;
    int qa = quadrant(sort_px[ia], sort_py[ia]);
    int qb = quadrant(sort_px[ib], sort_py[ib]);
    if (qa != qb) return qa - qb;
    /* Same quadrant: use cross product. a x b > 0 means a comes before b (smaller angle). */
    ll cross = (ll)sort_px[ia] * sort_py[ib] - (ll)sort_py[ia] * sort_px[ib];
    if (cross > 0) return -1;
    if (cross < 0) return 1;
    return 0;
}

/* Check if two points have the same angle */
static int same_angle(int i, int j) {
    int qi = quadrant(px[i], py[i]);
    int qj = quadrant(px[j], py[j]);
    if (qi != qj) return 0;
    ll cross = (ll)px[i] * py[j] - (ll)py[i] * px[j];
    return cross == 0;
}

static ll nCr2(ll n) {
    if (n < 2) return 0;
    return n * (n - 1) / 2;
}

static ll nCr3(ll n) {
    if (n < 3) return 0;
    return n * (n - 1) * (n - 2) / 6;
}

int main(void) {
    int N = NPOINTS;

    /* Generate points */
    int n1 = 1, n2 = 1;
    for (int i = 0; i < N; i++) {
        n1 = (n1 * 1248) % 32323;
        n2 = (n2 * 8421) % 30103;
        px[i] = n1 - 16161;
        py[i] = n2 - 15051;
    }

    /* Sort indices by angle */
    int *idx = (int *)malloc(N * sizeof(int));
    for (int i = 0; i < N; i++) idx[i] = i;
    sort_px = px;
    sort_py = py;
    qsort(idx, N, sizeof(int), cmp_angle);

    /* Group into rays */
    /* ray_start[r] = start index in idx[], ray_len[r] = count */
    int max_rays = N;
    int *ray_start = (int *)malloc(max_rays * sizeof(int));
    int *ray_len = (int *)malloc(max_rays * sizeof(int));
    int num_rays = 0;

    int i = 0;
    while (i < N) {
        int j = i + 1;
        while (j < N && same_angle(idx[i], idx[j])) j++;
        ray_start[num_rays] = i;
        ray_len[num_rays] = j - i;
        num_rays++;
        i = j;
    }

    /* Build window: sorted by angle, all points after the first ray come first,
     * then we iterate over rays, removing points that are > pi ahead and adding
     * the current ray's points.
     *
     * But it's simpler to use the two-pointer approach on the sorted array:
     * For each point i, count how many points j > i (in angle order) are
     * within < 180 degrees. A triangle doesn't contain origin if all 3 points
     * are in some half-plane.
     *
     * Actually, let me implement the Python algorithm exactly using rays.
     */

    /* The Python algorithm:
     * 1. Start with all points from rays[1..] in the window (start=0, end=total_non_ray0).
     * 2. For each ray (in angle order):
     *    a. Advance 'start' past points with cross(ray_point, window[start]) > 0
     *       (these are points that are now > 180 degrees away)
     *    b. Subtract C(end-start, 2) * ray_size
     *    c. Advance 'start' past points with cross == 0 (collinear, same direction)
     *    d. Subtract (end-start) * C(ray_size, 2)
     *    e. Subtract C(ray_size, 3)
     *    f. Append ray's points to window (end increases)
     *
     * The window is a virtual array - we never remove, just advance 'start'.
     * We need cross products between the "representative" of each ray and
     * each individual point in the window.
     *
     * Since points in the window are sorted by angle, we can use the ray representative.
     */

    /* Build flat window array: ordered by angle, starting from ray 1 */
    /* All points sorted by angle are in idx[]. */
    /* Window order: rays[1], rays[2], ..., rays[num_rays-1], then rays[0] will be appended */
    int *window = (int *)malloc(2 * N * sizeof(int));
    int wlen = 0;

    /* Add all points from rays[1..] */
    for (int r = 1; r < num_rays; r++) {
        for (int k = ray_start[r]; k < ray_start[r] + ray_len[r]; k++) {
            window[wlen++] = idx[k];
        }
    }

    ll ans = nCr3(N);
    int start = 0;
    int end = wlen;

    for (int r = 0; r < num_rays; r++) {
        int rp = idx[ray_start[r]];  /* representative point of this ray */
        int rpx = px[rp], rpy = py[rp];
        int rlen = ray_len[r];

        /* Advance start past points where cross(ray_rep, window[start]) > 0 */
        /* cross(ray_rep, w) = rpx * py[w] - rpy * px[w] */
        while (start < end) {
            int wi = window[start];
            ll cross = (ll)rpx * py[wi] - (ll)rpy * px[wi];
            if (cross <= 0) break;
            start++;
        }

        /* Subtract triangles with 1 point on this ray and 2 in window */
        ans -= nCr2(end - start) * rlen;

        /* Advance start past collinear points (cross == 0) */
        while (start < end) {
            int wi = window[start];
            ll cross = (ll)rpx * py[wi] - (ll)rpy * px[wi];
            if (cross != 0) break;
            start++;
        }

        /* Subtract triangles with 2 points on this ray and 1 in window */
        ans -= (ll)(end - start) * nCr2(rlen);

        /* Subtract triangles with all 3 points on this ray */
        ans -= nCr3(rlen);

        /* Add this ray's points to window */
        for (int k = ray_start[r]; k < ray_start[r] + ray_len[r]; k++) {
            window[wlen] = idx[k];
            wlen++;
            end++;
        }
    }

    printf("%lld\n", ans);

    free(idx);
    free(ray_start);
    free(ray_len);
    free(window);
    return 0;
}

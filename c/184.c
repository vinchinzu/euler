/* Project Euler 184: Triangles containing the origin. */
#include <stdio.h>
#include <stdlib.h>
#include <math.h>

#define RADIUS 105
#define RADIUS_SQ (RADIUS * RADIUS)
#define PI 3.14159265358979323846
#define TWO_PI (2.0 * PI)
#define EPSILON 1e-12
#define MAX_POINTS 50000
#define MAX_DIRS 50000

static int gcd(int a, int b) {
    if (a < 0) a = -a;
    if (b < 0) b = -b;
    while (b) { int t = b; b = a % b; a = t; }
    return a;
}

typedef struct {
    int dx, dy;
    long long pos_count, neg_count;
} DirEntry;

static double angles[MAX_POINTS];
static double extended_angles[MAX_POINTS * 2];
static DirEntry dirs[MAX_DIRS];
static int n_dirs = 0;

static int find_dir(int dx, int dy) {
    for (int i = 0; i < n_dirs; i++) {
        if (dirs[i].dx == dx && dirs[i].dy == dy) return i;
    }
    dirs[n_dirs].dx = dx;
    dirs[n_dirs].dy = dy;
    dirs[n_dirs].pos_count = 0;
    dirs[n_dirs].neg_count = 0;
    return n_dirs++;
}

static int cmp_double(const void *a, const void *b) {
    double da = *(const double *)a, db = *(const double *)b;
    if (da < db) return -1;
    if (da > db) return 1;
    return 0;
}

int main(void) {
    int count_points = 0;
    int min_coord = -(RADIUS - 1), max_coord = RADIUS - 1;

    for (int x = min_coord; x <= max_coord; x++) {
        for (int y = min_coord; y <= max_coord; y++) {
            if (x == 0 && y == 0) continue;
            if (x * x + y * y >= RADIUS_SQ) continue;

            angles[count_points++] = atan2((double)y, (double)x);

            int g = gcd(abs(x), abs(y));
            int dx = x / g, dy = y / g;

            if (dy > 0 || (dy == 0 && dx > 0)) {
                int idx = find_dir(dx, dy);
                dirs[idx].pos_count++;
            } else {
                int idx = find_dir(-dx, -dy);
                dirs[idx].neg_count++;
            }
        }
    }

    qsort(angles, count_points, sizeof(double), cmp_double);

    for (int i = 0; i < count_points; i++) {
        extended_angles[i] = angles[i];
        extended_angles[i + count_points] = angles[i] + TWO_PI;
    }

    long long bad = 0;
    int j = 0;
    for (int i = 0; i < count_points; i++) {
        if (j < i + 1) j = i + 1;
        while (j < i + count_points && extended_angles[j] - angles[i] < PI - EPSILON) {
            j++;
        }
        long long m = j - i - 1;
        bad += m * (m - 1) / 2;
    }

    long long opposite = 0;
    for (int i = 0; i < n_dirs; i++) {
        long long pos = dirs[i].pos_count, neg = dirs[i].neg_count;
        if (pos == 0 || neg == 0) continue;
        long long total_on_line = pos + neg;
        long long other_points = count_points - total_on_line;

        opposite += pos * neg * other_points;
        opposite += (pos * (pos - 1) / 2) * neg;
        opposite += pos * (neg * (neg - 1) / 2);
    }

    long long total = (long long)count_points * (count_points - 1) * (count_points - 2) / 6;
    long long result = total - bad - opposite;
    printf("%lld\n", result);
    return 0;
}

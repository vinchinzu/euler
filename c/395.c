/*
 * Project Euler Problem 395: Pythagorean Tree
 *
 * Find minimum bounding rectangle area for the infinite Pythagorean tree.
 * Uses iterative refinement of extreme points with high precision (long double).
 */
#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <string.h>

#define MAX_SEGS 100000

typedef struct { long double x, y; } Point;

static Point point_add(Point a, Point b) {
    return (Point){a.x + b.x, a.y + b.y};
}

static Point point_sub(Point a, Point b) {
    return (Point){a.x - b.x, a.y - b.y};
}

static Point complex_mul(Point a, Point b) {
    return (Point){a.x * b.x - a.y * b.y, a.x * b.y + a.y * b.x};
}

static long double point_dist(Point a, Point b) {
    long double dx = a.x - b.x, dy = a.y - b.y;
    return sqrtl(dx * dx + dy * dy);
}

typedef struct { Point start, end; } Segment;

static long double find_extreme(int dir) {
    /* dir: 0=max_x, 1=max_y, 2=min_x(negated), 3=min_y(negated) */
    long double A = 3.0L, B = 4.0L, C = 5.0L;
    long double L_bound = sqrtl(2.0L) / (1.0L - B / C);

    Segment *segs = malloc(MAX_SEGS * sizeof(Segment));
    Segment *new_segs = malloc(MAX_SEGS * sizeof(Segment));
    int nsegs = 1;
    segs[0] = (Segment){{0, 0}, {1, 0}};

    long double extremity = -1e30L;

    for (int iter = 0; iter < 100; iter++) {
        int nnew = 0;

        Point transforms[4] = {
            {-A * B / (C * C), B * B / (C * C)},
            {(B - A) * B / (C * C), (B + A) * B / (C * C)},
            {(B + A) * B / (C * C), (B + A) * A / (C * C)},
            {1.0L + A * B / (C * C), A * A / (C * C)}
        };

        for (int i = 0; i < nsegs && nnew + 2 <= MAX_SEGS; i++) {
            Point diff = point_sub(segs[i].end, segs[i].start);
            Point pts[4];
            for (int j = 0; j < 4; j++) {
                pts[j] = point_add(segs[i].start, complex_mul(diff, transforms[j]));
            }
            new_segs[nnew++] = (Segment){pts[0], pts[1]};
            new_segs[nnew++] = (Segment){pts[2], pts[3]};
        }

        /* Find new extreme */
        long double new_extremity = -1e30L;
        for (int i = 0; i < nnew; i++) {
            long double v1, v2;
            switch (dir) {
                case 0: v1 = new_segs[i].start.x; v2 = new_segs[i].end.x; break;
                case 1: v1 = new_segs[i].start.y; v2 = new_segs[i].end.y; break;
                case 2: v1 = -new_segs[i].start.x; v2 = -new_segs[i].end.x; break;
                case 3: v1 = -new_segs[i].start.y; v2 = -new_segs[i].end.y; break;
                default: v1 = v2 = 0; break;
            }
            if (v1 > new_extremity) new_extremity = v1;
            if (v2 > new_extremity) new_extremity = v2;
        }

        if (fabsl(extremity - new_extremity) < 1e-25L) break;
        extremity = new_extremity;

        /* Prune segments that can't contribute */
        int npruned = 0;
        for (int i = 0; i < nnew; i++) {
            long double seg_len = point_dist(new_segs[i].start, new_segs[i].end);
            long double v1, v2;
            switch (dir) {
                case 0: v1 = new_segs[i].start.x; v2 = new_segs[i].end.x; break;
                case 1: v1 = new_segs[i].start.y; v2 = new_segs[i].end.y; break;
                case 2: v1 = -new_segs[i].start.x; v2 = -new_segs[i].end.x; break;
                case 3: v1 = -new_segs[i].start.y; v2 = -new_segs[i].end.y; break;
                default: v1 = v2 = 0; break;
            }
            long double max_v = (v1 > v2) ? v1 : v2;
            if (max_v > new_extremity - L_bound * seg_len) {
                if (npruned < MAX_SEGS) {
                    segs[npruned++] = new_segs[i];
                }
            }
        }
        nsegs = npruned;
    }

    free(segs);
    free(new_segs);
    return extremity;
}

int main(void) {
    long double max_x = find_extreme(0);
    long double max_y = find_extreme(1);
    long double min_x_neg = find_extreme(2); /* -min_x */
    long double min_y_neg = find_extreme(3); /* -min_y */

    long double width = max_x + min_x_neg;
    long double height = max_y + min_y_neg;
    long double area = width * height;

    /* Format to match expected output */
    printf("%.10Lf\n", area);
    return 0;
}

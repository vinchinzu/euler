/*
 * Project Euler Problem 816: Shortest distance among points.
 *
 * Generate 2,000,000 points using Blum Blum Shub, find closest pair.
 */
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <math.h>

#define NUM_POINTS 2000000

typedef struct {
    uint64_t x, y;
} Point;

int cmp_x(const void *a, const void *b) {
    Point *pa = (Point *)a;
    Point *pb = (Point *)b;
    if (pa->x < pb->x) return -1;
    if (pa->x > pb->x) return 1;
    return 0;
}

int main() {
    Point *points = malloc(NUM_POINTS * sizeof(Point));

    /* Blum Blum Shub sequence */
    uint64_t s = 290797;
    uint64_t m = 50515093;
    for (int i = 0; i < NUM_POINTS; i++) {
        points[i].x = s;
        s = (s * s) % m;
        points[i].y = s;
        s = (s * s) % m;
    }

    /* Sort by x */
    qsort(points, NUM_POINTS, sizeof(Point), cmp_x);

    double ans_sq = 1e36;
    for (int i = 0; i < NUM_POINTS; i++) {
        for (int j = i + 1; j < NUM_POINTS; j++) {
            double dx = (double)points[j].x - (double)points[i].x;
            double dx_sq = dx * dx;
            if (dx_sq >= ans_sq) break;
            double dy = (double)points[j].y - (double)points[i].y;
            double d_sq = dx_sq + dy * dy;
            if (d_sq < ans_sq) ans_sq = d_sq;
        }
    }

    printf("%.9f\n", sqrt(ans_sq));
    free(points);
    return 0;
}

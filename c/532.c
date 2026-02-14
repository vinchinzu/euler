/*
 * Project Euler 532 - Robots on a Sphere
 *
 * n robots on a circle of radius R on a unit sphere, each moving toward
 * the next. Find the smallest n such that each moves >= N=1000, then
 * compute total combined distance.
 *
 * Uses numerical integration (Simpson's rule) and binary search.
 */
#include <stdio.h>
#include <math.h>

#define STEPS 10000

static double line_length(int num_robots, double R) {
    double lam = 2.0 * M_PI / num_robots;
    double sin_lam = sin(lam);
    double cos_lam = cos(lam);
    double one_minus_cos_lam = 1.0 - cos_lam;
    double start = acos(R);
    double end = M_PI / 2.0;
    double h = (end - start) / STEPS;

    /* Simpson's rule */
    double sum = 0.0;
    for (int i = 0; i <= STEPS; i++) {
        double t = start + h * i;
        double sin_t = sin(t);
        double cos_t = cos(t);
        double dlong = sin_lam * cos_t;
        double dlat = sin_t * cos_t * one_minus_cos_lam;
        double val;
        if (fabs(dlat) < 1e-30)
            val = 0.0;
        else
            val = hypot(dlat, dlong) / dlat;

        if (i == 0 || i == STEPS)
            sum += val;
        else if (i % 2 == 1)
            sum += 4.0 * val;
        else
            sum += 2.0 * val;
    }

    return sum * h / 3.0;
}

int main(void) {
    int N_target = 1000;
    double R = 0.999;

    /* Binary search for smallest n such that line_length(n, R) >= N_target */
    int low = 1, high = 1 << 30;
    while (low + 1 < high) {
        int mid = low + (high - low) / 2;
        if (line_length(mid, R) >= N_target)
            high = mid;
        else
            low = mid;
    }

    double ans = line_length(high, R) * high;
    printf("%.2f\n", ans);
    return 0;
}

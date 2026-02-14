/*
 * Project Euler 695 - Random Rectangles
 *
 * Expected median area of three rectangles formed by three random points
 * in the unit square.
 *
 * Closed-form: (24*ln((3+sqrt(5))/4) + 22*sqrt(5) - 41) / 144
 */
#include <stdio.h>
#include <math.h>

int main(void) {
    double s5 = sqrt(5.0);
    double ans = (24.0 * log((3.0 + s5) / 4.0) + 22.0 * s5 - 41.0) / 144.0;
    printf("%.10f\n", ans);
    return 0;
}

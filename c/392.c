/*
 * Project Euler Problem 392: Enmeshed Unit Circle
 *
 * Place N inner gridlines to minimize overlap area with unit circle.
 * Binary search on first grid position; recurrence for the rest.
 */
#include <stdio.h>
#include <math.h>

int main(void) {
    int N = 400;
    int half = N / 2 + 1;

    double lo = 1e-15;
    double hi = 1.0 - 1e-15;

    for (int iter = 0; iter < 200; iter++) {
        double mid = (lo + hi) / 2.0;
        double x_prev2 = 0.0;
        double x_prev1 = mid;

        int ok = 1;
        for (int i = 2; i <= half; i++) {
            double s1 = sqrt(1.0 - x_prev1 * x_prev1);
            double s2 = sqrt(1.0 - x_prev2 * x_prev2);
            double x_new = x_prev1 - (s1 - s2) * s1 / x_prev1;
            if (x_new > 1.0) {
                ok = 0;
                break;
            }
            x_prev2 = x_prev1;
            x_prev1 = x_new;
        }

        if (!ok || x_prev1 > 1.0) {
            hi = mid;
        } else {
            lo = mid;
        }
    }

    /* Final computation */
    double mid = (lo + hi) / 2.0;
    double x_prev2 = 0.0;
    double x_prev1 = mid;
    double area = x_prev1;

    for (int i = 2; i <= half; i++) {
        double s1 = sqrt(1.0 - x_prev1 * x_prev1);
        double s2 = sqrt(1.0 - x_prev2 * x_prev2);
        double x_new = x_prev1 - (s1 - s2) * s1 / x_prev1;
        area += (x_new - x_prev1) * s1;
        x_prev2 = x_prev1;
        x_prev1 = x_new;
    }

    printf("%.10f\n", 4.0 * area);
    return 0;
}

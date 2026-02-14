/*
 * Project Euler Problem 607: Marsh Crossing
 *
 * Find the minimum time to cross from A to B through a marsh with
 * strips of different speeds. Uses Snell's Law + binary search.
 */
#include <stdio.h>
#include <math.h>

#define D 100.0
#define L 50.0
#define K 5
#define NUM_REGIONS 7

int main(void) {
    double SPEEDS[NUM_REGIONS] = {10.0, 9.0, 8.0, 7.0, 6.0, 5.0, 10.0};

    /* Compute border positions (lines through (bx, 0) at angle pi/4) */
    int num_borders = K + 2;  /* K+1 marsh borders + final border at D */
    double border_xs[K + 2];
    for (int i = 0; i <= K; i++) {
        border_xs[i] = D / 2.0 - L / sqrt(2.0) + (L / K) * sqrt(2.0) * i;
    }
    border_xs[K + 1] = D;

    /* Binary search on initial angle */
    double low = 0.0, high = M_PI / 4.0;

    for (int iter = 0; iter < 200; iter++) {
        double alpha = (low + high) / 2.0;
        double a = alpha;
        double px = 0.0, py = 0.0;
        double final_y = 0.0;

        for (int i = 0; i < num_borders; i++) {
            double bx = border_xs[i];

            double A1 = sin(M_PI / 4.0);
            double B1 = -cos(M_PI / 4.0);
            double C1 = A1 * bx;

            double A2 = sin(a);
            double B2 = -cos(a);
            double C2 = A2 * px + B2 * py;

            double denom = A2 * B1 - B2 * A1;
            if (fabs(denom) < 1e-15) break;

            double nx = (C2 * B1 - B2 * C1) / denom;
            double ny = (C2 * A1 - A2 * C1) / (B2 * A1 - A2 * B1);

            px = nx; py = ny;

            if (i < num_borders - 1) {
                double sin_val = SPEEDS[i + 1] * sin(M_PI / 4.0 + a) / SPEEDS[i];
                if (fabs(sin_val) > 1.0) break;
                a = asin(sin_val) - M_PI / 4.0;
            }

            final_y = py;
        }

        if (final_y < 0.0) low = alpha;
        else high = alpha;
    }

    /* Compute the time for the converged angle */
    double alpha = (low + high) / 2.0;
    double a = alpha;
    double px = 0.0, py = 0.0;
    double total_time = 0.0;

    for (int i = 0; i < num_borders; i++) {
        double bx = border_xs[i];

        double A1 = sin(M_PI / 4.0);
        double B1 = -cos(M_PI / 4.0);
        double C1 = A1 * bx;

        double A2 = sin(a);
        double B2 = -cos(a);
        double C2 = A2 * px + B2 * py;

        double denom = A2 * B1 - B2 * A1;
        if (fabs(denom) < 1e-15) break;

        double nx = (C2 * B1 - B2 * C1) / denom;
        double ny = (C2 * A1 - A2 * C1) / (B2 * A1 - A2 * B1);

        double dist = sqrt((nx - px) * (nx - px) + (ny - py) * (ny - py));
        total_time += dist / SPEEDS[i];

        px = nx; py = ny;

        if (i < num_borders - 1) {
            double sin_val = SPEEDS[i + 1] * sin(M_PI / 4.0 + a) / SPEEDS[i];
            if (fabs(sin_val) > 1.0) break;
            a = asin(sin_val) - M_PI / 4.0;
        }
    }

    printf("%.10f\n", total_time);
    return 0;
}

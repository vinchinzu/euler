/* Project Euler 199: Iterative Circle Packing. */
#include <stdio.h>
#include <math.h>

#define ITERATIONS 10

static double circle_area(double k) {
    if (k <= 0.0) return 0.0;
    double r = 1.0 / k;
    return M_PI * r * r;
}

static double recurse_area(double k1, double k2, double k3, int depth) {
    if (depth == 0) return 0.0;

    double sum_val = k1 + k2 + k3;
    double root_term = sqrt(k1 * k2 + k1 * k3 + k2 * k3);
    double k_new = sum_val + 2.0 * root_term;

    double area_new = circle_area(k_new);
    return area_new
        + recurse_area(k_new, k1, k2, depth - 1)
        + recurse_area(k_new, k1, k3, depth - 1)
        + recurse_area(k_new, k2, k3, depth - 1);
}

int main(void) {
    double K_LARGE = -1.0;
    double K_SMALL = 1.0 + 2.0 / sqrt(3.0);
    double AREA_LARGE = M_PI * 1.0 * 1.0;
    double SMALL_AREA = M_PI * (1.0 / K_SMALL) * (1.0 / K_SMALL);

    double peripheral_extra = 3.0 * recurse_area(K_LARGE, K_SMALL, K_SMALL, ITERATIONS);
    double central_extra = recurse_area(K_SMALL, K_SMALL, K_SMALL, ITERATIONS);
    double extra_area = peripheral_extra + central_extra;
    double total_area = 3.0 * SMALL_AREA + extra_area;
    double uncovered_fraction = 1.0 - (total_area / AREA_LARGE);

    printf("%.8f\n", uncovered_fraction);
    return 0;
}

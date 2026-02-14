/*
 * Project Euler Problem 587: L-section area.
 *
 * Find the minimum n such that a ray through the corner with slope 1/n
 * divides the L-section into a region which is less than R of the total area.
 */

#include <stdio.h>
#include <math.h>

double f(int n) {
    /* Compute area of smaller region divided by ray with slope 1/n */
    double y = 1.0 / (n + sqrt(2.0 * n) + 1.0);
    return (1.0 - (n - 1) * y - asin(1.0 - n * y)) / 2.0;
}

int main() {
    double R = 0.001;
    double total_area = 1.0 - M_PI / 4.0;

    int ans = 0;
    while (f(ans) >= R * total_area) {
        ans++;
    }

    printf("%d\n", ans);
    return 0;
}

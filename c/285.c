/*
 * Project Euler 285 - Pythagorean Odds
 *
 * For round k, P(k points) = area_of_intersection / k^2 where the intersection
 * is between [0,1]^2 and the annulus k-0.5 < sqrt((k*a+1)^2+(k*b+1)^2) < k+0.5.
 * Expected score = sum_{k=1}^{N} k * P(k) = sum k * [area(k+0.5) - area(k-0.5)] / k^2.
 */
#include <stdio.h>
#include <math.h>

static double area(double r) {
    return (M_PI / 4.0 - asin(1.0 / r)) * r * r - (sqrt(r * r - 1.0) - 1.0);
}

int main(void) {
    int N = 100000;
    double ans = 0.0;

    for (int k = 1; k <= N; k++) {
        double a = area(k + 0.5);
        if (k > 1)
            a -= area(k - 0.5);
        ans += k * a / ((double)k * k);
    }

    printf("%.5f\n", ans);
    return 0;
}

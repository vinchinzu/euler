/*
 * Project Euler Problem 620: Gears
 *
 * Embedded C extracted from Python wrapper.
 */
#include <stdio.h>
#include <math.h>

int main(void) {
    int N = 500;
    long long ans = 0;
    for (int s = 5; s < N - 9; s++) {
        for (int p = 5; p < N - s; p++) {
            for (int q = p + 1; q <= N - s - p; q++) {
                double a = s + p;
                double b = p + q - 2.0 * M_PI;
                double c = s + q;
                double alpha = acos((a*a + b*b - c*c) / (2.0 * a * b));
                double beta = asin(a * sin(alpha) / c);
                int g = (int)(((s + q) * beta - (s + p) * alpha) / M_PI + s + p);
                ans += g;
            }
        }
    }
    printf("%lld\n", ans);
    return 0;
}

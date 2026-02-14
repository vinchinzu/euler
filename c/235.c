/*
 * Project Euler Problem 235: An Arithmetic Geometric sequence
 *
 * Find r such that s(r) = sum_{k=1}^{5000} (900 - 3k) * r^(k-1) = -600000000000.
 */
#include <stdio.h>
#include <math.h>

static double s(double r) {
    double A = 900.0, D = 3.0, N = 5000.0;
    double rN = pow(r, N);
    if (fabs(r - 1.0) < 1e-10)
        return A * N - D * N * (N + 1) / 2.0;
    double rm1 = r - 1.0;
    return A * (rN - 1.0) / rm1 - D * (N * rN / rm1 - (rN - 1.0) / (rm1 * rm1));
}

int main(void) {
    double T = 600000000000.0;
    double low = 1.0, high = 1.1;

    for (int i = 0; i < 200; i++) {
        double mid = (low + high) / 2.0;
        if (s(mid) > -T)
            low = mid;
        else
            high = mid;
    }

    printf("%.12f\n", low);
    return 0;
}

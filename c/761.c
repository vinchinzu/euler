/*
 * Project Euler 761 - Runner and Swimmer
 *
 * Find critical speed V for runner around N-sided pool.
 * Uses the formula from the referenced paper.
 */
#include <stdio.h>
#include <math.h>

int main(void) {
    int N = 6;
    double t = M_PI / N;
    int K = 0;

    /* Find largest K for which sin(K*t) - (K+N)*tan(t)*cos(K*t) < 0 */
    while (sin(K * t) - (K + N) * tan(t) * cos(K * t) < 0)
        K++;
    K--;

    /* Compute alpha */
    double numerator = 2.0 * sin(K * t);
    double denominator = (K + N) * tan(t);
    double a = (K * t + acos(numerator / denominator - cos(K * t))) / 2.0;

    double ans = 1.0 / cos(a);
    printf("%.8f\n", ans);
    return 0;
}

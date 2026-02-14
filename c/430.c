/*
 * Project Euler 430 - Range flips
 *
 * Expected number of disks facing up after M flips on N disks.
 * P(k) = (1 + (2*p_k - 1)^M) / 2 where p_k = ((k-1)^2 + (N-k)^2) / N^2.
 * Translated from python/430.py.
 */
#include <stdio.h>
#include <math.h>

int main(void) {
    double N = 1e10;
    int M = 4000;

    double ans = 0.0;
    double N2 = N * N;

    for (long long k = 1; k <= (long long)(N / 2); k++) {
        double km1 = (double)(k - 1);
        double nmk = N - (double)k;
        double p_k = (km1 * km1 + nmk * nmk) / N2;
        double term = pow(2.0 * p_k - 1.0, M);
        if (fabs(term) < 1e-15)
            break;
        ans += term;
    }
    ans += N / 2.0;

    printf("%.2f\n", ans);
    return 0;
}

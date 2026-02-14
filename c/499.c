/* Project Euler 499 - St. Petersburg Lottery
 * Translated from python/499.py
 *
 * Binary search for largest root of sum equation,
 * then compute 1 - root^N.
 */
#include <stdio.h>
#include <math.h>

int main() {
    long long N_val = 1000000000LL;  /* 10^9 */
    int K = 15;
    int L = 50;

    /* Binary search for largest root */
    double low = 0.0, high = 1.0;
    for (int iter = 0; iter < 200; iter++) {
        double mid = (low + high) / 2.0;
        double res = 0.0;
        for (int i = 0; i < L; i++) {
            /* mid^((1<<i) - K) / (2 << i) */
            double exponent = (double)((1LL << i) - K);
            res += pow(mid, exponent) / (double)(2LL << i);
        }
        if (res < 1.0)
            high = mid;
        else
            low = mid;
    }

    /* ans = 1 - low^N */
    double ans = 1.0 - pow(low, (double)N_val);
    printf("%.7f\n", ans);
    return 0;
}

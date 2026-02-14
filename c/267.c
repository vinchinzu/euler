/*
 * Project Euler 267: Billionaire
 *
 * Find the probability of becoming a billionaire with 1000 coin flips
 * and optimal betting fraction f.
 *
 * Optimal f for w wins out of 1000: f = (3w/N - 1) / 2
 * Sum C(N,w)/2^N for all w where optimal strategy reaches 10^9.
 */
#include <stdio.h>
#include <math.h>

int main(void) {
    int N = 1000;
    double log_C = log(1e9);

    /* Precompute log(C(N, w)) */
    double log_ncr[1001];
    log_ncr[0] = 0.0;
    for (int i = 1; i <= N; i++)
        log_ncr[i] = log_ncr[i-1] + log((double)(N - i + 1)) - log((double)i);

    double log_2N = N * log(2.0);

    double ans = 0.0;

    /* Find cutoff: scan from N down */
    for (int w = N; w >= 0; w--) {
        double f = (3.0 * w / N - 1.0) / 2.0;
        if (f <= 0.0 || f >= 1.0) continue;
        double log_e = (N - w) * log(1.0 - f) + w * log(1.0 + 2.0 * f);
        if (log_e < log_C) break;
        ans += exp(log_ncr[w] - log_2N);
    }

    printf("%.12f\n", ans);
    return 0;
}

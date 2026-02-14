/*
 * Project Euler 286 - Scoring Probabilities
 *
 * Barbara scores from distance x with probability 1 - x/q.
 * Find q such that P(exactly 20 points in 50 shots) = 0.02.
 * Binary search on q.
 */
#include <stdio.h>

static double prob(double q, int n, int k) {
    /* dp[i][j] = P(j points in first i shots) */
    double dp[51][21];
    for (int i = 0; i <= n; i++)
        for (int j = 0; j <= k; j++)
            dp[i][j] = 0.0;
    dp[0][0] = 1.0;

    for (int i = 1; i <= n; i++) {
        for (int j = 0; j <= k; j++) {
            dp[i][j] = dp[i - 1][j] * (double)i / q;
            if (j > 0)
                dp[i][j] += dp[i - 1][j - 1] * (1.0 - (double)i / q);
        }
    }
    return dp[n][k];
}

int main(void) {
    int N = 50, K = 20;
    double R = 0.02;

    double lo = (double)N, hi = 1e10;

    for (int iter = 0; iter < 200; iter++) {
        double mid = (lo + hi) / 2.0;
        if (prob(mid, N, K) < R)
            hi = mid;
        else
            lo = mid;
    }

    printf("%.10f\n", lo);
    return 0;
}

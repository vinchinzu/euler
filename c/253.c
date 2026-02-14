/*
 * Project Euler Problem 253: Caterpillar
 *
 * Expected value of maximum number of segments when filling N=40 slots
 * one at a time in random order.
 *
 * Uses DP: dp[p][s] = number of orderings of p pieces that have exactly s segments.
 */
#include <stdio.h>
#include <string.h>

#define NVAL 40

int main(void) {
    int N = NVAL;

    /* Compute N! using long double for sufficient precision */
    long double fact_N = 1.0L;
    for (int i = 2; i <= N; i++)
        fact_N *= i;

    long double ans = (long double)N / 2.0L;

    for (int M = 0; M < N / 2; M++) {
        /* dp[p][s] */
        long double dp[N + 1][M + 2];
        memset(dp, 0, sizeof(dp));
        dp[0][0] = 1.0L;

        for (int p = 1; p <= N; p++) {
            for (int s = 1; s <= M; s++) {
                dp[p][s] = (long double)s * dp[p-1][s-1]
                          + (long double)s * dp[p-1][s+1]
                          + 2.0L * (long double)s * dp[p-1][s];
            }
        }

        ans -= dp[N][1] / fact_N;
    }

    printf("%.6Lf\n", ans);
    return 0;
}

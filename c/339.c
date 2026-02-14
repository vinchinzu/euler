/*
 * Project Euler Problem 339 - Peredur fab Efrawg
 * Extracted from embedded C in Python solution.
 *
 * Tridiagonal system approach, O(N) memory.
 */
#include <stdio.h>
#include <stdlib.h>

int main(void) {
    const int N = 10000;
    const int K = 2 * N;

    /* e array, indexed 0..K */
    double *e = (double *)calloc(K + 1, sizeof(double));
    if (!e) return 1;
    e[1] = 1.0;
    e[2] = 1.0;

    double *cp = (double *)malloc(N * sizeof(double));  /* modified c' */
    double *dp = (double *)malloc(N * sizeof(double));  /* modified d' */
    double *x  = (double *)malloc(N * sizeof(double));  /* solution */
    if (!cp || !dp || !x) return 1;

    for (int k = 3; k <= K; k++) {
        int n = (k - 1) / 2;
        double inv_k = 1.0 / k;

        /* i = 0: b[0] = 1.0, c[0] = -inv_k, d[0] = 1.0 - inv_k */
        cp[0] = -inv_k;
        dp[0] = 1.0 - inv_k;

        for (int i = 1; i < n; i++) {
            double ai = (i + 1.0) * inv_k - 1.0;
            double ci = -(i + 1.0) * inv_k;
            double m = 1.0 - ai * cp[i - 1];
            double inv_m = 1.0 / m;
            cp[i] = ci * inv_m;
            dp[i] = (- ai * dp[i - 1]) * inv_m;
        }

        /* Back substitution */
        x[n - 1] = dp[n - 1];
        for (int i = n - 2; i >= 0; i--) {
            x[i] = dp[i] - cp[i] * x[i + 1];
        }

        double xn = x[n - 1];
        int idx = (k / 2) * 2 - 1;
        e[k] = xn * k + (1.0 - xn) * e[idx];
    }

    double ans = (e[K] + e[K - 3]) / 2.0;
    printf("%.6f\n", ans);

    free(e);
    free(cp);
    free(dp);
    free(x);
    return 0;
}

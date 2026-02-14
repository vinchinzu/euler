/*
 * Project Euler 661 - A Long Chess Match
 *
 * Compute sum_{k=3}^{50} E(1/sqrt(k+3), 1/sqrt(k+3) + 1/k^2, 1/k^3)
 * where E is expected number of times player A leads, solved via
 * tridiagonal system (Thomas algorithm).
 */
#include <stdio.h>
#include <stdlib.h>
#include <math.h>

static void tridiagonal_solve(double *a, double *b, double *c, double *d,
                              double *x, int n) {
    double *cp = (double *)malloc(n * sizeof(double));
    double *dp = (double *)malloc(n * sizeof(double));

    cp[0] = c[0] / b[0];
    dp[0] = d[0] / b[0];

    for (int i = 1; i < n; i++) {
        double denom = b[i] - a[i] * cp[i - 1];
        cp[i] = c[i] / denom;
        dp[i] = (d[i] - a[i] * dp[i - 1]) / denom;
    }

    x[n - 1] = dp[n - 1];
    for (int i = n - 2; i >= 0; i--) {
        x[i] = dp[i] - cp[i] * x[i + 1];
    }

    free(cp);
    free(dp);
}

static double E(double pa, double pb, double p) {
    double prev_guess = -1e300;
    int L = 1;
    while (1) {
        int sz = 2 * L + 1;
        double *a = (double *)calloc(sz, sizeof(double));
        double *b = (double *)calloc(sz, sizeof(double));
        double *c = (double *)calloc(sz, sizeof(double));
        double *d = (double *)calloc(sz, sizeof(double));
        double *x = (double *)calloc(sz, sizeof(double));

        for (int diff = -L; diff <= L; diff++) {
            int idx = diff + L;
            a[idx] = -(1 - p) * pb;
            b[idx] = 1 - (1 - p) * (1 - pa - pb);
            c[idx] = -(1 - p) * pa;

            if (diff >= 0) d[idx] += pa;
            if (diff >= 2) d[idx] += pb;
            if (diff >= 1) d[idx] += 1 - pa - pb;
        }

        tridiagonal_solve(a, b, c, d, x, sz);
        double guess = x[L];

        free(a); free(b); free(c); free(d); free(x);

        if (fabs(prev_guess - guess) < 1e-10) {
            return guess;
        }
        prev_guess = guess;
        L *= 2;
    }
}

int main() {
    int N = 50;
    double ans = 0.0;
    for (int k = 3; k <= N; k++) {
        double pa = 1.0 / sqrt(k + 3);
        double pb = pa + 1.0 / ((double)k * k);
        double p = 1.0 / ((double)k * k * k);
        ans += E(pa, pb, p);
    }
    printf("%.4f\n", ans);
    return 0;
}

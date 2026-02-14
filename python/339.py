#!/usr/bin/env python3
"""Project Euler Problem 339 - Peredur fab Efrawg.

Uses tridiagonal system approach with embedded C for speed.
Memory: O(N) instead of O(N^2).
"""

import subprocess
import tempfile
import os

C_CODE = r"""
#include <stdio.h>
#include <stdlib.h>

/*
 * For each k from 3..2*N, solve a tridiagonal system of size n = (k-1)/2.
 *
 * System coefficients for i in [0, n):
 *   a[i] = (i+1.0)/k - 1.0
 *   b[i] = 1.0
 *   c[i] = -(i+1.0)/k
 *   d[0] = 1.0 - 1.0/k, d[1..n-1] = 0.0
 *
 * Thomas algorithm (forward sweep + back substitution).
 * Then e[k] = x[n-1]*k + (1 - x[n-1]) * e[(k/2)*2 - 1].
 *
 * Answer = (e[2*N] + e[2*N-3]) / 2.
 */

int main(void) {
    const int N = 10000;
    const int K = 2 * N;

    /* e array, indexed 0..K */
    double *e = (double *)calloc(K + 1, sizeof(double));
    if (!e) return 1;
    e[1] = 1.0;
    e[2] = 1.0;

    /* Max tridiagonal size is (K-1)/2 = N - 1 (for k=2*N, n = (2*N-1)/2 = N-1 when N is large).
       Actually max n = (K-1)/2 = (2*N-1)/2 = N-1 (integer division for odd K)
       or n = (K-1)/2 = N - 1 for even K=2*N: n = (2*N-1)/2 = N-1.
       So max n ~ N. Allocate N. */
    double *cp = (double *)malloc(N * sizeof(double));  /* modified c' */
    double *dp = (double *)malloc(N * sizeof(double));  /* modified d' */
    double *x  = (double *)malloc(N * sizeof(double));  /* solution */
    if (!cp || !dp || !x) return 1;

    for (int k = 3; k <= K; k++) {
        int n = (k - 1) / 2;
        double inv_k = 1.0 / k;

        /* Forward sweep of Thomas algorithm.
         * a[i] = (i+1)*inv_k - 1.0
         * b[i] = 1.0
         * c[i] = -(i+1)*inv_k
         * d[0] = 1.0 - inv_k, d[i>0] = 0.0
         */

        /* i = 0: b[0] = 1.0, c[0] = -inv_k, d[0] = 1.0 - inv_k */
        cp[0] = -inv_k;          /* c[0] / b[0] */
        dp[0] = 1.0 - inv_k;    /* d[0] / b[0] */

        for (int i = 1; i < n; i++) {
            double ai = (i + 1.0) * inv_k - 1.0;
            double ci = -(i + 1.0) * inv_k;
            /* bi = 1.0 */
            double m = 1.0 - ai * cp[i - 1];
            double inv_m = 1.0 / m;
            cp[i] = ci * inv_m;
            dp[i] = (/* d[i]=0 */ - ai * dp[i - 1]) * inv_m;
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
"""


def solve(N=10000):
    with tempfile.NamedTemporaryFile(suffix='.c', mode='w', delete=False) as f:
        f.write(C_CODE)
        c_path = f.name
    bin_path = c_path.replace('.c', '')
    try:
        subprocess.run(['gcc', '-O2', '-o', bin_path, c_path, '-lm'], check=True)
        result = subprocess.run([bin_path], capture_output=True, text=True, timeout=280)
        return result.stdout.strip()
    finally:
        if os.path.exists(c_path):
            os.unlink(c_path)
        if os.path.exists(bin_path):
            os.unlink(bin_path)


if __name__ == "__main__":
    print(solve())

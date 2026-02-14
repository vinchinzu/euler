/*
 * Project Euler Problem 589: Pooh-sticks Marathon.
 *
 * Two sticks dropped simultaneously. Transit time uniform integer in [n, m].
 * Retrieval takes K=5 seconds. Game ends when one stick is more than one full
 * lap ahead. Find S(100) = sum over m=2..100, n=1..m-1 of E(m,n).
 *
 * Extracted from embedded C in Python solution.
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

// Solve Ax = b using Gaussian elimination with partial pivoting
void solve_linear(double *A, double *b, double *x, int n) {
    double *aug = (double *)malloc(n * (n + 1) * sizeof(double));
    for (int i = 0; i < n; i++) {
        for (int j = 0; j < n; j++)
            aug[i * (n + 1) + j] = A[i * n + j];
        aug[i * (n + 1) + n] = b[i];
    }

    // Forward elimination
    for (int col = 0; col < n; col++) {
        int max_row = col;
        double max_val = fabs(aug[col * (n + 1) + col]);
        for (int row = col + 1; row < n; row++) {
            double v = fabs(aug[row * (n + 1) + col]);
            if (v > max_val) {
                max_val = v;
                max_row = row;
            }
        }
        if (max_row != col) {
            for (int j = col; j <= n; j++) {
                double tmp = aug[col * (n + 1) + j];
                aug[col * (n + 1) + j] = aug[max_row * (n + 1) + j];
                aug[max_row * (n + 1) + j] = tmp;
            }
        }
        double pivot = aug[col * (n + 1) + col];
        for (int row = col + 1; row < n; row++) {
            double factor = aug[row * (n + 1) + col] / pivot;
            for (int j = col; j <= n; j++) {
                aug[row * (n + 1) + j] -= factor * aug[col * (n + 1) + j];
            }
        }
    }

    // Back substitution
    for (int i = n - 1; i >= 0; i--) {
        x[i] = aug[i * (n + 1) + n];
        for (int j = i + 1; j < n; j++) {
            x[i] -= aug[i * (n + 1) + j] * x[j];
        }
        x[i] /= aug[i * (n + 1) + i];
    }

    free(aug);
}

double compute_E(int m, int n, int K) {
    int span = m - n + 1;
    int size = m + K + 1;
    double mult = 1.0 / span;

    double *M = (double *)calloc(size * size, sizeof(double));
    double *T = (double *)calloc(size, sizeof(double));
    double *x = (double *)malloc(size * sizeof(double));

    for (int d = 0; d < size; d++) {
        M[d * size + d] = 1.0;
        for (int t1 = n; t1 <= m; t1++) {
            if (t1 < d - K) {
                T[d] += t1 * mult;
            } else {
                for (int t2 = d + n; t2 <= d + m; t2++) {
                    int abs_diff = abs(t1 - t2);
                    if (abs_diff < size) {
                        M[d * size + abs_diff] -= mult * mult;
                    }
                    int min_val = (t1 < t2) ? t1 : t2;
                    T[d] += (min_val + K) * mult * mult;
                }
            }
        }
    }

    solve_linear(M, T, x, size);
    double result = x[0];

    free(M);
    free(T);
    free(x);
    return result;
}

int main() {
    int N = 100;
    int K = 5;
    double ans = 0.0;

    for (int m = 2; m <= N; m++) {
        for (int n = 1; n < m; n++) {
            ans += compute_E(m, n, K);
        }
    }

    printf("%.2f\n", ans);
    return 0;
}

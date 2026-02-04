"""Project Euler Problem 589: Pooh-sticks Marathon.

Two sticks dropped simultaneously. Transit time uniform integer in [n, m].
Retrieval takes K=5 seconds. Game ends when one stick is more than one full
lap ahead. Find S(100) = sum over m=2..100, n=1..m-1 of E(m,n).

Uses linear system to solve for expected time from each delay state.
Optimized matrix construction via C.
"""

import subprocess
import tempfile
import os


def solve():
    c_code = r"""
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

// Solve Ax = b using Gaussian elimination with partial pivoting
void solve_linear(double *A, double *b, double *x, int n) {
    // A is n x n stored row-major, b is length n, x is output length n
    // Work in-place on augmented matrix

    // Copy A and b into augmented matrix
    double *aug = (double *)malloc(n * (n + 1) * sizeof(double));
    for (int i = 0; i < n; i++) {
        for (int j = 0; j < n; j++)
            aug[i * (n + 1) + j] = A[i * n + j];
        aug[i * (n + 1) + n] = b[i];
    }

    // Forward elimination
    for (int col = 0; col < n; col++) {
        // Find pivot
        int max_row = col;
        double max_val = fabs(aug[col * (n + 1) + col]);
        for (int row = col + 1; row < n; row++) {
            double v = fabs(aug[row * (n + 1) + col]);
            if (v > max_val) {
                max_val = v;
                max_row = row;
            }
        }
        // Swap rows
        if (max_row != col) {
            for (int j = col; j <= n; j++) {
                double tmp = aug[col * (n + 1) + j];
                aug[col * (n + 1) + j] = aug[max_row * (n + 1) + j];
                aug[max_row * (n + 1) + j] = tmp;
            }
        }
        // Eliminate
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
"""

    with tempfile.NamedTemporaryFile(mode='w', suffix='.c', delete=False) as f:
        f.write(c_code)
        c_path = f.name

    bin_path = c_path.replace('.c', '')
    try:
        subprocess.run(['gcc', '-O2', '-o', bin_path, c_path, '-lm'],
                      check=True, capture_output=True)
        result = subprocess.run([bin_path], capture_output=True, text=True, check=True,
                              timeout=30)
        return result.stdout.strip()
    finally:
        os.unlink(c_path)
        if os.path.exists(bin_path):
            os.unlink(bin_path)


def main():
    result = solve()
    print(result)


if __name__ == "__main__":
    main()

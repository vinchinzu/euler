/*
 * Project Euler 825: Chasing Game
 *
 * Compute sum_{n=2}^N S(n) where S(n) is the difference in winning
 * probabilities. Small n computed via linear system, large n via
 * harmonic series approximation.
 */
#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <string.h>

static void solve_linear_system(int n, double **A, double *B, double *X) {
    /* Create augmented matrix */
    double **aug = (double **)malloc(n * sizeof(double *));
    for (int i = 0; i < n; i++) {
        aug[i] = (double *)malloc((n + 1) * sizeof(double));
        for (int j = 0; j < n; j++) aug[i][j] = A[i][j];
        aug[i][n] = B[i];
    }

    /* Forward elimination with partial pivoting */
    for (int i = 0; i < n; i++) {
        int max_row = i;
        for (int k = i + 1; k < n; k++) {
            if (fabs(aug[k][i]) > fabs(aug[max_row][i]))
                max_row = k;
        }
        double *tmp = aug[i];
        aug[i] = aug[max_row];
        aug[max_row] = tmp;

        for (int k = i + 1; k < n; k++) {
            double factor = aug[k][i] / aug[i][i];
            for (int j = i; j <= n; j++) {
                aug[k][j] -= factor * aug[i][j];
            }
        }
    }

    /* Back substitution */
    for (int i = n - 1; i >= 0; i--) {
        X[i] = aug[i][n];
        for (int j = i + 1; j < n; j++) {
            X[i] -= aug[i][j] * X[j];
        }
        X[i] /= aug[i][i];
    }

    for (int i = 0; i < n; i++) free(aug[i]);
    free(aug);
}

static double harmonic_approx(double n) {
    double gamma = 0.5772156649015329;
    return gamma + log(n) + 1.0 / (2.0 * n) - 1.0 / (12.0 * n * n);
}

static double S_val(int n) {
    int dim = 2 * n;
    double **A = (double **)malloc(dim * sizeof(double *));
    double *B = (double *)malloc(dim * sizeof(double));
    double *X = (double *)malloc(dim * sizeof(double));

    for (int i = 0; i < dim; i++) {
        A[i] = (double *)calloc(dim, sizeof(double));
        B[i] = 1.0;
        A[i][i] = 1.0;
        for (int j = 1; j <= 3; j++) {
            if (j < i) {
                A[i][dim - i + j] += 1.0 / 3.0;
            }
        }
    }

    solve_linear_system(dim, A, B, X);
    double result = 2.0 * X[n] - 1.0;

    for (int i = 0; i < dim; i++) free(A[i]);
    free(A);
    free(B);
    free(X);
    return result;
}

int main(void) {
    long long NN = 100000000000000LL; /* 10^14 */
    int L = 100;

    double ans = 0.0;
    for (int n = 2; n <= L; n++) {
        ans += S_val(n);
    }

    /* Add harmonic series approximation for large n */
    double sqrt3 = sqrt(3.0);
    double offset = 1.0 / (3.0 - sqrt3);
    ans += harmonic_approx((double)NN) - harmonic_approx((double)L - offset);

    printf("%.8f\n", ans);
    return 0;
}

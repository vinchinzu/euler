/* Project Euler 959 - Frog on number line
 * f(89, 97) = rate of new sites visited
 * Uses random walk return probability via linear system.
 * After p=a+b steps, position change is binomial on [-a..b].
 * Solve (I - T) h = e_0 type system for return probabilities.
 * Answer: 0.857162085
 */
#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <string.h>

/* Compute C(n,k) / 2^n as double */
double binom_prob(int n, int k) {
    if (k < 0 || k > n) return 0.0;
    /* log(C(n,k)) = lgamma(n+1) - lgamma(k+1) - lgamma(n-k+1) */
    double lp = lgamma(n+1) - lgamma(k+1) - lgamma(n-k+1) - n * log(2.0);
    return exp(lp);
}

#define RANGE 1000
#define N (2*RANGE) /* states: -1000..-1, 1..1000 */

/* state index: states[i] for i in 0..N-1 */
/* states = [-1000, -999, ..., -1, 1, 2, ..., 1000] */
static int states[N];
static double A_mat[N][N]; /* Too large for stack on some systems */
static double b_vec[N];

int main(void) {
    int a = 89, b = 97;
    int p = a + b; /* 186 */

    /* Build state list */
    int idx = 0;
    for (int x = -RANGE; x < 0; x++) states[idx++] = x;
    for (int x = 1; x <= RANGE; x++) states[idx++] = x;

    /* Build lookup from state value to index */
    /* state x -> index: if x < 0, idx = x + RANGE; if x > 0, idx = RANGE + x - 1 */
    #define STATE_IDX(x) ((x) < 0 ? (x) + RANGE : RANGE + (x) - 1)

    /* Compute transition probabilities */
    /* z ranges from -a to b (step of 1). prob[z] = C(p, z+a) / 2^p */
    int z_min = -a, z_max = b;
    int z_count = z_max - z_min + 1;
    double *prob = (double *)calloc(z_count, sizeof(double));
    for (int z = z_min; z <= z_max; z++) {
        int k = z + a;
        prob[z - z_min] = binom_prob(p, k);
    }

    /* Initialize A = I, b = 0 */
    memset(A_mat, 0, sizeof(A_mat));
    memset(b_vec, 0, sizeof(b_vec));
    for (int i = 0; i < N; i++) A_mat[i][i] = 1.0;

    /* Build system: h(x) = sum_z prob(z) * h(x+z) for x != 0, h(0) = 1 */
    /* (I - T) h = e_0 where T is transition restricted to non-zero states */
    /* A[i][j] = delta_{ij} - prob(states[j] - states[i]) */
    /* b[i] = prob(0 - states[i]) = prob(-states[i]) */
    for (int i = 0; i < N; i++) {
        int x = states[i];
        for (int zi = 0; zi < z_count; zi++) {
            int z = z_min + zi;
            double pp = prob[zi];
            if (pp == 0.0) continue;
            int next_x = x + z;
            if (next_x == 0) {
                b_vec[i] += pp;
            } else if (next_x >= -RANGE && next_x <= RANGE && next_x != 0) {
                int j = STATE_IDX(next_x);
                A_mat[i][j] -= pp;
            }
            /* else next_x out of range: h(next_x) = 0 (absorbing boundary) */
        }
    }

    /* Solve A * h = b using Gaussian elimination with partial pivoting */
    /* A is N x N where N = 2000. O(N^3) ~ 8e9 ops. This might be slow. */
    /* Actually N=2000, N^3 = 8e9. With doubles, about 30-60 seconds. */
    /* Let me use a banded structure instead. The matrix is sparse! */
    /* Only entries within z_min..z_max of the diagonal are nonzero. */
    /* Bandwidth = p = 186. So banded elimination is O(N * p^2) ~ 7e7. */

    /* Banded LU: for each row k, pivot and eliminate rows k+1..min(k+p, N-1) */
    int bw = p; /* bandwidth */
    for (int k = 0; k < N; k++) {
        /* Partial pivoting within band */
        int max_row = k;
        double max_val = fabs(A_mat[k][k]);
        int search_end = k + bw < N ? k + bw : N - 1;
        for (int r = k + 1; r <= search_end; r++) {
            if (fabs(A_mat[r][k]) > max_val) {
                max_val = fabs(A_mat[r][k]);
                max_row = r;
            }
        }
        if (max_row != k) {
            /* Swap rows k and max_row */
            for (int j = k; j < N && j <= k + 2*bw; j++) {
                double tmp = A_mat[k][j]; A_mat[k][j] = A_mat[max_row][j]; A_mat[max_row][j] = tmp;
            }
            double tmp = b_vec[k]; b_vec[k] = b_vec[max_row]; b_vec[max_row] = tmp;
        }
        /* Eliminate */
        double pivot = A_mat[k][k];
        for (int r = k + 1; r <= search_end; r++) {
            double factor = A_mat[r][k] / pivot;
            if (factor == 0.0) continue;
            A_mat[r][k] = 0.0;
            int col_end = k + 2*bw < N ? k + 2*bw : N - 1;
            for (int j = k + 1; j <= col_end; j++) {
                A_mat[r][j] -= factor * A_mat[k][j];
            }
            b_vec[r] -= factor * b_vec[k];
        }
    }

    /* Back substitution */
    double *h = (double *)malloc(N * sizeof(double));
    for (int i = N - 1; i >= 0; i--) {
        double s = b_vec[i];
        int col_end = i + 2*bw < N ? i + 2*bw : N - 1;
        for (int j = i + 1; j <= col_end; j++) {
            s -= A_mat[i][j] * h[j];
        }
        h[i] = s / A_mat[i][i];
    }

    /* Compute f = sum_z prob(z) * (1 - h(z)) for z != 0 */
    double f = 0.0;
    for (int zi = 0; zi < z_count; zi++) {
        int z = z_min + zi;
        if (z == 0) continue;
        double pp = prob[zi];
        if (pp == 0.0) continue;
        if (z >= -RANGE && z <= RANGE) {
            int j = STATE_IDX(z);
            f += pp * (1.0 - h[j]);
        } else {
            f += pp; /* h(z) = 0 for out-of-range */
        }
    }

    printf("%.9f\n", f);

    free(h);
    free(prob);
    return 0;
}

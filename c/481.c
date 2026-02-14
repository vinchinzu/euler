/* Project Euler 481 - Chef Showdown
 * Extracted from embedded C in python/481.py
 * Bitmask DP with linear system solving.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

#define MAXN 14
#define MAXMASK (1 << MAXN)

double S[MAXN];
double W[MAXMASK][MAXN][MAXN];
double E[MAXMASK][MAXN];

int popcount(int x) {
    int c = 0;
    while (x) { c += x & 1; x >>= 1; }
    return c;
}

/* Get the k-th set bit in mask */
int kth_bit(int mask, int k) {
    int count = 0;
    for (int i = 0; i < MAXN; i++) {
        if (mask & (1 << i)) {
            if (count == k) return i;
            count++;
        }
    }
    return -1;
}

/* Solve Ax = b, n x n system, returns x in b */
void linear_solve(int n, double A[MAXN][MAXN], double b[MAXN]) {
    /* Gaussian elimination with partial pivoting */
    for (int col = 0; col < n; col++) {
        /* Find pivot */
        int pivot = col;
        for (int row = col + 1; row < n; row++)
            if (fabs(A[row][col]) > fabs(A[pivot][col]))
                pivot = row;
        /* Swap rows */
        for (int j = 0; j < n; j++) {
            double t = A[col][j]; A[col][j] = A[pivot][j]; A[pivot][j] = t;
        }
        { double t = b[col]; b[col] = b[pivot]; b[pivot] = t; }
        /* Eliminate */
        for (int row = col + 1; row < n; row++) {
            double factor = A[row][col] / A[col][col];
            for (int j = col; j < n; j++)
                A[row][j] -= factor * A[col][j];
            b[row] -= factor * b[col];
        }
    }
    /* Back substitution */
    for (int row = n - 1; row >= 0; row--) {
        for (int j = row + 1; j < n; j++)
            b[row] -= A[row][j] * b[j];
        b[row] /= A[row][row];
    }
}

long long fibonacci(int n) {
    if (n <= 0) return 0;
    if (n == 1) return 1;
    long long a = 0, b = 1;
    for (int i = 2; i <= n; i++) {
        long long c = a + b;
        a = b; b = c;
    }
    return b;
}

int main() {
    int N = 14;

    /* Compute S[i] = fib(i+1) / fib(N+1) */
    long long fibN1 = fibonacci(N + 1);
    for (int i = 0; i < N; i++)
        S[i] = (double)fibonacci(i + 1) / (double)fibN1;

    memset(W, 0, sizeof(W));
    memset(E, 0, sizeof(E));

    for (int subset = 1; subset < (1 << N); subset++) {
        int n = popcount(subset);
        int chefs[MAXN];
        int ci = 0;
        for (int i = 0; i < N; i++)
            if (subset & (1 << i))
                chefs[ci++] = i;

        if (n == 1) {
            W[subset][chefs[0]][chefs[0]] = 1.0;
            continue;
        }

        /* For each starting position, find the best move */
        double Bs[MAXN][MAXN]; /* Bs[i][start] */
        double B[MAXN]; /* B[start] for E */
        memset(Bs, 0, sizeof(Bs));
        memset(B, 0, sizeof(B));

        for (int start = 0; start < n; start++) {
            double bestProb = -1e18;
            double allProbs[MAXN];
            double expected = 0;
            memset(allProbs, 0, sizeof(allProbs));

            for (int e = 1; e < n; e++) {
                int j = (start + e) % n;
                int next = (start + (e == 1 ? 2 : 1)) % n;
                int new_subset = subset & ~(1 << chefs[j]);

                double prob = W[new_subset][chefs[next]][chefs[start]];
                if (prob > bestProb) {
                    bestProb = prob;
                    for (int i = 0; i < n; i++)
                        allProbs[i] = W[new_subset][chefs[next]][chefs[i]];
                    expected = E[new_subset][chefs[next]];
                }
            }

            for (int i = 0; i < n; i++)
                Bs[i][start] = S[chefs[start]] * allProbs[i];
            B[start] = 1.0 + S[chefs[start]] * expected;
        }

        /* Solve for W[subset][start][i] for each i */
        for (int i = 0; i < n; i++) {
            double A[MAXN][MAXN];
            double rhs[MAXN];
            memset(A, 0, sizeof(A));
            for (int start = 0; start < n; start++) {
                A[start][start] = 1.0;
                A[start][(start + 1) % n] = S[chefs[start]] - 1.0;
                rhs[start] = Bs[i][start];
            }
            linear_solve(n, A, rhs);
            for (int start = 0; start < n; start++)
                W[subset][chefs[start]][chefs[i]] = rhs[start];
        }

        /* Solve for E[subset][start] */
        {
            double A[MAXN][MAXN];
            double rhs[MAXN];
            memset(A, 0, sizeof(A));
            for (int start = 0; start < n; start++) {
                A[start][start] = 1.0;
                A[start][(start + 1) % n] = S[chefs[start]] - 1.0;
                rhs[start] = B[start];
            }
            linear_solve(n, A, rhs);
            for (int start = 0; start < n; start++)
                E[subset][chefs[start]] = rhs[start];
        }
    }

    double ans = E[(1 << N) - 1][0];
    printf("%.8f\n", ans);
    return 0;
}

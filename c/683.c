/*
 * Project Euler 683 - The Chase II
 *
 * Band matrix linear system for computing expected money per round,
 * then sum over n=2..500 players.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

#define MAX_L 260
#define MAX_DIFF 3
#define BAND_W (2 * MAX_DIFF + 1)

static double band[MAX_L][BAND_W]; /* band[row][MAX_DIFF + diff] = A[row][row+diff] */
static double B_vec[MAX_L];
static double X_arr[3][MAX_L]; /* X[e][d] for e=0,1,2 */
static double res[MAX_L];

static double nCr(int n, int r) {
    if (r < 0 || r > n) return 0.0;
    if (r == 0 || r == n) return 1.0;
    double result = 1.0;
    if (r > n - r) r = n - r;
    for (int i = 0; i < r; i++)
        result = result * (n - i) / (i + 1);
    return result;
}

static void solve_band(int l) {
    /* Forward elimination */
    for (int i = 0; i < l; i++) {
        for (int jj = 1; jj <= MAX_DIFF; jj++) {
            if (i + jj >= l) break;
            double pivot = band[i][MAX_DIFF];
            if (fabs(pivot) < 1e-15) continue;
            double ratio = band[i + jj][MAX_DIFF - jj] / pivot;
            for (int k = 0; k <= MAX_DIFF; k++) {
                int col_offset = k - jj;
                if (MAX_DIFF + col_offset >= 0 && MAX_DIFF + col_offset < BAND_W)
                    band[i + jj][MAX_DIFF + col_offset] -= ratio * band[i][MAX_DIFF + k];
            }
            B_vec[i + jj] -= ratio * B_vec[i];
        }
    }

    /* Back substitution */
    for (int i = l - 1; i >= 0; i--) {
        res[i] = B_vec[i];
        for (int jj = 1; jj <= MAX_DIFF; jj++) {
            if (i + jj < l)
                res[i] -= band[i][MAX_DIFF + jj] * res[i + jj];
        }
        double pivot = band[i][MAX_DIFF];
        if (fabs(pivot) > 1e-15)
            res[i] /= pivot;
    }
}

static double expected_round_money(int n, int E) {
    int l = n / 2 + 1;

    /* Initialize X[0][d] = 1 for all d */
    for (int d = 0; d < l; d++)
        X_arr[0][d] = 1.0;

    for (int e = 1; e <= E; e++) {
        memset(band, 0, sizeof(band));
        memset(B_vec, 0, sizeof(B_vec));

        for (int d = 0; d < l; d++)
            band[d][MAX_DIFF] = 1.0;

        for (int d = 1; d < l; d++) {
            for (int da = -1; da <= 1; da++) {
                for (int db = -1; db <= 1; db++) {
                    int nd = ((d + da + db) % n + n) % n;
                    if (nd > n - nd) nd = n - nd;
                    int diff = nd - d;
                    if (abs(diff) <= MAX_DIFF) {
                        band[d][MAX_DIFF + diff] -= 1.0 / 9.0;
                    }
                    for (int ep = 0; ep < e; ep++) {
                        B_vec[d] += nCr(e, ep) * X_arr[ep][nd] / 9.0;
                    }
                }
            }
        }

        solve_band(l);
        for (int d = 0; d < l; d++)
            X_arr[e][d] = res[d];
    }

    double expected = 0.0;
    for (int d = 0; d < n; d++) {
        int md = d;
        if (md > n - md) md = n - md;
        expected += X_arr[E][md];
    }
    return expected / n;
}

int main(void) {
    int N = 500;
    int E = 2;

    double ans = 0.0;
    for (int n = 2; n <= N; n++)
        ans += expected_round_money(n, E);

    /* Format: scientific notation with 8 decimal digits, no '+' in exponent */
    char buf[64];
    sprintf(buf, "%.8e", ans);
    /* Remove '+' characters */
    char out[64];
    int j = 0;
    for (int i = 0; buf[i]; i++)
        if (buf[i] != '+') out[j++] = buf[i];
    out[j] = '\0';
    printf("%s\n", out);
    return 0;
}

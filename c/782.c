/*
 * Project Euler 782 - Distinct Rows and Columns
 *
 * C(n) = 3*n^2 - 1 - N2 + N4
 * where N2 = count of k with c(n,k)=2 (block matrices)
 *       N4 = count of k with c(n,k)>=4 (not achievable with comp<=3)
 *
 * Uses bitarray sieve for achievability.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main(void) {
    int n = 10000;
    long long N = (long long)n * n;

    /* Sieve: achievable[k] = 1 if c(n,k) <= 3 */
    unsigned char *achievable = (unsigned char *)calloc(N + 1, 1);
    achievable[0] = 1;
    achievable[N] = 1;

    /* S2: comp=2 values from 2x2 block matrices */
    /* Use a temporary bitset for S2 */
    unsigned char *is_s2 = (unsigned char *)calloc(N + 1, 1);

    for (int c = 1; c < n; c++) {
        long long v = (long long)c * c;
        if (v > 0 && v < N) is_s2[v] = 1;
        long long w = N - v;
        if (w > 0 && w < N) is_s2[w] = 1;
    }
    for (int x = 1; x < n; x++) {
        int y = n - x;
        long long v1 = (long long)x * x + (long long)y * y;
        long long v2 = 2LL * x * y;
        if (v1 > 0 && v1 < N) is_s2[v1] = 1;
        if (v2 > 0 && v2 < N) is_s2[v2] = 1;
    }

    int N2 = 0;
    for (long long k = 1; k < N; k++) {
        if (is_s2[k]) {
            N2++;
            achievable[k] = 1;
        }
    }
    free(is_s2);

    /* Construction 1: Products d*m with 1 <= d,m <= n-1 */
    for (int d = 1; d < n; d++) {
        for (long long k = d; k < (long long)d * n; k += d) {
            achievable[k] = 1;
        }
    }

    /* Construction 2: Complement symmetry */
    for (long long k = 1; k < N; k++) {
        if (achievable[k] || achievable[N - k]) {
            achievable[k] = 1;
            achievable[N - k] = 1;
        }
    }

    /* Construction 3: Kernel 3x3 matrices
     * Enumerate all 3x3 binary matrices where every column is also a row
     */
    int rows_3[8][3];
    for (int i = 0; i < 8; i++) {
        rows_3[i][0] = (i >> 2) & 1;
        rows_3[i][1] = (i >> 1) & 1;
        rows_3[i][2] = i & 1;
    }

    /* Store unique kernel forms (aa, bb, ab, a1, b1, c0) */
    typedef struct { int aa, bb, ab, a1, b1; long long c0; } KForm;
    KForm forms[600];
    int nforms = 0;

    for (int r0i = 0; r0i < 8; r0i++) {
        for (int r1i = 0; r1i < 8; r1i++) {
            for (int r2i = 0; r2i < 8; r2i++) {
                int M[3][3];
                for (int j = 0; j < 3; j++) {
                    M[0][j] = rows_3[r0i][j];
                    M[1][j] = rows_3[r1i][j];
                    M[2][j] = rows_3[r2i][j];
                }

                /* Check that every column is also a row */
                int ok = 1;
                for (int j = 0; j < 3 && ok; j++) {
                    int col[3] = { M[0][j], M[1][j], M[2][j] };
                    int found = 0;
                    for (int ri = 0; ri < 3 && !found; ri++) {
                        if (M[ri][0] == col[0] && M[ri][1] == col[1] && M[ri][2] == col[2])
                            found = 1;
                    }
                    if (!found) ok = 0;
                }
                if (!ok) continue;

                int A = M[0][0], B = M[1][1], C = M[2][2];
                int D01 = M[0][1] + M[1][0];
                int D02 = M[0][2] + M[2][0];
                int D12 = M[1][2] + M[2][1];
                int aa = A + C - D02;
                int bb = B + C - D12;
                int ab = D01 + 2 * C - D02 - D12;
                int a1 = n * (D02 - 2 * C);
                int b1 = n * (D12 - 2 * C);
                long long c0 = (long long)C * n * n;

                /* Check if this form is already recorded */
                int dup = 0;
                for (int fi = 0; fi < nforms && !dup; fi++) {
                    if (forms[fi].aa == aa && forms[fi].bb == bb &&
                        forms[fi].ab == ab && forms[fi].a1 == a1 &&
                        forms[fi].b1 == b1 && forms[fi].c0 == c0)
                        dup = 1;
                }
                if (!dup && nforms < 600) {
                    forms[nforms].aa = aa;
                    forms[nforms].bb = bb;
                    forms[nforms].ab = ab;
                    forms[nforms].a1 = a1;
                    forms[nforms].b1 = b1;
                    forms[nforms].c0 = c0;
                    nforms++;
                }
            }
        }
    }

    for (int fi = 0; fi < nforms; fi++) {
        int aa = forms[fi].aa, bb = forms[fi].bb, ab = forms[fi].ab;
        int a1 = forms[fi].a1, b1 = forms[fi].b1;
        long long c0 = forms[fi].c0;
        for (int a = 0; a <= n; a++) {
            int b_max = n - a;
            for (int b = 0; b <= b_max; b++) {
                long long k = (long long)aa * a * a + (long long)bb * b * b +
                              (long long)ab * a * b + (long long)a1 * a +
                              (long long)b1 * b + c0;
                if (k > 0 && k < N)
                    achievable[k] = 1;
            }
        }
    }

    int N4 = 0;
    for (long long k = 1; k < N; k++) {
        if (achievable[k] == 0) N4++;
    }

    printf("%lld\n", 3 * N - 1 - N2 + N4);

    free(achievable);
    return 0;
}

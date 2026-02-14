/*
 * Project Euler 150 - Searching a triangular array for sub-triangles with minimum sum
 *
 * Uses prefix sums along right-slanting and left-slanting directions
 * (matching the Java reference approach).
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define NN 1000

/* rightSums[j][i] and leftSums[i][j] */
static long long rightSums[NN][NN];
static long long leftSums[NN][NN];

int main(void) {
    int total = NN * (NN + 1) / 2;

    /* Generate S[k] for k = 1..total */
    long long *S = calloc(total + 1, sizeof(long long));
    long long t = 0;
    for (int k = 1; k <= total; k++) {
        t = (615949 * t + 797807) % (1 << 20);
        S[k] = t - (1 << 19);
    }

    memset(rightSums, 0, sizeof(rightSums));
    memset(leftSums, 0, sizeof(leftSums));

    for (int i = 0; i < NN; i++) {
        for (int j = 0; j < NN - i; j++) {
            int row = i + j;
            int idx = row * (row + 1) / 2 + i + 1;

            if (i > 0)
                rightSums[j][i] = rightSums[j + 1][i - 1] + S[idx];
            else
                rightSums[j][i] = S[idx];

            if (i > 0)
                leftSums[i][j] = leftSums[i - 1][j + 1] + S[row * (row + 1) / 2 + i];
            else
                leftSums[i][j] = 0;
        }
    }

    free(S);

    long long ans = (long long)1 << 62;

    for (int i = 0; i < NN; i++) {
        for (int j = 0; j < NN - i; j++) {
            long long totalSum = 0;
            for (int k = 0; k < NN - i - j; k++) {
                totalSum += rightSums[i][j + k] - leftSums[j][i + k];
                if (totalSum < ans) ans = totalSum;
            }
        }
    }

    printf("%lld\n", ans);
    return 0;
}

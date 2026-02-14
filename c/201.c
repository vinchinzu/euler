/*
 * Project Euler 201: Subsets with a Unique Sum
 *
 * Find the sum of all numbers that are the sum of exactly one 50-element
 * subset of {1^2, 2^2, ..., 100^2}.
 *
 * DP with cap at L/2 using symmetry. Values capped at min(2, count).
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main(void) {
    int N = 100, K = 50;
    long long L = 0;
    for (int i = 1; i <= N; i++) L += (long long)i * i;  /* 338350 */

    int half = (int)(L / 2);  /* 169175 */

    /* dp[i][j] = min(2, number of i-element subsets summing to j) */
    /* Use unsigned char since values are only 0, 1, or 2 */
    /* Allocate as flat array: (K+1) * (half+1) */
    unsigned char *dp = calloc((size_t)(K + 1) * (half + 1), 1);
    if (!dp) { fprintf(stderr, "malloc failed\n"); return 1; }

    #define DP(i, j) dp[(size_t)(i) * (half + 1) + (j)]

    DP(0, 0) = 1;

    for (int n = 1; n <= N; n++) {
        int sq = n * n;
        long long max_j_ll = (long long)n * (n + 1) * (2 * n + 1) / 6;
        int max_j = (int)(max_j_ll < half ? max_j_ll : half);
        int max_i = K < n ? K : n;

        for (int i = max_i; i >= 1; i--) {
            int end = max_j + 1;
            if (end <= sq) continue;
            for (int j = end - 1; j >= sq; j--) {
                int val = DP(i, j) + DP(i - 1, j - sq);
                DP(i, j) = (unsigned char)(val < 2 ? val : 2);
            }
        }
    }

    long long ans = 0;
    for (int j = 0; j <= half; j++) {
        if (DP(K, j) == 1) {
            ans += L;
        }
    }

    printf("%lld\n", ans);
    free(dp);
    return 0;
}

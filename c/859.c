#include <stdio.h>
#include <string.h>

/*
 * Project Euler 859 - Cookies game
 *
 * Compute game values G[k], then DP over partitions.
 * C(300) = number of partitions of 300 where sum of G values <= 0.
 */

#define N 300
#define OFFSET 2000
#define MAX_VAL 4001

typedef long long ll;

ll dp[N + 1][MAX_VAL];

int main(void) {
    /* Compute G values */
    int G[N + 1];
    G[0] = 0;

    for (int k = 1; k <= N; k++) {
        if (k % 2 == 1) { /* Odd */
            int m = (k - 1) / 2;
            int val = 2 * G[m];
            G[k] = (val < 0) ? 0 : val + 1;
        } else { /* Even */
            int m = (k - 2) / 2;
            int val = 2 * G[m];
            G[k] = (val > 0) ? 0 : val - 1;
        }
    }

    /* DP for partitions */
    memset(dp, 0, sizeof(dp));
    dp[0][OFFSET] = 1;

    for (int k = 1; k <= N; k++) {
        int g_val = G[k];
        for (int n = k; n <= N; n++) {
            /* Add partitions using part k */
            if (g_val >= 0) {
                for (int i = 0; i < MAX_VAL - g_val; i++) {
                    if (dp[n - k][i] > 0) {
                        dp[n][i + g_val] += dp[n - k][i];
                    }
                }
            } else {
                for (int i = -g_val; i < MAX_VAL; i++) {
                    if (dp[n - k][i] > 0) {
                        dp[n][i + g_val] += dp[n - k][i];
                    }
                }
            }
        }
    }

    /* Sum counts for value <= 0 (index <= OFFSET) */
    ll total = 0;
    for (int i = 0; i <= OFFSET; i++) {
        total += dp[N][i];
    }

    printf("%lld\n", total);
    return 0;
}

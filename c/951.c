/*
 * Project Euler Problem 951
 * Card game with 2n cards (n red, n black).
 * F(n) = C(2n,n) - (configs with no run of length 2).
 * Find F(26).
 */
#include <stdio.h>
#include <string.h>

/*
 * DP state: (r, b, length, color) where r,b in [0..26], length in [1..52], color in {0,1}
 * Use a hash map approach or direct array.
 * r,b each 0..26 (27 values), length 1..52 (52 values), color 0..1
 * Total: 27 * 27 * 52 * 2 = 75816 states - small enough for arrays
 */

#define N 26
#define MAXRB (N+1)
#define MAXLEN (2*N+1)

/* dp[r][b][len][color] - but len is 1-indexed, store as len-1 for 0-indexed */
static long long dp[MAXRB][MAXRB][MAXLEN][2];
static long long new_dp[MAXRB][MAXRB][MAXLEN][2];

int main(void) {
    int n = N;

    memset(dp, 0, sizeof(dp));
    /* Initial: "R" (color 0=Red) or "B" (color 1=Black) */
    dp[1][0][1][0] = 1;  /* one red card */
    dp[0][1][1][1] = 1;  /* one black card */

    for (int k = 1; k < 2 * n; k++) {
        memset(new_dp, 0, sizeof(new_dp));
        for (int r = 0; r <= n; r++) {
            for (int b = 0; b <= n; b++) {
                if (r + b != k) continue;
                for (int len = 1; len <= k; len++) {
                    for (int color = 0; color <= 1; color++) {
                        long long count = dp[r][b][len][color];
                        if (count == 0) continue;

                        /* Try adding Red (0) */
                        if (r + 1 <= n) {
                            if (color == 0) {
                                /* Extend red run */
                                new_dp[r+1][b][len+1][0] += count;
                            } else {
                                /* Switch from black to red, check finished black run != 2 */
                                if (len != 2) {
                                    new_dp[r+1][b][1][0] += count;
                                }
                            }
                        }

                        /* Try adding Black (1) */
                        if (b + 1 <= n) {
                            if (color == 1) {
                                /* Extend black run */
                                new_dp[r][b+1][len+1][1] += count;
                            } else {
                                /* Switch from red to black, check finished red run != 2 */
                                if (len != 2) {
                                    new_dp[r][b+1][1][1] += count;
                                }
                            }
                        }
                    }
                }
            }
        }
        memcpy(dp, new_dp, sizeof(dp));
    }

    /* Sum valid final states: r=n, b=n, final run length != 2 */
    long long bad_count = 0;
    for (int len = 1; len <= 2 * n; len++) {
        if (len == 2) continue;
        for (int color = 0; color <= 1; color++) {
            bad_count += dp[n][n][len][color];
        }
    }

    /* Compute C(2n, n) */
    /* Use __int128 to avoid overflow for C(52, 26) */
    __int128 total = 1;
    for (int i = 1; i <= n; i++) {
        total = total * (n + i) / i;
    }

    long long fair = (long long)total - bad_count;
    printf("%lld\n", fair);
    return 0;
}

/* Project Euler Problem 750: Optimal Card Stacking.
 * Extracted from embedded C in python/750.py
 *
 * Interval DP: dp[s][e] = min cost to merge cards s..e into one stack.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define N 976

static long long dp[N][N+1];
static int pos[N];

int main(void) {
    long long p = 1;
    int i, s, length, mid, end;
    for (i = 0; i < N; i++) {
        p = (p * 3) % (N + 1);
        pos[(int)p - 1] = i;
    }

    memset(dp, 0, sizeof(dp));

    for (length = 2; length <= N; length++) {
        for (s = 0; s <= N - length; s++) {
            end = s + length;
            long long best = -1;
            for (mid = s + 1; mid < end; mid++) {
                long long cost = dp[s][mid] + dp[mid][end]
                    + abs(pos[mid - 1] - pos[end - 1]);
                if (best < 0 || cost < best)
                    best = cost;
            }
            dp[s][end] = best;
        }
    }

    printf("%lld\n", dp[0][N]);
    return 0;
}

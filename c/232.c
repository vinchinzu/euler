/*
 * Project Euler Problem 232: The Race
 *
 * Two players race to 100. Player 1 advances 1 on heads.
 * Player 2 chooses T>=1, flips T coins; advances 2^(T-1) if all heads (prob 1/2^T).
 * Find probability player 2 wins with optimal play.
 */
#include <stdio.h>

#define N 100

static double dp[N + 1][2 * N];

int main(void) {
    /* Base cases: player 2 wins if score2 >= N */
    for (int s1 = 0; s1 <= N; s1++)
        for (int s2 = N; s2 < 2 * N; s2++)
            dp[s1][s2] = 1.0;

    /* Fill DP backwards */
    for (int s1 = N - 1; s1 >= 0; s1--) {
        for (int s2 = N - 1; s2 >= 0; s2--) {
            double best = 0.0;
            for (int T = 1; T < 100; T++) {
                long long points = 1LL << (T - 1);
                if (s2 + points >= 2 * N) break;

                long long pow2T = 1LL << T;
                double prob = (dp[s1 + 1][s2 + (int)points]
                             + dp[s1][(int)(s2 + points)]
                             + (double)(pow2T - 1) * dp[s1 + 1][s2])
                             / (double)(pow2T + 1);

                if (prob > best) best = prob;
            }
            dp[s1][s2] = best;
        }
    }

    double ans = (dp[0][0] + dp[1][0]) / 2.0;
    printf("%.8f\n", ans);
    return 0;
}

/*
 * Project Euler 205: Dice Game
 *
 * Peter rolls 9 four-sided dice, Colin rolls 6 six-sided dice.
 * Find the probability that Peter beats Colin.
 */
#include <stdio.h>
#include <string.h>

int main(void) {
    /* Peter: 9d4, sums from 9 to 36 */
    /* Colin: 6d6, sums from 6 to 36 */
    int peter_max = 36, colin_max = 36;

    /* Count distributions via DP */
    long long peter[37], colin[37];
    memset(peter, 0, sizeof(peter));
    memset(colin, 0, sizeof(colin));

    /* Peter: 9d4 */
    {
        long long dp[37];
        memset(dp, 0, sizeof(dp));
        dp[0] = 1;
        for (int die = 0; die < 9; die++) {
            long long new_dp[37];
            memset(new_dp, 0, sizeof(new_dp));
            for (int s = 0; s <= peter_max; s++) {
                if (dp[s] == 0) continue;
                for (int face = 1; face <= 4; face++) {
                    if (s + face <= peter_max)
                        new_dp[s + face] += dp[s];
                }
            }
            for (int s = 0; s <= peter_max; s++) dp[s] = new_dp[s];
        }
        for (int s = 0; s <= peter_max; s++) peter[s] = dp[s];
    }

    /* Colin: 6d6 */
    {
        long long dp[37];
        memset(dp, 0, sizeof(dp));
        dp[0] = 1;
        for (int die = 0; die < 6; die++) {
            long long new_dp[37];
            memset(new_dp, 0, sizeof(new_dp));
            for (int s = 0; s <= colin_max; s++) {
                if (dp[s] == 0) continue;
                for (int face = 1; face <= 6; face++) {
                    if (s + face <= colin_max)
                        new_dp[s + face] += dp[s];
                }
            }
            for (int s = 0; s <= colin_max; s++) dp[s] = new_dp[s];
        }
        for (int s = 0; s <= colin_max; s++) colin[s] = dp[s];
    }

    /* Total outcomes: 4^9 * 6^6 */
    double peter_total = 1.0;
    for (int i = 0; i < 9; i++) peter_total *= 4.0;
    double colin_total = 1.0;
    for (int i = 0; i < 6; i++) colin_total *= 6.0;
    double total = peter_total * colin_total;

    /* Count wins */
    long long wins = 0;
    for (int p = 1; p <= peter_max; p++) {
        for (int c = 1; c < p; c++) {
            wins += peter[p] * colin[c];
        }
    }

    printf("%.7f\n", (double)wins / total);
    return 0;
}

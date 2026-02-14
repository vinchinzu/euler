/* Project Euler Problem 117: Red, Green, and Blue Tiles */
#include <stdio.h>

int main(void) {
    long long dp[51];
    dp[0] = 1;
    for (int i = 1; i <= 50; i++) {
        dp[i] = dp[i - 1]; /* grey */
        if (i >= 2) dp[i] += dp[i - 2]; /* red */
        if (i >= 3) dp[i] += dp[i - 3]; /* green */
        if (i >= 4) dp[i] += dp[i - 4]; /* blue */
    }
    printf("%lld\n", dp[50]);
    return 0;
}

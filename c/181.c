/* Project Euler 181: Investigating in how many ways objects of two different colours can be grouped. */
#include <stdio.h>
#include <string.h>

#define BLACK 60
#define WHITE 40

int main(void) {
    static long long dp[BLACK + 1][WHITE + 1];
    memset(dp, 0, sizeof(dp));
    dp[0][0] = 1;

    for (int b = 0; b <= BLACK; b++) {
        for (int w = 0; w <= WHITE; w++) {
            if (b == 0 && w == 0) continue;
            for (int i = b; i <= BLACK; i++) {
                for (int j = w; j <= WHITE; j++) {
                    dp[i][j] += dp[i - b][j - w];
                }
            }
        }
    }

    printf("%lld\n", dp[BLACK][WHITE]);
    return 0;
}

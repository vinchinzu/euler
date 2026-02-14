/* Project Euler Problem 116: Red, Green or Blue Tiles */
#include <stdio.h>

static long long ways_for_colour(int row_length, int tile_length) {
    long long dp[51];
    dp[0] = 1;
    for (int i = 1; i <= row_length; i++) {
        dp[i] = dp[i - 1];
        if (i >= tile_length)
            dp[i] += dp[i - tile_length];
    }
    return dp[row_length] - 1; /* subtract all-grey case */
}

int main(void) {
    int row = 50;
    long long total = ways_for_colour(row, 2) + ways_for_colour(row, 3) + ways_for_colour(row, 4);
    printf("%lld\n", total);
    return 0;
}

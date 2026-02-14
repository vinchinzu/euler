/* Project Euler Problem 115: Counting Block Combinations II */
#include <stdio.h>

#define MAXN 500

static long long dp[MAXN];

static long long count_arrangements(int min_len, int length) {
    /* dp[i] = number of ways to tile row of length i */
    dp[0] = 1;
    for (int i = 1; i <= length; i++) {
        long long total = dp[i - 1]; /* grey square at end */
        for (int bl = min_len; bl <= i; bl++) {
            /* Place red block of length bl ending at position i */
            int remaining = i - bl;
            if (remaining == 0)
                total += 1;
            else if (remaining >= 1)
                total += dp[remaining - 1]; /* must have grey separator */
        }
        dp[i] = total;
    }
    return dp[length];
}

int main(void) {
    int min_len = 50;
    long long target = 1000000;

    for (int n = min_len; n < MAXN; n++) {
        long long val = count_arrangements(min_len, n);
        if (val > target) {
            printf("%d\n", n);
            return 0;
        }
    }
    return 1;
}

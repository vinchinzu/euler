/*
 * Project Euler 219: Skew-cost Coding
 *
 * Minimum cost prefix-free code for 10^9 characters with
 * cost(0) = 1, cost(1) = 4.
 */
#include <stdio.h>

int main(void) {
    long long N = 1000000000LL;
    int C0 = 1, C1 = 4;

    /* dp[i] = number of codes at depth i available for splitting */
    /* We maintain a dynamic array of dp values */
    long long dp[200];
    int dp_len = 1;
    dp[0] = 1;

    long long n = 1;
    long long ans = 0;

    while (n < N) {
        long long num_codes = dp[dp_len - 1];
        if (num_codes > N - n) num_codes = N - n;
        n += num_codes;
        ans += num_codes * ((long long)(dp_len - 1) + C0 + C1);

        long long next = 0;
        if (dp_len >= C0) next += dp[dp_len - C0];
        if (dp_len >= C1) next += dp[dp_len - C1];
        dp[dp_len] = next;
        dp_len++;
    }

    printf("%lld\n", ans);
    return 0;
}

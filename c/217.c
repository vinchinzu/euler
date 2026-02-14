/*
 * Project Euler 217: Balanced Numbers
 *
 * A k-digit number is balanced if the sum of its first ceil(k/2) digits
 * equals the sum of its last ceil(k/2) digits.
 * Find the sum of all balanced numbers < 10^47, mod 3^15.
 */
#include <stdio.h>
#include <string.h>

#define MAX_N 47
#define B 10
#define MAX_SUM (B * (MAX_N / 2 + 1))

static long long M;

static long long dp[MAX_N / 2 + 2][MAX_SUM + 1];

static long long pow_mod(long long base, long long exp, long long mod) {
    long long result = 1;
    base %= mod;
    while (exp > 0) {
        if (exp & 1) result = result * base % mod;
        base = base * base % mod;
        exp >>= 1;
    }
    return result;
}

int main(void) {
    int N_digits = 47;
    /* M = 3^15 */
    M = 1;
    for (int i = 0; i < 15; i++) M *= 3;

    /* dp[i][j] = number of ways for i digits to sum to j (mod M) */
    memset(dp, 0, sizeof(dp));
    dp[0][0] = 1;

    for (int i = 1; i <= N_digits / 2; i++) {
        for (int j = 0; j < MAX_SUM; j++) {
            long long s = 0;
            for (int d = 0; d < B && d <= j; d++) {
                s += dp[i-1][j-d];
            }
            dp[i][j] = s % M;
        }
    }

    long long ans = 0;

    for (int k = 1; k <= N_digits; k++) {
        int half = k / 2;
        for (int i = 0; i < k; i++) {
            for (int d = 1; d < B; d++) {
                long long mult = pow_mod(B, i, M) * d % M;
                if (k % 2 == 1 && i != k / 2) {
                    mult = mult * B % M;
                }

                for (int S = 0; S < MAX_SUM; S++) {
                    long long term;
                    if (i < half) {
                        long long a = (dp[half][S] - dp[half - 1][S] % M + M) % M;
                        long long b = (S >= d) ? dp[half - 1][S - d] : 0;
                        term = a % M * (b % M) % M;
                    } else if (k % 2 == 1 && i == half) {
                        long long a = (dp[half][S] - (k >= 2 ? dp[half - 1][S] : 0) % M + M) % M;
                        long long b = dp[half][S] % M;
                        term = a * b % M;
                    } else if (i < k - 1) {
                        long long a1 = (S >= d) ? dp[half - 1][S - d] : 0;
                        long long a2 = (k >= 4 && S >= d) ? dp[half - 2][S - d] : 0;
                        long long a = (a1 - a2 % M + M) % M;
                        long long b = dp[half][S] % M;
                        term = a * b % M;
                    } else { /* i == k - 1 */
                        long long a = (S >= d) ? dp[half - 1][S - d] : 0;
                        long long b = dp[half][S] % M;
                        term = (a % M) * b % M;
                    }

                    ans = (ans + mult % M * (term % M)) % M;
                }
            }
        }
    }

    printf("%lld\n", ans);
    return 0;
}

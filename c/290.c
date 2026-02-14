/*
 * Project Euler Problem 290: Digital Signature
 *
 * Find the number of positive integers less than 10^N such that the sum
 * of the digits of n equals the sum of the digits of 137n.
 *
 * DP over digits with state (diff, carry).
 */
#include <stdio.h>
#include <string.h>

#define N 18
#define K 137
#define B 10
#define MAX_DIFF (B * N + 1)
#define DIFF_OFFSET (B * N)
#define DIFF_SIZE (2 * B * N + 1)

static long long counts[2][DIFF_SIZE][K];

static int sum_digits(int n) {
    int s = 0;
    while (n > 0) {
        s += n % 10;
        n /= 10;
    }
    return s;
}

int main(void) {
    memset(counts, 0, sizeof(counts));
    /* Initial state: 0 digits, diff=0, carry=0 */
    counts[0][DIFF_OFFSET][0] = 1;

    int cur = 0;
    for (int i = 0; i < N; i++) {
        int nxt = 1 - cur;
        memset(counts[nxt], 0, sizeof(counts[nxt]));

        for (int j = 0; j < DIFF_SIZE; j++) {
            for (int k = 0; k < K; k++) {
                long long count = counts[cur][j][k];
                if (count == 0) continue;
                for (int d = 0; d < B; d++) {
                    int t = d * K + k;
                    int new_j = (j - DIFF_OFFSET) - sum_digits(k) + sum_digits(t) - d + DIFF_OFFSET;
                    int new_k = t / B;
                    if (new_j >= 0 && new_j < DIFF_SIZE) {
                        counts[nxt][new_j][new_k] += count;
                    }
                }
            }
        }
        cur = nxt;
    }

    long long ans = 0;
    for (int carry = 0; carry < K; carry++) {
        ans += counts[cur][DIFF_OFFSET][carry];
    }
    printf("%lld\n", ans);
    return 0;
}

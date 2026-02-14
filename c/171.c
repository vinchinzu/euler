/*
 * Project Euler Problem 171: Finding numbers for which the sum of the
 * squares of the digits is a perfect square.
 *
 * DP over 20 digits, tracking digit-square-sum and digit-sum contribution.
 */
#include <stdio.h>
#include <stdlib.h>
#include <math.h>

#define DIGITS 20
#define MAX_SUM (DIGITS * 81)
#define MOD 1000000000LL

int main(void) {
    long long pow10[DIGITS + 1];
    pow10[0] = 1;
    for (int i = 1; i <= DIGITS; i++)
        pow10[i] = (pow10[i - 1] * 10) % MOD;

    int is_square[MAX_SUM + 1];
    for (int s = 0; s <= MAX_SUM; s++) {
        int root = (int)sqrt((double)s);
        while (root * root < s) root++;
        while (root * root > s) root--;
        is_square[s] = (root * root == s);
    }

    /* count[length][s] and sum_dp[length][s] */
    /* Use two rows to save memory */
    long long *count_prev = calloc(MAX_SUM + 1, sizeof(long long));
    long long *count_cur = calloc(MAX_SUM + 1, sizeof(long long));
    long long *sum_prev = calloc(MAX_SUM + 1, sizeof(long long));
    long long *sum_cur = calloc(MAX_SUM + 1, sizeof(long long));

    count_prev[0] = 1;

    for (int length = 0; length < DIGITS; length++) {
        for (int s = 0; s <= MAX_SUM; s++) {
            count_cur[s] = 0;
            sum_cur[s] = 0;
        }

        for (int s = 0; s <= MAX_SUM; s++) {
            long long cnt = count_prev[s];
            if (cnt == 0) continue;
            long long cur_sum = sum_prev[s];
            long long factor = pow10[length];

            for (int d = 0; d <= 9; d++) {
                int ns = s + d * d;
                if (ns > MAX_SUM) break;

                count_cur[ns] = (count_cur[ns] + cnt) % MOD;
                long long added = (cnt % MOD * d % MOD * factor) % MOD;
                long long total = (cur_sum + added) % MOD;
                sum_cur[ns] = (sum_cur[ns] + total) % MOD;
            }
        }

        long long *tmp;
        tmp = count_prev; count_prev = count_cur; count_cur = tmp;
        tmp = sum_prev; sum_prev = sum_cur; sum_cur = tmp;
    }

    long long result = 0;
    for (int s = 0; s <= MAX_SUM; s++) {
        if (is_square[s])
            result = (result + sum_prev[s]) % MOD;
    }

    printf("%lld\n", result);

    free(count_prev); free(count_cur);
    free(sum_prev); free(sum_cur);
    return 0;
}

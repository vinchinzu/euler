/*
 * Project Euler Problem 164: Numbers for which no three consecutive digits
 * have a sum greater than 9.
 *
 * DP: track last two digits.
 */
#include <stdio.h>

int main(void) {
    int n = 20;
    long long prev[10][10] = {{0}};
    long long cur[10][10];

    /* Base case: two-digit numbers */
    for (int d1 = 1; d1 <= 9; d1++)
        for (int d2 = 0; d2 <= 9; d2++)
            prev[d1][d2] = 1;

    for (int pos = 3; pos <= n; pos++) {
        for (int i = 0; i < 10; i++)
            for (int j = 0; j < 10; j++)
                cur[i][j] = 0;

        for (int d1 = 0; d1 <= 9; d1++)
            for (int d2 = 0; d2 <= 9; d2++) {
                if (prev[d1][d2] == 0) continue;
                int max_d3 = 9 - d1 - d2;
                if (max_d3 < 0) continue;
                for (int d3 = 0; d3 <= max_d3; d3++)
                    cur[d2][d3] += prev[d1][d2];
            }

        for (int i = 0; i < 10; i++)
            for (int j = 0; j < 10; j++)
                prev[i][j] = cur[i][j];
    }

    long long total = 0;
    for (int i = 0; i < 10; i++)
        for (int j = 0; j < 10; j++)
            total += prev[i][j];

    printf("%lld\n", total);
    return 0;
}

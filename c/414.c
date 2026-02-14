#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAXB 1810
#define MOD 1000000000000000000LL  /* 10^18 */

static int cache[MAXB][MAXB];

/* Compute sb(b, d1, d2) using memoization */
int sb(int b, int d1, int d2) {
    /* Base case: Kaprekar constant has d1 = 2b/3, d2 = b/3 */
    if (d1 == 2 * b / 3 && d2 == b / 3)
        return 1;

    if (cache[d1][d2] == 0) {
        int digits[5];
        if (d2 == 0) {
            digits[0] = d1 - 1;
            digits[1] = b - 1;
            digits[2] = b - 1;
            digits[3] = b - 1;
            digits[4] = b - d1;
        } else {
            digits[0] = d1;
            digits[1] = d2 - 1;
            digits[2] = b - 1;
            digits[3] = b - d2 - 1;
            digits[4] = b - d1;
        }

        /* Sort digits */
        for (int i = 0; i < 4; i++)
            for (int j = i+1; j < 5; j++)
                if (digits[i] > digits[j]) {
                    int t = digits[i]; digits[i] = digits[j]; digits[j] = t;
                }

        int new_d1 = digits[4] - digits[0];
        int new_d2 = digits[3] - digits[1];
        cache[d1][d2] = 1 + sb(b, new_d1, new_d2);
    }

    return cache[d1][d2];
}

int main() {
    int N = 300;
    long long ans = 0;

    for (int k = 2; k <= N; k++) {
        int b = 6 * k + 3;
        memset(cache, 0, sizeof(cache));

        for (int d1 = 1; d1 < b; d1++) {
            for (int d2 = 0; d2 <= d1; d2++) {
                long long mult = (long long)(b - d1);

                if (d2 == 0)
                    mult *= 20 * d1 - 10;
                else if (d2 == d1)
                    mult *= 30 * d1 - 10;
                else
                    mult *= (long long)120 * d2 * (d1 - d2) - 20;

                ans = (ans + mult % MOD * sb(b, d1, d2)) % MOD;
            }
        }

        ans = (ans - 1 + MOD) % MOD;
    }

    printf("%lld\n", ans);
    return 0;
}

/*
 * Project Euler Problem 178: Step Numbers
 *
 * Count step numbers of length 10..40 that contain all digits 0-9.
 * A step number has consecutive digits differing by 1.
 * DP: dp[digit][mask] = count of step numbers ending in digit with
 * digit-set mask.
 */
#include <stdio.h>
#include <string.h>

#define MAX_LEN 40
#define FULL_MASK 1023  /* (1 << 10) - 1 */

int main(void) {
    long long cur[10][1024];
    long long nxt[10][1024];

    memset(cur, 0, sizeof(cur));

    /* Initialize: 1-digit step numbers (digits 1-9) */
    for (int d = 1; d <= 9; d++)
        cur[d][1 << d] = 1;

    long long total = 0;

    for (int step = 0; step < MAX_LEN - 1; step++) {
        memset(nxt, 0, sizeof(nxt));

        for (int digit = 0; digit <= 9; digit++) {
            for (int mask = 0; mask <= FULL_MASK; mask++) {
                long long count = cur[digit][mask];
                if (count == 0) continue;

                if (digit > 0) {
                    int nd = digit - 1;
                    int nm = mask | (1 << nd);
                    nxt[nd][nm] += count;
                }
                if (digit < 9) {
                    int nd = digit + 1;
                    int nm = mask | (1 << nd);
                    nxt[nd][nm] += count;
                }
            }
        }

        memcpy(cur, nxt, sizeof(cur));

        /* After step k, we have numbers of length k+2 */
        if (step + 2 >= 10) {
            for (int digit = 0; digit <= 9; digit++)
                total += cur[digit][FULL_MASK];
        }
    }

    printf("%lld\n", total);
    return 0;
}

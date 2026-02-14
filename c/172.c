/*
 * Project Euler Problem 172: Investigating numbers with few repeated digits.
 *
 * Count 18-digit numbers where each digit 0-9 appears at most 3 times.
 * State = tuple of counts per digit, encoded as base-4 number (4^10 states).
 *
 * The Python approach with 4^10 = 1M states x 18 positions works but
 * uses significant memory. Let's use the same approach.
 */
#include <stdio.h>
#include <string.h>
#include <stdlib.h>

#define N_DIGITS 18
#define BASE 4
#define NUM_DIGITS 10
#define NUM_STATES 1048576  /* 4^10 */

int main(void) {
    int powers[NUM_DIGITS];
    powers[0] = 1;
    for (int d = 1; d < NUM_DIGITS; d++)
        powers[d] = powers[d - 1] * BASE;

    /* dp[state] = number of ways */
    long long *dp = calloc(NUM_STATES, sizeof(long long));
    long long *ndp = calloc(NUM_STATES, sizeof(long long));

    /* Position 0: first digit (1-9, no leading zero) */
    dp[0] = 1;

    for (int pos = 0; pos < N_DIGITS; pos++) {
        memset(ndp, 0, NUM_STATES * sizeof(long long));

        for (int state = 0; state < NUM_STATES; state++) {
            if (dp[state] == 0) continue;

            for (int digit = 0; digit < 10; digit++) {
                /* Skip leading zero */
                if (pos == 0 && digit == 0) continue;

                /* Check count for this digit */
                int count = (state / powers[digit]) % BASE;
                if (count >= 3) continue;

                int new_state = state + powers[digit];
                ndp[new_state] += dp[state];
            }
        }

        long long *tmp = dp; dp = ndp; ndp = tmp;
    }

    long long total = 0;
    for (int state = 0; state < NUM_STATES; state++)
        total += dp[state];

    printf("%lld\n", total);

    free(dp);
    free(ndp);
    return 0;
}
